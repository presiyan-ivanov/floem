use floem::{
    peniko::Color,
    reactive::{create_signal, ReadSignal},
    style::Position,
    view::View,
    views::{container, empty, h_stack, img, label, stack, static_label, svg, v_stack, Decorators},
};

use crate::models::{Movie, Page};

pub fn home_view() -> impl View {
    let trending = include_str!("../../assets/data/popular_movies.json");
    let popular_movies: Page<Movie> =
        serde_json::from_str(trending).expect("JSON was not well-formatted");
    let popular_movies = popular_movies.results;
    let most_popular_movie = popular_movies.first().unwrap();
    let (most_popular_movie, _) = create_signal(most_popular_movie.to_owned());

    movie_hero_container(most_popular_movie)
}

pub fn dyn_img() -> impl View {
    let poster = include_bytes!("../../assets/poster.jpg");

    img(move || poster.to_vec()).style(|s| s.width(1280.).height(720.))
}

pub fn movie_hero_container(movie: ReadSignal<Movie>) -> impl View {
    let release_year =
        movie.with_untracked(|m| m.release_date.split('-').next().unwrap().to_owned());
    let movie_details_width = 700.0;
    let bg_container_width = 300.0;
    let backdrop_gradient = include_str!("../../assets/backdrop_gradient.svg");
    h_stack((
        dyn_img().style(move |s| s.margin_left(bg_container_width).height_full()),
        empty().style(move |s| {
            s.position(Position::Absolute)
                .width(bg_container_width)
                .height_full()
                .background(Color::BLACK)
        }),
        v_stack((
            label(move || movie.get().title).style(|s| s.font_size(26.0).margin_vert(15.0)),
            h_stack((
                label(move || format!("Rating: 3.5/5")).style(|s| s.margin_right(10.0)),
                label(move || format!("{} Reviews", movie.get().vote_count))
                    .style(|s| s.margin_right(10.0)),
                label(move || release_year.clone()).style(|s| s.margin_right(10.0)),
                label(move || "1h 20m"),
            ))
            .style(|s| s.color(Color::rgb8(153, 153, 153))),
            label(move || movie.get().overview.unwrap_or_default())
                .style(|s| s.color(Color::WHITE).width_pct(70.).margin_top(20.0)),
        ))
        .style(move |s| {
            s.position(Position::Absolute)
                .padding(20.)
                .width(movie_details_width)
                .justify_center()
                .height_full()
        }),
        svg(move || backdrop_gradient.to_string()).style(move |s| {
            s.width(400.)
                .height_full()
                .margin_left(movie_details_width)
                .position(Position::Absolute)
        }),
    ))
}
