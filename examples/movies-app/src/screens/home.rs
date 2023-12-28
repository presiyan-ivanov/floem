use std::{
    rc::Rc,
    sync::{atomic::AtomicUsize, Arc, Mutex},
    thread,
    time::Duration,
};

use floem::{
    action::exec_after,
    ext_event::create_signal_from_channel,
    kurbo::Size,
    peniko::Color,
    reactive::{create_effect, create_rw_signal, create_signal, use_context, ReadSignal, RwSignal},
    style::{
        BorderBottom, BorderColor, BorderLeft, BorderRadius, BorderRight, BorderTop, CursorStyle,
        Position, Transition,
    },
    unit::UnitExt,
    view::View,
    views::{
        clip, container, dyn_container, empty, h_stack, img, label, list, scroll, stack,
        static_label, static_list, svg, v_stack, virtual_list, Decorators, VirtualListDirection,
        VirtualListItemSize,
    },
};
use reqwest::{Error, Response};

use crate::{
    models::{Movie, Page, TvShow},
    spinner::{self, spinner},
    GlobalState, MovieDetails, Tab, ACCENT_COLOR, BG_COLOR_2, DIMMED_ACCENT_COLOR,
    NEUTRAL_BG_COLOR, PRIMARY_FG_COLOR,
};

