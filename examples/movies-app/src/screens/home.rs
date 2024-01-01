use std::sync::Arc;

use floem::{
    ext_event::create_signal_from_channel,
    peniko::Color,
    reactive::{create_effect, create_rw_signal, create_signal, use_context, ReadSignal, RwSignal},
    style::{
        BorderBottom, BorderColor, BorderLeft, BorderRight, BorderTop, CursorStyle, Position,
        Transition, AlignItems,
    },
    style_class,
    unit::UnitExt,
    view::View,
    views::{
        clip, container, dyn_container, empty, h_stack, img, label, list, scroll, svg, text,
        v_stack, Decorators,
    },
};

use crate::{
    models::{Movie, Page, TvShow},
    spinner::spinner,
    ActiveTabKind, GlobalState, MainTab, MovieDetailsState, SubTab, ACCENT_BG_COLOR, ACCENT_COLOR,
    DIMMED_ACCENT_COLOR, PRIMARY_FG_COLOR, SECONDARY_BG_COLOR,
};

pub fn home_view() -> impl View {
    let movies_json = include_str!("../../assets/data/popular_movies.json");
    let tv_shows_json = include_str!("../../assets/data/tv_shows/popular.json");
    let popular_movies: Page<Movie> =
        serde_json::from_str(movies_json).expect("JSON was not well-formatted");

    let popular_tv_shows: Page<TvShow> =
        serde_json::from_str(tv_shows_json).expect("JSON was not well-formatted");

    let popular_movies = popular_movies.results;
    let most_popular_movie = popular_movies.get(0).unwrap();
    let (most_popular_movie, _) = create_signal(Some(most_popular_movie.to_owned().into()));
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
    let state: Arc<GlobalState> = use_context().unwrap();
    let win_size = state.main_tab_size;

    scroll(
        v_stack((
            media_hero_container(most_popular_movie),
            v_stack((
                label(move || "Popular Movies").class(CarouselTitle),
                movie_poster_carousel(popular_movies),
            ))
            .class(MediaCarousel)
            .style(move |s| s.width(win_size.get().width)),
            v_stack((
                label(move || "Popular TV shows").class(CarouselTitle),
                movie_poster_carousel(popular_tv_shows),
            ))
            .class(MediaCarousel)
            .style(move |s| s.width(win_size.get().width)),
        ))
        .style(|s| s.width_full()),
    )
    .style(|s| s.width_full())
}

style_class!(pub MediaCarousel);
style_class!(pub CarouselTitle);

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

