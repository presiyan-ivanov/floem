use floem::{
    peniko::Color,
    reactive::create_rw_signal,
    view::View,
    views::{h_stack, text, v_stack, Decorators, Label},
    widgets::text_input,
};

use crate::SECONDARY_FG_COLOR;

fn form_field(name: &str, view: impl View + 'static) -> impl View {
    v_stack((
        // text(name).style(|s| s.width(100.).color(SECONDARY_FG_COLOR)),
        text(name),
        view,
    ))
    .style(|s| s.margin_bottom(10.))
}

pub fn add_media() -> impl View {
    let title = create_rw_signal("Sample title".to_owned());
    let description = create_rw_signal(String::new());
    let duration_minutes = create_rw_signal(String::new());

    v_stack((
        form_field("Title", text_input(title)),
        form_field("Description", text_input(description)),
        form_field("Duration(in minutes)", text_input(duration_minutes)),
    ))
    .style(|s| s.padding_left(30))
}
