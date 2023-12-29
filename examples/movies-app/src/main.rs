pub mod data_provider;
pub mod linear_grad_backdrop;
pub mod models;
pub mod screens;
pub mod spinner;

use anyhow::{Context, Result};
use std::{error::Error, fs, path::Path, sync::Arc};

use floem::{
    event::{Event, EventListener},
    keyboard::{Key, NamedKey},
    kurbo::Size,
    peniko::Color,
    reactive::{create_effect, create_rw_signal, create_signal, provide_context, RwSignal},
    style::{Background, CursorStyle, JustifyContent, TextColor, Transition},
    unit::UnitExt,
    view::View,
    views::{
        container, container_box, empty, h_stack, label, list, scroll, stack, static_label, svg,
        tab, text, v_stack, virtual_list, Decorators, VirtualListDirection, VirtualListItemSize,
    },
    widgets::button,
    EventPropagation,
};
use models::MovieDetails;
use screens::{home::home_view, movie_details::{self, movie_details_screen}};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum MainTab {
    Home,
    Movies,
    TvShows,
    Search,
    MovieDetails(Option<MovieDetailsState>),
    TvShowDetails,
    ActorDetails,
}

enum SubTab {
    MovieDetails,
    TvShowDetails,
    ActorDetails,
}

impl MainTab {
    fn is_visible_in_nav(&self) -> bool {
        match self {
            MainTab::Home => true,
            MainTab::Movies => true,
            MainTab::TvShows => true,
            MainTab::Search => true,
            MainTab::MovieDetails(_) => false,
            MainTab::TvShowDetails => false,
            MainTab::ActorDetails => false,
        }
    }
    fn movie_id(&self) -> Option<u64> {
        match self {
            MainTab::MovieDetails(Some(MovieDetailsState { movie_id })) => Some(*movie_id),
            x => None,
        }
    }

    fn tv_show_id(&self) -> Option<u64> {
        match self {
            MainTab::TvShowDetails => Some(0),
            _ => None,
        }
    }

