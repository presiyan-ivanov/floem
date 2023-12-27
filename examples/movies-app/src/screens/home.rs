use std::{
    rc::Rc,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use floem::{
    action::exec_after,
    ext_event::create_signal_from_channel,
    peniko::Color,
    reactive::{create_effect, create_rw_signal, create_signal, ReadSignal, RwSignal},
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
    ACCENT_COLOR, NEUTRAL_BG_COLOR, PRIMARY_FG_COLOR,
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
            .take(13)
            .collect(),
    );

    let popular_tv_shows = popular_tv_shows.results;
    let most_popular_tv_show = popular_tv_shows.get(0).unwrap();
    let (most_popular_tv_show, _) = create_signal(most_popular_tv_show.to_owned());
    let (popular_tv_shows, _) = create_signal(
        popular_tv_shows
            .into_iter()
            .map(|m| PosterCarouselItem::TvShow(m).to_owned())
            .take(13)
            .collect(),
    );
    let (available_width, set_available_width) = create_signal(1200.);

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
    let container = container(
        scroll(
            list(
                move || movies.get(),
                move |item| item.id(),
                move |item| poster_carousel_item(item),
            )
            .style(|s| s.gap(10.0, 0.).padding_bottom(15.)),
        )
        .style(move |s| s.width_full()),
    )
    .style(|s| s.size(100.pct(), 100.pct()).padding_vert(20.0).flex_col());

    container
}

pub enum StarsKind {
    Filled,
    Unfilled,
}

static STAR_WIDTH: f64 = 16.;
static STAR_HEIGHT: f64 = 14.;

pub fn five_stars(kind: StarsKind) -> impl View {
    let star_icon = match kind {
        StarsKind::Filled => include_str!("../../assets/filled_star.svg"),
        StarsKind::Unfilled => include_str!("../../assets/unfilled_star.svg"),
    };

    list(
        || (0..5).collect::<Vec<i32>>(),
        move |n| *n,
        move |_| {
            svg(|| star_icon.to_string()).style(|s| {
                s.size(STAR_WIDTH, STAR_HEIGHT)
                    .color(ACCENT_COLOR.with_alpha_factor(0.9))
            })
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

static CAROUSEL_CARD_WIDTH: f64 = 200.;
static CAROUSEL_CARD_HEIGHT: f64 = 300.;

fn get_bytes(url: reqwest::Url) -> Result<Vec<u8>, Error> {
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

    let img_bytes: RwSignal<Option<Vec<u8>>> = create_rw_signal(None);
    let error_msg: RwSignal<Option<String>> = create_rw_signal(None);

    // The reactive runtime is thread-local, so we need to go through a channel
    // when we are doing work in another thread
    let (success_tx, success_rx) = crossbeam_channel::bounded(1);
    let (error_tx, error_rx) = crossbeam_channel::bounded(1);
    let chan_bytes = create_signal_from_channel(success_rx);
    let chan_error = create_signal_from_channel(error_rx);
    create_effect(move |_| {
        error_msg.set(chan_error.get());
        img_bytes.set(chan_bytes.get());
    });

    std::thread::spawn(move || {
        println!("Spawning thread");
        let result = get_bytes(url);
        match result {
            Ok(body) => success_tx.send(body).unwrap(),
            Err(e) => {
                dbg!(&e);
                error_tx.send(e.to_string()).unwrap();
            }
        }
    });

    let vote_average = item.vote_average();
    v_stack((
        dyn_container(
            move || (img_bytes.get(), error_msg.get()),
            move |(img_bytes, error_msg)| -> Box<dyn View> {
                if let Some(err) = error_msg {
                    Box::new(label(move || err.to_string()))
                } else if let Some(bytes) = img_bytes {
                    Box::new(img(move || bytes.to_vec()).style(|s| s.width_full().height_full()))
                } else {
                    Box::new(spinner())
                }
            },
        )
        .style(|s| {
            s.width(CAROUSEL_CARD_WIDTH)
                .transition(BorderColor, Transition::linear(0.5))
                .transition(BorderLeft, Transition::linear(0.5))
                .transition(BorderRight, Transition::linear(0.5))
                .transition(BorderTop, Transition::linear(0.5))
                .transition(BorderBottom, Transition::linear(0.5))
                .height(CAROUSEL_CARD_HEIGHT)
                .border(4.)
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
            )),
        ))
        .style(|s| s.font_size(14.).width(200.)),
    ))
}

pub fn movie_img(movie: ReadSignal<Movie>) -> impl View {
    let url = reqwest::Url::parse(&format!(
        "https://image.tmdb.org/t/p/original{}",
        movie.get_untracked().backdrop_path.unwrap()
    ))
    .unwrap();
    let bytes: RwSignal<Option<Vec<u8>>> = create_rw_signal(None);
    let error_msg: RwSignal<Option<String>> = create_rw_signal(None);

    dyn_container(
        move || (bytes.get(), error_msg.get()),
        move |(bytes, error_msg)| -> Box<dyn View> {
            if let Some(e) = error_msg {
                Box::new(label(move || e.to_string()))
            } else if let Some(b) = bytes {
                println!("success");
                Box::new(spinner().style(|s| s.margin_left(30.pct())))
                // Box::new(img(move || b.to_vec()).style(|s| s.width_full().height_full()))
            } else {
                println!("loading");
                Box::new(spinner())
                // Box::new(label(move || "Loading..."))
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
    let backdrop_width = 1100.px();
    let backdrop_gradient = include_bytes!("../../assets/old_black_gradient3.png");

    h_stack((
        empty().style(move |s| {
            s.position(Position::Absolute)
                .width(bg_container_width)
                .height_full()
                .background(NEUTRAL_BG_COLOR)
        }),
        movie_img(movie).style(move |s| {
            s.width(100.pct())
                .margin_left(bg_container_width)
                .height_full()
        }),
        img(move || backdrop_gradient.to_vec()).style(move |s| {
            s.width(backdrop_width)
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
    .style(|s| s.width_full().height(720.))
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
