pub mod data_provider;
pub mod models;

use floem::{
    event::{Event, EventListener},
    keyboard::{Key, NamedKey},
    peniko::Color,
    reactive::create_signal,
    style::{Background, CursorStyle, Transition},
    unit::UnitExt,
    view::View,
    views::{
        container, container_box, h_stack, label, list, scroll, stack, static_label, svg, tab,
        v_stack, virtual_list, Decorators, VirtualListDirection, VirtualListItemSize,
    },
    widgets::button,
    EventPropagation,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum MainTab {
    Home,
    Movies,
    TvShows,
}

fn app_view() -> impl View {
    let tabs: im::Vector<&str> = vec!["Home", "Movies", "TvShows"].into_iter().collect();
    let (tabs, _set_tabs) = create_signal(tabs);
    let home_icon = include_str!("../assets/home_icon.svg");
    let movie_icon = include_str!("../assets/movie_icon.svg");
    let tv_icon = include_str!("../assets/tv_icon.svg");

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
            v_stack((svg(move || match item {
                "Home" => home_icon.to_string(),
                "Movies" => movie_icon.to_string(),
                "TvShows" => tv_icon.to_string(),
                x => panic!("Unknown tab: {}", x),
            })
            .style(|s| s.size(22., 22.).color(Color::WHITE)),))
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
                s.padding(5.0)
                    .width(70.px())
                    .margin_top(40.0)
                    .transition(Background, Transition::linear(0.4))
                    .items_center()
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
    )
    .style(|s| {
        s.flex_col()
            .padding_top(50.)
            .background(Color::BLACK)
            .border_color(Color::rgb8(32, 33, 36))
            .border_right(1.0)
    });

    let list = container(list).style(|s| s.flex_grow(1.0).min_height(0));

    let id = list.id();
    let inspector = button(|| "Open Inspector")
        .on_click_stop(move |_| {
            id.inspect();
        })
        .style(|s| s);

    let nav_left = v_stack((list, inspector)).style(|s| s.height_full().gap(0.0, 5.0));

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

    let tab_contents = scroll(tab).style(|s| {
        s.flex_basis(0)
            .width_full()
            .flex_grow(1.0)
            .color(Color::WHITE)
    });

    let view = h_stack((nav_left, tab_contents))
        .style(|s| {
            s.width_full()
                .height_full()
                .gap(5.0, 0.0)
                .background(Color::rgb8(20, 20, 20))
        })
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