    fn actor_id(&self) -> Option<u64> {
        match self {
            MainTab::ActorDetails => Some(0),
            _ => None,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct MovieDetailsState {
    movie_id: u64,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct TvShowDetails {
    tv_show_id: u64,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct ActorDetails {
    actor_id: u64,
}

// #[derive(Clone, PartialEq, Eq, Hash, Debug)]
// enum TabState {
//     MovieDetails { movie_id: u64 },
//     TvShowDetails { tv_show_id: u64 },
//     ActorDetails { actor_id: u64 },
// }
//
// impl TabState {
//     fn movie_id(&self) -> Option<u64> {
//         match self {
//             TabState::MovieDetails { movie_id } => Some(*movie_id),
//             _ => None,
//         }
//     }

// fn tv_show_id(&self) -> Option<u64> {
//     match self {
//         TabState::TvShowDetails { tv_show_id } => Some(*tv_show_id),
//         _ => None,
//     }
// }
//
// fn actor_id(&self) -> Option<u64> {
//     match self {
//         TabState::ActorDetails { actor_id } => Some(*actor_id),
//         _ => None,
//     }
// }
// }

impl MainTab {
    fn index(&self) -> usize {
        match self {
            MainTab::Home => 0,
            MainTab::Movies => 1,
            MainTab::TvShows => 2,
            MainTab::Search => 3,
            MainTab::MovieDetails { .. } => 4,
            MainTab::TvShowDetails => 5,
            MainTab::ActorDetails => 6,
        }
    }

    fn from_str(s: &str) -> Option<MainTab> {
        dbg!(s);
        match s {
            "Home" => Some(MainTab::Home),
            "Movies" => Some(MainTab::Movies),
            "TvShows" => Some(MainTab::TvShows),
            "Search" => Some(MainTab::Search),
            "MovieDetails" => Some(MainTab::MovieDetails(None)),
            "TvShowDetails" => Some(MainTab::TvShowDetails),
            "ActorDetails" => Some(MainTab::ActorDetails),
            _ => None,
        }
    }

    fn from_index(idx: usize) -> MainTab {
        match idx {
            0 => MainTab::Home,
            1 => MainTab::Movies,
            2 => MainTab::TvShows,
            3 => MainTab::Search,
            _ => panic!("Invalid tab : {}", idx),
        }
    }
}

static PRIMARY_FG_COLOR: Color = Color::WHITE;
static ACCENT_COLOR: Color = Color::rgb8(64, 193, 173);
static DIMMED_ACCENT_COLOR: Color = Color::rgb8(0, 173, 153);
static NEUTRAL_BG_COLOR: Color = Color::BLACK;
static SECONDARY_BG_COLOR: Color = Color::rgb8(20, 20, 20);
static BG_COLOR_2: Color = Color::rgb8(32, 33, 36);

struct GlobalState {
    active_tab: RwSignal<MainTab>,
    window_size: RwSignal<Size>,
    main_tab_size: RwSignal<Size>,
    data_provider: DataProvider,
    // tab_state: RwSignal<Option<TabState>>,
}

struct DataProvider {
    client: reqwest::blocking::Client,
}

impl DataProvider {
    fn get_bytes(&self, url: reqwest::Url) -> Result<Vec<u8>, reqwest::Error> {
        self.client
            .get(url)
            .send()?
            .error_for_status()?
            .bytes()
            .map(|b| b.to_vec())
    }

    fn build_backdrop_url(&self, poster: &str) -> reqwest::Url {
        reqwest::Url::parse(&format!("https://image.tmdb.org/t/p/original{}", poster)).unwrap()
    }

    fn build_poster_url(&self, poster: &str) -> reqwest::Url {
        reqwest::Url::parse(&format!("https://image.tmdb.org/t/p/w500{}", poster)).unwrap()
    }

    fn get_poster_img(&self, id: &str) -> Result<Vec<u8>, reqwest::Error> {
        self.get_bytes(self.build_poster_url(id))
    }

    fn get_backdrop_img(&self, id: &str) -> Result<Vec<u8>, reqwest::Error> {
        self.get_bytes(self.build_backdrop_url(id))
    }

    fn get_movie_details(&self, movie_id: u64) -> Result<MovieDetails> {
        let path = &format!("./../assets/data/movie_details/{movie_id}.json");
        let path = Path::new(path);
        let data = fs::read_to_string(path).context("Failed to read movie details")?;
        serde_json::from_str::<MovieDetails>(&data).context("Failed to parse movie details json")
    }
}

struct Db {
    tv_shows: im::HashMap<u64, models::TvShow>,
    movies: im::HashMap<u64, models::Movie>,
    // actors: im::HashMap<u64, models::Actor>,
}

static MAIN_TAB_WIDTH: f64 = 60.0;

fn app_view() -> impl View {
    let state = Arc::new(GlobalState {
        active_tab: create_rw_signal(MainTab::Home),
        window_size: create_rw_signal(Size::ZERO),
        main_tab_size: create_rw_signal(Size::ZERO),
        data_provider: DataProvider {
            client: reqwest::blocking::Client::new(),
        },
    });

    let active_tab = state.active_tab;
    let window_size = state.window_size;
    let main_tab_size = state.main_tab_size;
    provide_context(state.clone());
    let tabs: im::Vector<&str> = vec![
        "Home",
        "Movies",
        "TvShows",
        "Search",
        "MovieDetails",
        "TvShowDetails",
        "ActorDetails",
    ]
    .into_iter()
    .collect();
    let (tabs, _set_tabs) = create_signal(tabs);
    let home_icon = include_str!("../assets/home_icon.svg");
    let movie_icon = include_str!("../assets/movie_icon.svg");
    let tv_icon = include_str!("../assets/tv_icon.svg");
    let search_icon = include_str!("../assets/search_icon.svg");

    create_effect(move |_| {
        let window_size = window_size.get();
        let size = Size::new(window_size.width - MAIN_TAB_WIDTH, window_size.height);
        main_tab_size.set(size);
    });

    let list = list(
        move || tabs.get(),
        move |item| *item,
        move |item| {
            let index = tabs
                .get_untracked()
                .iter()
                .position(|it| *it == item)
                .unwrap();
            let tab = MainTab::from_str(item).unwrap();
            let tab2 = tab.clone();
            v_stack((svg(move || match tab {
                MainTab::Home => home_icon.to_string(),
                MainTab::Movies => movie_icon.to_string(),
                MainTab::TvShows => tv_icon.to_string(),
                MainTab::Search => search_icon.to_string(),
                MainTab::MovieDetails(_) => "".to_owned(),
                MainTab::TvShowDetails => "".to_owned(),
                MainTab::ActorDetails => "".to_owned(),
            })
            .style(move |s| {
                s.size(22.px(), 22.px())
                    .color(PRIMARY_FG_COLOR)
                    .apply_if(!tab2.is_visible_in_nav(), move |s| s.hide())
                    .apply_if(index == active_tab.get().index(), |s| s.color(ACCENT_COLOR))
            }),))
            .on_click_stop(move |_| {
                active_tab.update(|v: &mut MainTab| {
                    *v = MainTab::from_index(
                        tabs.get_untracked()
                            .iter()
                            .position(|it| *it == item)
                            .unwrap(),
                    );
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
            .width(MAIN_TAB_WIDTH)
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

    // let tab_state = state.tab_state;
    let tab = tab(
        move || active_tab.get().index(),
        move || tabs.get(),
        |it| *it,
        move |it| match MainTab::from_str(it).unwrap() {
            MainTab::Home => container_box(home_view()).style(|s| s.width_full()),
            MainTab::Movies => container_box(movies_view()),
            MainTab::TvShows => container_box(tv_shows_view()),
            MainTab::Search => container_box(label(|| "Not implemented".to_owned())),
            MainTab::MovieDetails(_) => container_box(movie_details_screen()),
            MainTab::TvShowDetails => container_box(label(move || format!("TvShow ??"))),
            MainTab::ActorDetails => container_box(label(move || format!("Actor ??"))),
        },
    )
    .style(|s| s.flex_row().items_start().width_full());

    let tab_contents = scroll(v_stack((tab.style(|s| s.flex_grow(1.)), footer()))).style(|s| {
        s.flex_basis(0)
            .padding(0.0)
            .margin(0.)
            .height_full()
            .flex_grow(1.0)
            .color(PRIMARY_FG_COLOR)
    });

    let app_view = h_stack((nav_left, tab_contents))
        .style(|s| {
            s.width_full()
                .height_full()
                .background(SECONDARY_BG_COLOR)
                .class(scroll::Handle, |s| {
                    s.border_radius(4.0)
                        .background(Color::rgba8(166, 166, 166, 140))
                        .set(scroll::Thickness, 10.0)
                        .set(scroll::Rounded, true)
                        .active(|s| s.background(Color::rgb8(166, 166, 166)))
                        .hover(|s| s.background(Color::rgb8(184, 184, 184)))
                })
                .class(scroll::Track, |s| {
                    s.hover(|s| s.background(Color::rgba8(166, 166, 166, 30)))
                })
        })
        .on_event_stop(EventListener::WindowResized, move |event| {
            if let Event::WindowResized(size) = event {
                window_size.set(*size);
            }
        })
        .window_title(|| "Floem Movies".to_owned());

    let id = app_view.id();
    app_view.on_event_stop(EventListener::KeyUp, move |e| {
        if let Event::KeyUp(e) = e {
            if e.key.logical_key == Key::Named(NamedKey::F11) {
                id.inspect();
            }
        }
    })
}

struct State {
    active_tab: MainTab,
}

struct MovieDb {}

impl MovieDb {}

fn footer() -> impl View {
    let lapce_logo = include_str!("../assets/lapce_logo.svg");
    v_stack(
        (
            h_stack(
            (
            svg(move || lapce_logo.to_string()) .style(|s| s.size(30.px(), 30.px()).color(ACCENT_COLOR)),
            text("Floem Movies").style(|s| s.font_size(20.).height(30.px()).margin_left(5.))
        )).style(|s| s.height(70.px()).padding_vert(20.)),
            text("Designed by the Nuxt Movies authors, and ported by the Floem devs, with the original data provided by TMDb.")
        )
    ).style(|s| s.margin_left(40.))
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
