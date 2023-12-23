use floem::{
    peniko::Color,
    reactive::{create_signal, ReadSignal},
    style::Position,
    unit::UnitExt,
    view::View,
    views::{
        clip, container, empty, h_stack, img, label, list, scroll, stack, static_label,
        static_list, svg, v_stack, virtual_list, Decorators, VirtualListDirection,
        VirtualListItemSize,
    },
};

use crate::{
    models::{Movie, Page},
    ACCENT_COLOR, NEUTRAL_BG_COLOR, PRIMARY_FG_COLOR,
};

pub fn home_view() -> impl View {
    let trending = include_str!("../../assets/data/popular_movies.json");
    let popular_movies: Page<Movie> =
        serde_json::from_str(trending).expect("JSON was not well-formatted");
    let popular_movies = popular_movies.results;
    let most_popular_movie = popular_movies.get(1).unwrap();
    let (most_popular_movie, _) = create_signal(most_popular_movie.to_owned());
    let (popular_movies, _) = create_signal(popular_movies.take(7));

    scroll(v_stack((
        movie_hero_container(most_popular_movie),
        v_stack((
            label(move || "Popular Movies").style(|s| s.font_size(20.).margin_top(10.).padding(5.)),
            // carousel(popular_movies),
        ))
        .style(|s| s.padding(20.0).width_full()),
    )))
}

pub fn carousel(movies: ReadSignal<im::Vector<Movie>>) -> impl View {
    container(
        scroll(
            virtual_list(
                VirtualListDirection::Horizontal,
                VirtualListItemSize::Fixed(Box::new(|| 32.0)),
                move || movies.get(),
                move |item| item.id,
                move |item| movie_card(item),
            )
            .style(|s| s.gap(10.0, 0.)),
        )
        .style(|s| s.width_full()),
    )
    .style(|s| {
        s.size(100.pct(), 100.pct())
            .padding_vert(20.0)
            .flex_col()
            .items_center()
    })
}

pub enum StarsKind {
    Filled,
    Unfilled,
}

static STAR_WIDTH: f64 = 18.;
static STAR_HEIGHT: f64 = 16.;

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

pub fn movie_card(movie: Movie) -> impl View {
    let url = reqwest::Url::parse(&format!(
        "https://image.tmdb.org/t/p/w500{}",
        movie.poster_path.unwrap()
    ))
    .unwrap();
    // let client = reqwest::Client::new();
    // let response = client.get(url).send().unwrap();
    // let response = client.get(url).send().await.unwrap();
    // let resource = Resource

    // let adapter = futures::executor::block_on()
    //     .ok_or_else(|| anyhow::anyhow!("can't get adapter"))?;
    //
    let res = reqwest::blocking::get(url.clone()).unwrap();
    eprintln!("url: {}, status: {}", &url, res.status());
    let poster = res.bytes().unwrap();

    v_stack((
        img(move || poster.to_vec()).style(|s| {
            s.width(200.)
                .height(300.)
                .border(3.)
                .border_color(Color::rgba(156., 163., 175., 0.1))
        }),
        v_stack((
            label(move || movie.title.clone()),
            h_stack((
                stars_rating_bar(movie.vote_average),
                label(move || format!("{:.1}", movie.vote_average)).style(|s| s.margin_left(5.)),
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
    let res = reqwest::blocking::get(url.clone()).unwrap();
    eprintln!("url: {}, status: {}", &url, res.status());
    let poster = res.bytes().unwrap();

    img(move || poster.to_vec())
}

pub fn movie_hero_container(movie: ReadSignal<Movie>) -> impl View {
    let release_year =
        movie.with_untracked(|m| m.release_date.split('-').next().unwrap().to_owned());
    let movie_details_width = 900.0;
    let bg_container_width = 450.0;
    let backdrop_gradient = include_bytes!("../../assets/black_gradient3.png");

    h_stack((
        empty().style(move |s| {
            s.position(Position::Absolute)
                .width(bg_container_width)
                .height_full()
                .background(NEUTRAL_BG_COLOR)
        }),
        movie_img(movie).style(move |s| {
            s.width(1700.)
                .margin_left(bg_container_width)
                .max_height(800.px())
        }),
        img(move || backdrop_gradient.to_vec()).style(move |s| {
            s.width(600.)
                .height(1000.)
                .margin_left(bg_container_width)
                .position(Position::Absolute)
        }),
        v_stack((
            label(move || movie.get().title).style(|s| s.font_size(32.0).margin_vert(15.0)),
            h_stack((
                stars_rating_bar(movie.get().vote_average),
                label(move || format!("{:.1}", movie.get().vote_average))
                    .style(|s| s.margin_horiz(12.0)),
                label(move || format!("{} Reviews", movie.get().vote_count))
                    .style(|s| s.margin_right(12.0)),
                label(move || release_year.clone()).style(|s| s.margin_right(12.0)),
                label(move || "1h 20m"),
            ))
            .style(|s| s.color(Color::rgb8(153, 153, 153))),
            label(move || movie.get().overview.unwrap_or_default())
                .style(|s| s.color(PRIMARY_FG_COLOR).width_pct(70.).margin_top(20.0)),
        ))
        .style(move |s| {
            s.position(Position::Absolute)
                .padding(20.)
                .width(movie_details_width)
                .justify_center()
                .height_full()
        }),
    ))
}
