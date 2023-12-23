use floem::peniko::Color;
use floem::reactive::{create_signal, ReadSignal, WriteSignal};
use floem::style::{AlignItems, Style};
use floem::view::View;
use floem::views::{h_stack, label, v_stack, Decorators};
use floem::widgets::{button, checkbox};
use floem::{style_class, EventPropagation};

style_class!(pub ButtonStyle);

fn spinbox(counter: ReadSignal<i32>, set_counter: WriteSignal<i32>) -> impl View {
    h_stack((
        button(|| "-").class(ButtonStyle).on_click(move |_| {
            set_counter.update(|value| *value -= 1);
            EventPropagation::Stop
        }),
        label(move || counter.get()),
        button(|| "+").class(ButtonStyle).on_click(move |_| {
            set_counter.update(|value| *value += 1);
            EventPropagation::Stop
        }),
    ))
    .style(|s| {
        let button_style = Style::new()
            .background(Color::GREEN)
            .color(Color::WHITE)
            .font_size(18.0)
            .padding(5.0);

        s.class(ButtonStyle, move |_| button_style.clone())
            .align_items(AlignItems::Center)
            .disabled(|s| {
                s.background(Color::DIM_GRAY)
                    .class(ButtonStyle, |s| s.background(Color::DARK_GRAY))
            })
            .gap(10.0, 0.0)
    })
}

fn app_view() -> impl View {
    let (counter, set_counter) = create_signal(0);
    let (spinbox_enabled, set_spinbox_enabled) = create_signal(true);

    v_stack((
        checkbox(spinbox_enabled)
            .on_click_stop(move |_| set_spinbox_enabled.set(!spinbox_enabled.get())),
        spinbox(counter, set_counter).disabled(move || !spinbox_enabled.get()),
    ))
}

fn main() {
    floem::launch(app_view);
}
