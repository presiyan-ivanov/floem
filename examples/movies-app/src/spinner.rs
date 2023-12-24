use std::time::{Duration, Instant};

use floem::{
    action::exec_after,
    context,
    kurbo::{CircleSegment, Point},
    peniko::Color,
    style::Style,
    taffy::{self, prelude::*},
    Renderer,
};

use floem::{
    id::Id,
    view::{View, ViewData},
};

pub struct Spinner {
    data: ViewData,
    content_node: Option<Node>,
    last_paint_on: Option<Instant>,
}

pub fn spinner() -> Spinner {
    Spinner {
        data: ViewData::new(Id::next()),
        content_node: None,
        last_paint_on: None,
    }
}

impl View for Spinner {
    fn view_data(&self) -> &ViewData {
        &self.data
    }

    fn view_data_mut(&mut self) -> &mut ViewData {
        &mut self.data
    }

    fn debug_name(&self) -> std::borrow::Cow<'static, str> {
        "Spinner".into()
    }

    fn layout(&mut self, cx: &mut context::LayoutCx) -> taffy::prelude::Node {
        cx.layout_node(self.id(), true, |cx| {
            if self.content_node.is_none() {
                self.content_node = Some(
                    cx.app_state_mut()
                        .taffy
                        .new_leaf(taffy::style::Style::DEFAULT)
                        .unwrap(),
                );
            }
            let content_node = self.content_node.unwrap();

            let style = Style::new().width(50.).height(50.).to_taffy_style();
            let _ = cx.app_state_mut().taffy.set_style(content_node, style);

            vec![content_node]
        })
    }

    fn paint(&mut self, cx: &mut context::PaintCx) {
        let id = self.id();
        let spin_duration_sec = 1.;

        let progress = if let Some(last_paint_on) = self.last_paint_on {
            let elapsed_sec = last_paint_on.elapsed().as_secs_f64();
            let curr_spin_progress = elapsed_sec % spin_duration_sec;

            if elapsed_sec >= spin_duration_sec {
                self.last_paint_on = Some(Instant::now());
            }

            curr_spin_progress
        } else {
            self.last_paint_on = Some(Instant::now());
            0.
        };

        let sweep_deg: f64 = 260.0;
        let start_deg: f64 = progress * 360.;
        let inner_radius = 80.0;
        let outer_radius = 70.0;
        let segment = CircleSegment::new(
            Point::new(200.0, 200.0),
            outer_radius,
            inner_radius,
            start_deg.to_radians(),
            sweep_deg.to_radians(),
        );

        cx.fill(&segment, Color::WHITE, 0.0);

        exec_after(
            Duration::from_millis(16),
            Box::new(move |_| {
                id.request_paint();
            }),
        );
    }
}
