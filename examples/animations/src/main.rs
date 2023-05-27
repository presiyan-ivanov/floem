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

fn app_view(cx: AppContext) -> impl View {
    let (counter, set_counter) = create_signal(cx.scope, 0.0);
    let (is_hovered, set_is_hovered) = create_signal(cx.scope, false);

    // stack(|| {
    //     (
    label(|| "Hover or click me!".to_string())
        .on_click(move |_| {
            set_counter.update(|value| *value += 1.0);
            dbg!(counter.get());
            true
        })
        .on_event(EventListner::PointerEnter, move |_| {
            set_is_hovered.update(|val| *val = true);
            println!("pointer enter");
            true
        })
        .on_event(EventListner::PointerLeave, move |_| {
            set_is_hovered.update(|val| *val = false);
            println!("pointer leave");
            true
        })
        .style(|| {
            Style::BASE
                .border(1.0)
                .background(Color::RED)
                .color(Color::BLACK)
                .padding_px(10.0)
                .margin_px(20.0)
                .size_px(120.0, 120.0)
        })
        .active_style(|| Style::BASE.color(Color::BLACK))
        .animation(
            animation()
                .border_radius(move || if is_hovered.get() { 1.0 } else { 40.0 })
                .border_color(|| Color::CYAN)
                .color(|| Color::CYAN)
                .background(move || {
                    if is_hovered.get() {
                        Color::DARK_RED
                    } else {
                        Color::SKY_BLUE
                    }
                })
                // .translate_x(move || {
                //     if counter.get() > 0.0 {
                //         100.0
                //     } else {
                //         200.0
                //     }
                // })
                .scale(move || {
                    if counter.get() == 0.0 {
                        1.0
                    } else {
                        2.0
                    }
                })
                .easing_fn(EasingFn::Quartic)
                .ease_in_out()
                .duration(Duration::from_secs(1)),
        )
    // ,)
    // })
    // .style(|| {
    //     Style::BASE
    //         .border(5.0)
    //         .background(Color::BLUE)
    //         .padding_px(10.0)
    //         .size_px(400.0, 400.0)
    //         .color(Color::BLACK)
    // })
    // .animation(
    //     animation()
    //         .width(move || {
    //             if counter.get() % 2.0 == 0.0 {
    //                 400.0
    //             } else {
    //                 600.0
    //             }
    //         })
    //         .height(move || {
    //             if counter.get() % 2.0 == 0.0 {
    //                 200.0
    //             } else {
    //                 500.0
    //             }
    //         })
    //         .border_color(|| Color::CYAN)
    //         .color(|| Color::CYAN)
    //         .background(|| Color::LAVENDER)
    //         .easing_fn(EasingFn::Cubic)
    //         .ease_in_out()
    //         .auto_reverse(true)
    //         .duration(Duration::from_secs(2)),
    // )
}

fn main() {
    floem::launch(app_view);
}