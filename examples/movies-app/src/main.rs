use floem::{
    event::{Event, EventListener},
    keyboard::{Key, NamedKey},
    peniko::Color,
    reactive::create_signal,
    style::{Background, CursorStyle, Transition},
    unit::UnitExt,
    view::View,
    views::{
        container, container_box, h_stack, label, scroll, stack, tab, v_stack, virtual_list,
        Decorators, VirtualListDirection, VirtualListItemSize, static_label, list,
    },
    widgets::button,
    EventPropagation,
};

fn app_view() -> impl View {
    let tabs: im::Vector<&str> = vec!["Home", "Movies", "TvShows"].into_iter().collect();
    let (tabs, _set_tabs) = create_signal(tabs);

    let (active_tab, set_active_tab) = create_signal(0);

    let list = list(
        move || tabs.get(),
        move |item| *item,
        move |item| {
            let index = tabs
                .get_untracked()
                .iter()
                .position(|it| *it == item)
                .unwrap();
            v_stack((label(move || item).style(|s| s.font_size(18.0)),))
                .on_click_stop(move |_| {
                    set_active_tab.update(|v: &mut usize| {
                        *v = tabs
                            .get_untracked()
                            .iter()
                            .position(|it| *it == item)
                            .unwrap();
                    });
                })
                .keyboard_navigatable()
                .draggable()
                .style(move |s| {
                    s.flex_row()
                        .padding(5.0)
                        .width(100.pct())
                        .height(36.0)
                        .transition(Background, Transition::linear(0.4))
                        .items_center()
                        .border_bottom(1.0)
                        .border_color(Color::LIGHT_GRAY)
                        .apply_if(index == active_tab.get(), |s| {
                            s.background(Color::GRAY.with_alpha_factor(0.6))
                        })
                        .focus_visible(|s| s.border(2.).border_color(Color::BLUE))
                        .hover(|s| {
                            s.background(Color::LIGHT_GRAY)
                                .apply_if(index == active_tab.get(), |s| s.background(Color::GRAY))
                                .cursor(CursorStyle::Pointer)
                        })
                })
        },
    ).style(|s| s.flex_col());

    let list = container(list).style(|s| {
        s.border(1.0)
            .border_color(Color::GRAY)
            .flex_grow(1.0)
            .min_height(0)
    });

    let id = list.id();
    let inspector = button(|| "Open Inspector")
        .on_click_stop(move |_| {
            id.inspect();
        })
        .style(|s| s);

    let left = v_stack((list, inspector)).style(|s| s.height_full().gap(0.0, 5.0));

    let tab = tab(
        move || active_tab.get(),
        move || tabs.get(),
        |it| *it,
        |it| match it {
            "Home" => container_box(home_view()),
            "Movies" => container_box(movies_view()),
            "TvShows" => container_box(tv_shows_view()),
            _ => container_box(label(|| "Not implemented".to_owned())),
        },
    )
    .style(|s| s.flex_row().items_start());

    let tab = scroll(tab).style(|s| s.flex_basis(0).min_width(0).flex_grow(1.0));

    let view = h_stack((left, tab))
        .style(|s| s.padding(5.0).width_full().height_full().gap(5.0, 0.0))
        .window_title(|| "Movies App".to_owned());

    let id = view.id();
    view.on_event_stop(EventListener::KeyUp, move |e| {
        if let Event::KeyUp(e) = e {
            if e.key.logical_key == Key::Named(NamedKey::F11) {
                id.inspect();
            }
        }
    })
}

fn home_view() -> impl View {
    static_label("Home").style(|s| s.font_size(24.0))
}

fn movies_view() -> impl View {
    static_label("Movies").style(|s| s.font_size(24.0))
}

fn tv_shows_view() -> impl View {
    static_label("TvShows").style(|s| s.font_size(24.0))
}

fn main() {
    floem::launch(app_view);
}
