use std::time::{Duration, Instant};

use floem::{
    action::exec_after,
    context,
    kurbo::{ Point, Rect, Size},
    peniko::Color,
    style::Style,
    taffy::{self, prelude::*},
    views::empty,
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
        let dims = 700.;
        let w = dims;
        let h = dims;
        let w2 = w / 2.;
        let h2 = h / 2.;
        let q = dims / 4.;

        let total_circles = 70;
        let spin_duration_sec = 1.;
        let target_num_spins = 3;

        let circles_to_draw = if let Some(last_paint_on) = self.last_paint_on {
            let elapsed_sec = last_paint_on.elapsed().as_secs_f64();
            let curr_spin_elapsed = elapsed_sec % spin_duration_sec;
            let num_spins = elapsed_sec / spin_duration_sec;

            if elapsed_sec > spin_duration_sec * target_num_spins as f64 {
                self.last_paint_on = Some(Instant::now());
            }

            total_circles as f64 * curr_spin_elapsed * num_spins
        } else {
            self.last_paint_on = Some(Instant::now());
            0.
        };

        let circle_size = Size::new(8., 8.);

        for t in 0..=circles_to_draw.round() as i32 {
            let t = t as f64 * 0.1;
            // or 
            // let t = t as f64;
            let x = w2 + q * t.cos();
            let y = h2 + q * t.sin();
            let circle = Rect::from_center_size(Point::new(x, y), circle_size).to_rounded_rect(40.);
            cx.fill(&circle, Color::WHITE, 0.0);
        }
        let update_interval = Duration::from_millis(16);

        let id = self.id();
        exec_after(
            update_interval,
            Box::new(move |_| {
                id.request_paint();
            }),
        );
    }
}
