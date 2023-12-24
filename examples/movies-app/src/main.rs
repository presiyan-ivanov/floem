pub mod data_provider;
pub mod linear_grad_backdrop;
pub mod models;
pub mod screens;

use floem::{
    event::{Event, EventListener},
    keyboard::{Key, NamedKey},
    peniko::Color,
    reactive::create_signal,
    style::{Background, CursorStyle, JustifyContent, TextColor, Transition},
    unit::UnitExt,
    view::View,
    views::{
        container, container_box, h_stack, label, list, scroll, stack, static_label, svg, tab,
        v_stack, virtual_list, Decorators, VirtualListDirection, VirtualListItemSize,
    },
    widgets::button,
    EventPropagation,
};
use screens::home::home_view;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum MainTab {
    Home,
    Movies,
    TvShows,
}

static PRIMARY_FG_COLOR: Color = Color::WHITE;
static ACCENT_COLOR: Color = Color::rgb8(64, 193, 173);
static NEUTRAL_BG_COLOR: Color = Color::BLACK;
static SECONDARY_BG_COLOR: Color = Color::rgb8(20, 20, 20);
static BG_COLOR_2: Color = Color::rgb8(32, 33, 36);

fn app_view() -> impl View {
    let tabs: im::Vector<&str> = vec!["Home", "Movies", "TvShows", "Search"]
        .into_iter()
        .collect();
    let (tabs, _set_tabs) = create_signal(tabs);
    let home_icon = include_str!("../assets/home_icon.svg");
    let movie_icon = include_str!("../assets/movie_icon.svg");
    let tv_icon = include_str!("../assets/tv_icon.svg");
    let search_icon = include_str!("../assets/search_icon.svg");

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
                "Search" => search_icon.to_string(),
                x => panic!("Unknown tab: {}", x),
            })
            .style(move |s| {
                s.size(22.px(), 22.px())
                    .color(PRIMARY_FG_COLOR)
                    .apply_if(index == active_tab.get(), |s| s.color(ACCENT_COLOR))
            }),))
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
                s.padding_vert(10.)
                    .transition(TextColor, Transition::linear(0.4))
                    .transition(Background, Transition::linear(0.4))
                    .color(PRIMARY_FG_COLOR)
                    .items_center()
                    .focus_visible(|s| {
                        s.border(3.)
                            .border_color(ACCENT_COLOR.with_alpha_factor(0.8))
                            .border_radius(5.)
                    })
                    .hover(|s| s.background(BG_COLOR_2).cursor(CursorStyle::Pointer))
            })
        },
    )
    .style(|s| {
        s.flex_col()
            .height_full()
            .justify_content(Some(JustifyContent::SpaceAround))
            .padding_vert(60.)
            .width(60.)
            .background(NEUTRAL_BG_COLOR)
            .border_color(BG_COLOR_2)
            .border_right(1.0)
    });

    let list = container(list).style(|s| s.flex_grow(1.0).min_height(0));

    let id = list.id();
    let inspector = button(|| "Inspect")
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
            "Home" => container_box(home_view()).style(|s| s.width_full()),
            "Movies" => container_box(movies_view()),
            "TvShows" => container_box(tv_shows_view()),
            "Search" => container_box(label(|| "Not implemented".to_owned())),
            _ => container_box(label(|| "Not implemented".to_owned())),
        },
    )
    .style(|s| s.flex_row().items_start().width_full());

    let tab_contents = scroll(tab).style(|s| {
        s.flex_basis(0)
            .padding(0.0)
            .margin(0.)
            .height_full()
            .flex_grow(1.0)
            .color(PRIMARY_FG_COLOR)
    });

    let view = h_stack((nav_left, tab_contents))
        .style(|s| s.width_full().height_full().background(SECONDARY_BG_COLOR))
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

fn movies_view() -> impl View {
    static_label("Movies").style(|s| s.font_size(24.0))
}

fn tv_shows_view() -> impl View {
    static_label("TvShows").style(|s| s.font_size(24.0))
}

fn main() {
    floem::launch(app_view);
}
