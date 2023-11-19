s floem::{
    event::{Event, EventListener},
    keyboard::{Key, NamedKey},
    kurbo::Size,
    peniko::Color,
    reactive::{create_effect, create_rw_signal, ReadSignal},
    view::View,
    views::{dyn_container, empty, label, stack, static_label, text_input, Decorators},
    window::{WindowConfig, WindowId},
    Application,
};

pub fn err_msg_container(err_msg: impl Fn() -> ReadSignal<Option<String>> + 'static) -> impl View {
    dyn_container(
        move || err_msg().get(),
        |err_msg| {
            if let Some(err_msg) = err_msg {
                Box::new(
                    label(move || format!("{}", err_msg)).style(move |s| s.color(Color::DARK_RED)),
                )
            } else {
                Box::new(empty())
            }
        },
    )
}

fn app_view(window_id: WindowId) -> impl View {
    let width = create_rw_signal(600.0);
    let height = create_rw_signal(600.0);

    let width_input = create_rw_signal(width.get_untracked().to_string());
    let height_input = create_rw_signal(height.get_untracked().to_string());

    let width_err_msg = create_rw_signal::<Option<String>>(None);
    let height_err_msg = create_rw_signal::<Option<String>>(None);

    create_effect(move |_| {
        floem::resize_window(window_id, Size::new(width.get(), height.get()));
    });

    fn get_error_msg(input: String) -> Option<String> {
        match input.parse::<f64>() {
            Ok(num) => {
                if num > 0.0 {
                    None
                } else {
                    Some("Must be greater than 0".to_string())
                }
            }
            Err(_) => Some("Must be a valid number".to_string()),
        }
    }

    let handle_submit = move || {
        width_err_msg.set(get_error_msg(width_input.get()));
        height_err_msg.set(get_error_msg(height_input.get()));

        if width_err_msg.get().is_none() && height_err_msg.get().is_none() {
            width.set(width_input.get().parse::<f64>().unwrap());
                width_err_msg.set(get_error_msg(width_input.get()));
            dth_err_msg.set(get_error_msg(width_input.get()));
            height.set(height_input.get().parse::<f64>().unwrap());
        }
    };

    stack((
        label(move || format!("Current window size: {}x{}", width.get(), height.get())),
        stack((
            static_label("New width:"),
            text_input(width_input)
                .keyboard_navigatable()
                .on_event_stop(EventListener::KeyDown, move |e| {
                    if let Event::KeyDown(e) = e {
                        if e.key.logical_key == Key::Named(NamedKey::Enter) {
                            handle_submit();
                        }
                    }
                }),
            err_msg_container(move || width_err_msg.read_only()),
        )),
        static_label("New height:"),
        text_input(height_input)
            .keyboard_navigatable()
            .on_event_stop(EventListener::KeyDown, move |e| {
                if let Event::KeyDown(e) = e {
                    if e.key.logical_key == Key::Named(NamedKey::Enter) {
                        handle_submit();
                    }
                }
            }),
        err_msg_container(move || height_err_msg.read_only()),
        static_label("Update")
            .on_click_stop(move |_| {
                handle_submit();
            })
            .keyboard_navigatable(),
    ))
}

fn main() {
    Application::new()
        .window(
            |window_id| app_view(window_id),
            Some(
                WindowConfig::default()
                    .size(Size::new(600.0, 600.0))
                    .title("Window size example"),
            ),
        )
        .run();
}
