use std::time::Duration;

use floem::{
    animate::{animation, EasingFn},
    event::EventListner,
    peniko::Color,
    reactive::{create_signal, SignalGet, SignalUpdate},
    style::Style,
    view::View,
    views::{label, stack, Decorators},
    AppContext,
};

fn app_view() -> impl View {
    let cx = AppContext::get_current();
    let (counter, set_counter) = create_signal(cx.scope, 0.0);
    let (is_hovered, set_is_hovered) = create_signal(cx.scope, false);

    stack(|| {
        (
            label(|| "Hover or click me!".to_string())
                .on_click(move |_| {
                    set_counter.update(|value| *value += 1.0);
                    true
                })
                .on_event(EventListner::PointerEnter, move |_| {
                    set_is_hovered.update(|val| *val = true);
                    true
                })
                .on_event(EventListner::PointerLeave, move |_| {
                    set_is_hovered.update(|val| *val = false);
                    true
                })
                .style(|| {
                    Style::BASE
                        .border(10.0)
                        .background(Color::RED)
                        .color(Color::BLACK)
                        .padding_px(20.0)
                        .margin_px(80.0)
                        .size_px(200.0, 140.0)
                    // .width_px(150.0)
                })
                .active_style(|| Style::BASE.background(Color::PURPLE))
                .animation(
                    animation()
                        .border_radius(move || if is_hovered.get() { 1.0 } else { 40.0 })
                        .border_color(|| Color::CYAN)
                        .color(|| Color::CYAN)
                        .background(move || {
                            if is_hovered.get() {
                                Color::PINK
                            } else {
                                Color::SKY_BLUE
                            }
                        })
                        // .width(move || if is_hovered.get() { 800.0 } else { 400.0 })
                        // .scale(move || if counter.get() % 2.0 == 0.0 { 1.0 } else { 2.5 })
                        .easing_fn(EasingFn::Quartic)
                        .ease_in_out()
                        .persists(true)
                        .duration(Duration::from_secs(1)),
                ),
            label(|| "Testt".to_string()).style(|| {
                Style::BASE
                    .border(5.0)
                    .background(Color::GREEN)
                    .color(Color::BLACK)
                    .padding_px(20.0)
                    .size_px(100.0, 80.0)
            }),
        )
    })
    .style(|| {
        Style::BASE
            .border(5.0)
            .background(Color::LIGHT_GREEN)
            .padding_px(10.0)
            .size_px(300.0, 300.0)
            .color(Color::BLACK)
    })
    .animation(
        animation()
            // .width(move || {
            //     if counter.get() % 2.0 == 0.0 {
            //         400.0
            //     } else {
            //         600.0
            //     }
            // })
            // .height(move || {
            //     if counter.get() % 2.0 == 0.0 {
            //         300.0
            //     } else {
            //         800.0
            //     }
            // })
            //
            .scale(move || if counter.get() % 2.0 == 0.0 { 0.5 } else { 2.0 })
            .border_color(|| Color::CYAN)
            .color(|| Color::CYAN)
            .background(|| Color::LAVENDER)
            .easing_fn(EasingFn::Cubic)
            .ease_in_out()
            // .persists(true)
            .duration(Duration::from_secs(2)),
    )
}

fn main() {
    floem::launch(app_view);
}
