use floem::{
    event::{Event, EventListener},
    keyboard::{Key, NamedKey},
    reactive::create_rw_signal,
    view::View,
    views::{h_stack, text, v_stack, Decorators},
    widgets::{button, text_input},
};

fn app_view() -> impl View {
    let value = create_rw_signal(String::from(""));

    let view = h_stack((
        text("Label: "),
        text_input(value).style(|s| s.width_pct(100.0)),
    ))
    .style(|s| s.width_full().items_center());

    let id = view.id();

    // v_stack((
    //     view,
    //     button(|| "Open Inspector").on_click_stop(move |_| {
    //         id.inspect();
    //     }),
    // ))
    view
}

fn main() {
    floem::launch(app_view);
}
