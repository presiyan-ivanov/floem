use floem::{
    peniko::Color,
    reactive::{create_signal, ReadSignal},
    style::Position,
    unit::UnitExt,
    view::View,
    views::{
        clip, container, empty, h_stack, img, label, scroll, stack, static_label, svg, v_stack,
        virtual_list, Decorators, VirtualListDirection, VirtualListItemSize,
    },
};

use crate::models::{Movie, Page};

pub fn home_view() -> impl View {
    let trending = include_str!("../../assets/data/popular_movies.json");
    let popular_movies: Page<Movie> =
        serde_json::from_str(trending).expect("JSON was not well-formatted");
    let popular_movies = popular_movies.results;
    let most_popular_movie = popular_movies.get(0).unwrap();
    let (most_popular_movie, _) = create_signal(most_popular_movie.to_owned());
    let (popular_movies, _) = create_signal(popular_movies);

    scroll(v_stack((
        movie_hero_container(most_popular_movie),
        v_stack((
            label(move || "Popular Movies").style(|s| s.font_size(20.).margin_top(10.).padding(5.)),
            carousel(popular_movies),
        ))
        .style(|s| s.padding(20.0)),
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

pub fn stars_rating(rating: f64) -> impl View {
    debug_assert!(rating >= 0. && rating <= 10.);
    let filled_stars = include_bytes!("../../assets/filled_stars.png");
    let stars = include_bytes!("../../assets/stars.png");
    let width = 188. / 2.5;
    let height = 36. / 2.5;
    let clip_width = width * (rating / 10.);

    h_stack((
        img(move || stars.to_vec())
            .style(move |s| s.width(width).height(height).position(Position::Absolute)),
        clip(img(move || filled_stars.to_vec()).style(move |s| s.width(width).height(height)))
            .style(move |s| s.width(clip_width)),
    ))
    .style(move |s| s.width(width).height(height))
}

pub fn movie_card(movie: Movie) -> impl View {
    let poster = include_bytes!("../../assets/poster.jpg");

    v_stack((
        img(move || poster.to_vec()).style(|s| {
            s.width(200.)
                .height(320.)
                .border(3.)
                .border_color(Color::rgba(156., 163., 175., 0.1))
        }),
        v_stack((
            label(move || movie.title.clone()),
            h_stack((
                stars_rating(movie.vote_average),
                label(move || format!("{:.1}", movie.vote_average))
                    .style(|s| s.margin_left(5.),
                ),
            )),
        ))
        .style(|s| s.font_size(14.).width(200.)),
    ))
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
    // let backdrop_gradient = include_str!("../../assets/backdrop_gradient.svg");
    h_stack((
        dyn_img().style(move |s| s.margin_left(bg_container_width).height_full()),
        empty().style(move |s| {
            s.position(Position::Absolute)
                .width(bg_container_width)
                .height_full()
                .background(Color::BLACK)
        }),
        v_stack((
            label(move || movie.get().title).style(|s| s.font_size(28.0).margin_vert(15.0)),
            h_stack((
                stars_rating(movie.get().vote_average),
                label(move || format!("{:.1}", movie.get().vote_average))
                    .style(|s| s.margin_horiz(12.0)),
                label(move || format!("{} Reviews", movie.get().vote_count))
                    .style(|s| s.margin_right(12.0)),
                label(move || release_year.clone()).style(|s| s.margin_right(12.0)),
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
        // svg(move || backdrop_gradient.to_string()).style(move |s| {
        //     s.width(400.)
        //         .height_full()
        //         .margin_left(movie_details_width)
        //         .position(Position::Absolute)
        // }),
    ))
}