pub fn movie_poster_carousel(movies: ReadSignal<im::Vector<PosterCarouselItem>>) -> impl View {
    let state: Arc<GlobalState> = use_context().unwrap();
    container(
        scroll(
            list(
                move || movies.get(),
                move |item| item.id(),
                move |item| poster_carousel_item(item),
            )
            .style(|s| s.gap(10.0, 0.).padding_bottom(15.)),
        )
        .style(move |s| s.width(state.main_tab_size.get().width)),
    )
    .style(|s| s.size(100.pct(), 100.pct()).padding_vert(20.0).flex_col())
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

pub fn stars_rating_progress_bar(rating: f64) -> impl View {
    let width = STAR_WIDTH * 5.;
    let height = STAR_HEIGHT;
    let max_rating = 10.;
    let clip_width = width * (rating / max_rating);
    let padding_top = 3.0;

    h_stack((
        five_stars(StarsKind::Unfilled).style(move |s| s.position(Position::Absolute)),
        clip(five_stars(StarsKind::Filled).style(move |s| s)).style(move |s| s.width(clip_width)),
    ))
    .style(move |s| {
        s.width(width)
            .height(height + padding_top)
            .padding_top(padding_top)
    })
}

///The aspect ratio is 1:1.5, e.g. for width 200(px), height is 300(px)
pub enum PosterImgSize {
    Width200,
    Width300,
}

pub fn dyn_poster_img(poster_path: String, poster_size: PosterImgSize) -> impl View {
    let img_bytes: RwSignal<Option<Result<Vec<u8>, String>>> = create_rw_signal(None);

    let (success_tx, success_rx) = crossbeam_channel::bounded(1);
    // The reactive runtime is thread-local, so we need to notify the runtime
    // when we are doing work in another thread
    let chan_bytes = create_signal_from_channel(success_rx);
    create_effect(move |_| {
        img_bytes.set(chan_bytes.get());
    });
    let state: Arc<GlobalState> = use_context().unwrap();

    std::thread::spawn(move || {
        let result = state
            .data_provider
            .get_poster_img(&poster_path)
            .map_err(|e| e.to_string());
        success_tx.send(result).unwrap();
    });

    let width = match poster_size {
        PosterImgSize::Width200 => 200.,
        PosterImgSize::Width300 => 300.,
    };
    let height = width * 1.5;

    dyn_container(
        move || img_bytes.get(),
        move |img_bytes| -> Box<dyn View> {
            match img_bytes {
                Some(resp) => match resp {
                    Ok(bytes) => Box::new(
                        img(move || bytes.to_vec()).style(|s| s.width_full().height_full()),
                    ),
                    Err(err_msg) => {
                        eprintln!("error: {err_msg}");
                        let card_img_err = include_str!("../../assets/alt_img.svg");
                        Box::new(svg(move || card_img_err.to_owned()).style(|s| {
                            s.size(180., 180.)
                                .margin_top(20.pct())
                                .color(SECONDARY_BG_COLOR)
                                .justify_self(Some(AlignItems::Center))
                        }))
                    }
                },
                None => Box::new(spinner()),
            }
        },
    )
    .style(move |s| {
        s.width(width)
            .height(height)
            .border(3.)
            .hover(|s| s.cursor(CursorStyle::Pointer))
            .border_color(Color::rgb8(37, 37, 38))
    })
}

pub fn poster_carousel_item(item: PosterCarouselItem) -> impl View {
    let vote_average = item.vote_average();
    let id = item.id();
    let state: Arc<GlobalState> = use_context().unwrap();
    let active_tab = state.active_tab;
    let poster = item.poster_path().unwrap();
    // let tab_state = state.tab_state;
    v_stack((
        dyn_poster_img(poster, PosterImgSize::Width200).on_click_stop(move |_| {
            active_tab.update(move |tab| {
                *tab = ActiveTabKind::Sub(SubTab::MovieDetails(MovieDetailsState { movie_id: id }));
            });
        }),
        v_stack((
            label(move || item.display_name().clone()),
            h_stack((
                stars_rating_progress_bar(vote_average),
                label(move || format!("{:.1}", vote_average))
                    .style(|s| s.margin_left(5.).padding_bottom(5.)),
            ))
            .style(|s| s.margin_top(5.).width_full()),
        ))
        .style(|s| {
            s.font_size(14.)
                .width(200.)
                .padding_bottom(10.)
                .padding_top(5.)
                .padding_horiz(1.)
            // .background(BG_COLOR_2)
        }),
    ))
}

pub fn hero_movie_img(media: MediaProduction) -> impl View {
    let img_bytes: RwSignal<Option<Result<Vec<u8>, String>>> = create_rw_signal(None);
    let (success_tx, success_rx) = crossbeam_channel::bounded(1);
    // The reactive runtime is thread-local, so we need to notify the runtime
    // when we are doing work in another thread
    let chan_bytes = create_signal_from_channel(success_rx);
    create_effect(move |_| {
        img_bytes.set(chan_bytes.get());
    });

    let backdrop_path = media.backdrop_path.unwrap();

    let state: Arc<GlobalState> = use_context().unwrap();
    std::thread::spawn(move || {
        let result = state
            .data_provider
            .get_backdrop_img(&backdrop_path)
            .map_err(|e| e.to_string());
        success_tx.send(result).unwrap();
    });

    dyn_container(
        move || img_bytes.get(),
        move |img_bytes| -> Box<dyn View> {
            match img_bytes {
                Some(resp) => match resp {
                    Ok(bytes) => Box::new(
                        img(move || bytes.to_vec()).style(|s| s.width_full().height_full()),
                    ),
                    Err(_) => {
                        let image_error = include_str!("../../assets/alt_img.svg");
                        Box::new(
                            svg(move || image_error.to_owned())
                                .style(|s| s.size(80., 80.).color(ACCENT_COLOR)),
                        )
                    }
                },
                None => Box::new(spinner()),
            }
        },
    )
    .style(|s| s.width_full().height_full())
}

#[derive(Clone)]
pub enum MediaKind {
    Movie,
    TvShow,
}

#[derive(Clone)]
pub struct MediaProduction {
    id: u64,
    kind: MediaKind,
    title: String,
    overview: Option<String>,
    poster_path: Option<String>,
    backdrop_path: Option<String>,
    vote_average: f64,
    vote_count: u64,
    // Some for movies, None for tv shows
    release_date: Option<String>,
}

impl From<Movie> for MediaProduction {
    fn from(movie: Movie) -> Self {
        Self {
            id: movie.id,
            kind: MediaKind::Movie,
            title: movie.title,
            overview: movie.overview,
            poster_path: movie.poster_path,
            backdrop_path: movie.backdrop_path,
            vote_average: movie.vote_average,
            vote_count: movie.vote_count,
            release_date: Some(movie.release_date),
        }
    }
}

impl From<TvShow> for MediaProduction {
    fn from(tv_show: TvShow) -> Self {
        Self {
            id: tv_show.id,
            kind: MediaKind::TvShow,
            title: tv_show.name,
            overview: tv_show.overview,
            poster_path: tv_show.poster_path,
            backdrop_path: tv_show.backdrop_path,
            vote_average: tv_show.vote_average,
            vote_count: tv_show.vote_count,
            release_date: None,
        }
    }
}

pub fn media_hero_container(movie: ReadSignal<Option<MediaProduction>>) -> impl View {
    let solid_bg_container_width = 30.pct();
    let bdrop_shadow_gradient_width = 45.pct();
    // the movie description goes over the solid gradient
    let movie_description_width = 70.pct();
    let backdrop_gradient_img = include_bytes!("../../assets/old_black_gradient3.png");
    let state: Arc<GlobalState> = use_context().unwrap();
    let win_size = state.main_tab_size;

    h_stack((
        empty().style(move |s| {
            s.position(Position::Absolute)
                .width(solid_bg_container_width)
                .height_full()
                .background(ACCENT_BG_COLOR)
        }),
        dyn_hero_media_img(movie).style(move |s| {
            s.width_full()
                .height_full()
                .margin_left(solid_bg_container_width)
        }),
        // The shadow that goes over the movie backdrop image, creating the linear gradient effect
        img(move || backdrop_gradient_img.to_vec()).style(move |s| {
            s.width(bdrop_shadow_gradient_width)
                .height_full()
                .margin_left(29.6.pct())
                .position(Position::Absolute)
                .border_color(Color::rgba(0., 0., 0., 0.1))
        }),
        dyn_movie_description(movie).style(move |s| {
            s.position(Position::Absolute)
                .width(movie_description_width)
                .height_full()
        }),
    ))
    .style(move |s| {
        s.width(win_size.get().width)
            .height(win_size.get().width / 2.5)
    })
}

fn dyn_hero_media_img(movie: ReadSignal<Option<MediaProduction>>) -> impl View {
    dyn_container(
        move || movie.get(),
        move |movie| -> Box<dyn View> {
            match movie {
                Some(movie) => Box::new(hero_movie_img(movie)),
                None => Box::new(spinner()),
            }
        },
    )
}

fn dyn_movie_description(movie: ReadSignal<Option<MediaProduction>>) -> impl View {
    dyn_container(
        move || movie.get(),
        move |movie| -> Box<dyn View> {
            match movie {
                Some(movie) => Box::new(movie_description(movie)),
                None => Box::new(spinner()),
            }
        },
    )
}

fn movie_description(movie: MediaProduction) -> impl View {
    let release_year = movie
        .release_date
        .map(|rd| rd.split('-').next().unwrap().to_owned())
        .unwrap_or("-".to_owned());
    v_stack((
        text(movie.title).style(|s| s.font_size(40.0).margin_vert(15.0)),
        h_stack((
            stars_rating_progress_bar(movie.vote_average),
            text(format!("{:.1}", movie.vote_average)).style(|s| s.margin_horiz(12.0)),
            text(format!(
                "{} Reviews",
                pretty_format_number(movie.vote_count)
            ))
            .style(|s| s.margin_right(12.0)),
            text(release_year.clone()).style(|s| s.margin_right(12.0)),
            text("1h 20m"),
        ))
        .style(|s| s.color(Color::rgb8(153, 153, 153))),
        text(movie.overview.unwrap_or_default()).style(|s| {
            s.color(PRIMARY_FG_COLOR)
                .width_pct(70.)
                .margin_top(20.0)
                .font_size(18.)
        }),
    ))
    .style(|s| s.width_full().padding(20.).justify_center().height_full())
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