pub fn home_view() -> impl View {
    let movies_json = include_str!("../../assets/data/popular_movies.json");
    let tv_shows_json = include_str!("../../assets/data/popular_tv_shows.json");
    let popular_movies: Page<Movie> =
        serde_json::from_str(movies_json).expect("JSON was not well-formatted");

    let popular_tv_shows: Page<TvShow> =
        serde_json::from_str(tv_shows_json).expect("JSON was not well-formatted");

    let popular_movies = popular_movies.results;
    let most_popular_movie = popular_movies.get(0).unwrap();
    let (most_popular_movie, _) = create_signal(most_popular_movie.to_owned());
    let (popular_movies, _) = create_signal(
        popular_movies
            .into_iter()
            .map(|m| PosterCarouselItem::Movie(m).to_owned())
            .take(12)
            .collect(),
    );

    let popular_tv_shows = popular_tv_shows.results;
    let most_popular_tv_show = popular_tv_shows.get(0).unwrap();
    let (most_popular_tv_show, _) = create_signal(most_popular_tv_show.to_owned());
    let (popular_tv_shows, _) = create_signal(
        popular_tv_shows
            .into_iter()
            .map(|m| PosterCarouselItem::TvShow(m).to_owned())
            .take(12)
            .collect(),
    );
    let (available_width, set_available_width) = create_signal(2100.);

    scroll(
        v_stack((
            movie_hero_container(most_popular_movie),
            v_stack((
                label(move || "Popular Movies")
                    .style(|s| s.font_size(20.).margin_top(10.).padding(5.)),
                carousel(popular_movies),
            ))
            .style(move |s| s.padding(20.0).width(available_width.get())),
            v_stack((
                label(move || "Popular TV shows")
                    .style(|s| s.font_size(20.).margin_top(10.).padding(5.)),
                carousel(popular_tv_shows),
            ))
            .style(move |s| s.padding(20.0).width(available_width.get())),
        ))
        .style(|s| s.width_full()),
    )
    .on_resize(move |rect| {
        println!("size: {:?}", rect.size());
        set_available_width.update(move |width| *width = rect.width());
    })
    .style(|s| s.width_full())
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum PosterCarouselItem {
    Movie(Movie),
    TvShow(TvShow),
}

impl PosterCarouselItem {
    fn id(&self) -> u64 {
        match self {
            Self::Movie(m) => m.id,
            Self::TvShow(t) => t.id,
        }
    }

    fn display_name(&self) -> String {
        match self {
            Self::Movie(m) => m.title.clone(),
            Self::TvShow(t) => t.name.clone(),
        }
    }

    fn vote_average(&self) -> f64 {
        match self {
            Self::Movie(m) => m.vote_average,
            Self::TvShow(t) => t.vote_average,
        }
    }

    fn poster_path(&self) -> Option<String> {
        match self {
            Self::Movie(m) => m.poster_path.clone(),
            Self::TvShow(t) => t.poster_path.clone(),
        }
    }
}

pub fn carousel(movies: ReadSignal<im::Vector<PosterCarouselItem>>) -> impl View {
    let state: Arc<GlobalState> = use_context().unwrap();
    let container = container(
        scroll(
            list(
                move || movies.get(),
                move |item| item.id(),
                move |item| poster_carousel_item(item),
            )
            .style(|s| s.gap(10.0, 0.).padding_bottom(15.)),
        )
        .style(move |s| s.width(state.window_size.get().width)),
    )
    .style(|s| s.size(100.pct(), 100.pct()).padding_vert(20.0).flex_col());

    container
}

pub enum StarsKind {
    Filled,
    Unfilled,
}

static STAR_WIDTH: f64 = 14.;
static STAR_HEIGHT: f64 = 12.;

pub fn five_stars(kind: StarsKind) -> impl View {
    let star_icon = match kind {
        StarsKind::Filled => include_str!("../../assets/filled_star.svg"),
        StarsKind::Unfilled => include_str!("../../assets/unfilled_star.svg"),
    };

    list(
        || (0..5).collect::<Vec<i32>>(),
        move |n| *n,
        move |_| {
            svg(|| star_icon.to_string())
                .style(|s| s.size(STAR_WIDTH, STAR_HEIGHT).color(DIMMED_ACCENT_COLOR))
        },
    )
}

pub fn stars_rating_bar(rating: f64) -> impl View {
    debug_assert!(rating >= 0. && rating <= 10.);
    let width = STAR_WIDTH * 5.;
    let height = STAR_HEIGHT;
    let clip_width = width * (rating / 10.);

    h_stack((
        five_stars(StarsKind::Unfilled).style(move |s| s.position(Position::Absolute)),
        clip(five_stars(StarsKind::Filled).style(move |s| s)).style(move |s| s.width(clip_width)),
    ))
    .style(move |s| s.width(width).height(height))
}

static CAROUSEL_CARD_IMG_WIDTH: f64 = 200.;
static CAROUSEL_CARD_IMG_HEIGHT: f64 = 300.;

fn get_bytes(url: reqwest::Url) -> Result<Vec<u8>, reqwest::Error> {
    reqwest::blocking::get(url)?
        .error_for_status()?
        .bytes()
        .map(|b| b.to_vec())
}

pub fn poster_carousel_item(item: PosterCarouselItem) -> impl View {
    let url = reqwest::Url::parse(&format!(
        "https://image.tmdb.org/t/p/w500{}",
        item.poster_path().unwrap()
    ))
    .unwrap();

    let img_bytes: RwSignal<Option<Result<Vec<u8>, String>>> = create_rw_signal(None);

    let (success_tx, success_rx) = crossbeam_channel::bounded(1);
    // The reactive runtime is thread-local, so we need to notify the runtime
    // when we are doing work in another thread
    let chan_bytes = create_signal_from_channel(success_rx);
    create_effect(move |_| {
        img_bytes.set(chan_bytes.get());
    });

    std::thread::spawn(move || {
        let result = get_bytes(url).map_err(|e| e.to_string());
        success_tx.send(result).unwrap();
    });

    let vote_average = item.vote_average();
    let id = item.id();
    let state: Arc<GlobalState> = use_context().unwrap();
    let active_tab = state.active_tab;
    // let tab_state = state.tab_state;
    v_stack((
        dyn_container(
            move || img_bytes.get(),
            move |img_bytes| -> Box<dyn View> {
                match img_bytes {
                    Some(resp) => match resp {
                        Ok(bytes) => Box::new(
                            img(move || bytes.to_vec()).style(|s| s.width_full().height_full()),
                        ),
                        Err(err_msg) => {
                            eprintln!("error: {}", err_msg);
                            let image_error = include_str!("../../assets/image-error.svg");
                            Box::new(svg(move || image_error.to_owned()))
                        }
                    },
                    None => Box::new(spinner()),
                }
            },
        )
        .on_click_stop(move |_| {
            active_tab
                .update(move |tab| *tab = Tab::MovieDetails(Some(MovieDetails { movie_id: id })));
        })
        .style(|s| {
            s.width(CAROUSEL_CARD_IMG_WIDTH)
                .transition(BorderColor, Transition::linear(0.5))
                .transition(BorderLeft, Transition::linear(0.5))
                .transition(BorderRight, Transition::linear(0.5))
                .transition(BorderTop, Transition::linear(0.5))
                .transition(BorderBottom, Transition::linear(0.5))
                .height(CAROUSEL_CARD_IMG_HEIGHT)
                .border(2.)
                //TODO: transition doesnt look good if it has opacity here, because the border
                //edges are overlapping
                .border_color(Color::rgb8(37, 37, 38))
                .hover(|s| {
                    s.cursor(CursorStyle::Pointer)
                        .border(7.0)
                        .border_color(Color::rgb8(107, 107, 107))
                })
        }),
        // img(move || poster.to_vec()).style(|s| {
        //     s.width(200.)
        //         .height(300.)
        //         .border(3.)
        //         .border_color(Color::rgba(156., 163., 175., 0.1))
        // }),
        v_stack((
            label(move || item.display_name().clone()),
            h_stack((
                stars_rating_bar(vote_average),
                label(move || format!("{:.1}", vote_average)).style(|s| s.margin_left(5.)),
            ))
            .style(|s| s.margin_top(5.).width_full()),
        ))
        .style(|s| {
            s.font_size(14.)
                .width(200.)
                .padding_vert(10.)
                .padding_horiz(5.)
                .background(BG_COLOR_2)
                .border_radius(10.)
        }),
    ))
}

pub fn movie_img(movie: ReadSignal<Movie>) -> impl View {
    let url = reqwest::Url::parse(&format!(
        "https://image.tmdb.org/t/p/original{}",
        movie.get_untracked().backdrop_path.unwrap()
    ))
    .unwrap();
    let img_bytes: RwSignal<Option<Result<Vec<u8>, String>>> = create_rw_signal(None);
    let (success_tx, success_rx) = crossbeam_channel::bounded(1);
    // The reactive runtime is thread-local, so we need to notify the runtime
    // when we are doing work in another thread
    let chan_bytes = create_signal_from_channel(success_rx);
    create_effect(move |_| {
        img_bytes.set(chan_bytes.get());
    });

    std::thread::spawn(move || {
        let result = get_bytes(url).map_err(|e| e.to_string());
        success_tx.send(result).unwrap();
    });

    dyn_container(
        move || img_bytes,
        move |img_bytes| -> Box<dyn View> {
            match img_bytes.get() {
                Some(resp) => match resp {
                    Ok(bytes) => {
                        println!("bytes: {:?}", bytes.len());
                        Box::new(
                            img(move || bytes.to_vec()).style(|s| s.width_full().height_full()),
                        )
                    }
                    Err(err_msg) => {
                        eprintln!("error: {}", err_msg);
                        let image_error = include_str!("../../assets/image-error.svg");
                        Box::new(svg(move || image_error.to_owned()))
                    }
                },
                None => Box::new(spinner()),
            }
        },
    )
    .style(|s| s.width_full().height_full())
}

pub fn movie_hero_container(movie: ReadSignal<Movie>) -> impl View {
    let release_year =
        movie.with_untracked(|m| m.release_date.split('-').next().unwrap().to_owned());
    let movie_details_width = 50.pct();
    let bg_container_width = 30.pct();
    let backdrop_gradient = include_bytes!("../../assets/old_black_gradient3.png");
    let state: Arc<GlobalState> = use_context().unwrap();
    let win_size = create_rw_signal(Size::new(2100., 800.));
    // let backdrop_width = window_width / 2.;
    // dbg!(window_width);
    println!("win size: {:?}", win_size.get().width);

    h_stack((
        empty().style(move |s| {
            s.position(Position::Absolute)
                .width(bg_container_width)
                .height_full()
                .background(NEUTRAL_BG_COLOR)
        }),
        movie_img(movie)
            .style(move |s| s.width_full().margin_left(bg_container_width).height_full()),
        img(move || backdrop_gradient.to_vec()).style(move |s| {
            s.width(win_size.get().width)
                .height_full()
                .margin_left(29.5.pct())
                .position(Position::Absolute)
            // .border_color(Color::rgba(0., 0., 0., 0.1))
        }),
        v_stack((
            label(move || movie.get().title).style(|s| s.font_size(40.0).margin_vert(15.0)),
            h_stack((
                stars_rating_bar(movie.get().vote_average),
                label(move || format!("{:.1}", movie.get().vote_average))
                    .style(|s| s.margin_horiz(12.0)),
                label(move || format!("{} Reviews", pretty_format_number(movie.get().vote_count)))
                    .style(|s| s.margin_right(12.0)),
                label(move || release_year.clone()).style(|s| s.margin_right(12.0)),
                label(move || "1h 20m"),
            ))
            .style(|s| s.color(Color::rgb8(153, 153, 153))),
            label(move || movie.get().overview.unwrap_or_default()).style(|s| {
                s.color(PRIMARY_FG_COLOR)
                    .width_pct(70.)
                    .margin_top(20.0)
                    .font_size(18.)
            }),
        ))
        .style(move |s| {
            s.position(Position::Absolute)
                .padding(20.)
                .width(movie_details_width)
                .justify_center()
                .height_full()
        }),
    ))
    .style(move |s| {
        s.width(win_size.get().width)
            .height(win_size.get().width / 2.)
    })
}

fn pretty_format_number(num: u64) -> String {
    let thousands: u64 = 1_000;
    let mil: u64 = 1_000_000;
    let bil: u64 = 1_000_000_000;

    if num < thousands {
        num.to_string()
    } else if num < mil {
        format!("{:.1}K", num as f64 / thousands as f64)
    } else if num < bil {
        format!("{:.1}M", num as f64 / mil as f64)
    } else {
        format!("{:.1}B", num as f64 / bil as f64)
    }
}
