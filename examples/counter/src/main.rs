use std::time::Duration;

use floem::action::exec_after;
use floem::event::{Event, EventListener};
use floem::keyboard::{Key, NamedKey};
use floem::peniko::Color;
use floem::reactive::{create_effect, create_rw_signal};
use floem::view::View;
use floem::views::{container, label, Decorators};

fn app_view() -> impl View {
    let root = container(label(|| "Image folder")).style(|s| {
        s.border(5.0)
            .border_color(Color::BLACK)
            .width_full()
            .height_full()
    });

    let id = root.id();
    id.request_focus();

    root.on_event_stop(EventListener::KeyUp, move |e| {
        println!("1");
        if let Event::KeyUp(e) = e {
            println!("2");

            dbg!(e);
            if e.key.logical_key == Key::Named(NamedKey::F1) {
                id.inspect();
            }
        }
    })
    .on_event_stop(EventListener::WindowGotFocus, move |_| {
        id.request_focus();
    })
}

fn main() {
    floem::launch(app_view);
}
