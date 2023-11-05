use std::time::Duration;

use floem::{
    animate::{EasingFn, FillMode},
    event::EventListener,
    peniko::Color,
    reactive::create_signal,
    style::{JustifyContent, Position},
    unit::UnitExt,
    view::View,
    views::{label, scroll, stack, Decorators},
};

fn app_view() -> impl View {
    let (counter, set_counter) = create_signal(0.0);
    let (is_hovered, set_is_hovered) = create_signal(false);
    let (scroll_offset_pct, set_scroll_offset_y) = create_signal(0.0);
    let base_label_height = 120.0;

    scroll({
        stack({
            (label(|| "Hover, click me, or scroll down")
                .on_click(move |_| {
                    set_counter.update(|value| *value += 1.0);
                    true
                })
                .on_event(EventListener::PointerEnter, move |_| {
                    set_is_hovered.update(|val| *val = true);
                    true
                })
                .on_event(EventListener::PointerLeave, move |_| {
                    set_is_hovered.update(|val| *val = false);
                    true
                })
                .style(move |s| {
                    s.border(1.0)
                        .background(Color::RED)
                        .color(Color::BLACK)
                        .padding(10.0)
                        .margin(20.0)
                        .size(250.0, base_label_height)
                }) 
                .animation(move |a| {
                    a.border_radius(move || if is_hovered.get() { 1.0 } else { 40.0 })
                        .border_color(|| Color::CYAN)
                        .background(move || {
                            Color::rgba8(
                                0,
                                if is_hovered.get() { 150 } else { 255 },
                                (255_f64 * scroll_offset_pct.get()).round() as u8,
                                (255_f64 * scroll_offset_pct.get()).round() as u8,
                            )
                        })
                        // .background(move || {
                        //     if is_hovered.get() {
                        //         Color::DEEP_PINK
                        //     } else {
                        //         Color::DARK_ORANGE
                        //     }
                        // })
                        // .height(move || base_label_height + (scroll_offset_pct.get() * 150.0))
                        .easing_fn(EasingFn::Quartic)
                        .ease_in_out()
                        .duration(Duration::from_secs(1))
                        .fill_mode(FillMode::Forwards)
                        // .auto_reverse(true)
                        // .repeating_forever(true)
                }),)
        })
        .style(|s| {
            s.border(5.0)
                .background(Color::LIGHT_CORAL)
                .padding(10.0)
                .flex_col()
                .flex_grow(1.0)
                .justify_content(Some(JustifyContent::FlexEnd))
                .padding_bottom(60.pct())
                .size(400.0, 800.0)
                .color(Color::BLACK)
        })
        // .animation(move |a| {
        //     a
        // .width(move || {
        //     if counter.get() % 2.0 == 0.0 {
        //         400.0
        //     } else {
        //         600.0
        //     }
        // })
        // .height(move || {
        //     if counter.get() % 2.0 == 0.0 {
        //         200.0
        //     } else {
        //         500.0
        //     }
        // })
        //         .background(|| Color::DARK_RED)
        //         .easing_fn(EasingFn::Cubic)
        //         .ease_in_out()
        //         .fill_mode(FillMode::Forwards)
        //         .duration(Duration::from_secs(10))
        // })
    })
    .on_scroll(move |rect| {
        let offset_y_pct = rect.y0 as f64 / rect.height() as f64;
        set_scroll_offset_y.update(|val| *val = offset_y_pct)
    })
    .style(|s| s.height(500.px()))
}

fn main() {
    floem::launch(app_view);
}
