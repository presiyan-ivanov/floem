use std::sync::Arc;

use floem::{
    reactive::{create_signal, use_context},
    view::View,
    views::{label, scroll, v_stack, Decorators},
};

use crate::{
    models::{Movie, Page, TvShow},
    GlobalState,
};

use super::home::{
    movie_hero_container, movie_poster_carousel, CarouselTitle, MediaCarousel, PosterCarouselItem,
};

pub fn movies_view() -> impl View {
    let top_rated = include_str!("../../assets/data/top_rated_movies.json");
    let popular = include_str!("../../assets/data/popular_movies.json");
    let now_playing = include_str!("../../assets/data/now_playing_movies.json");

    let popular_movies: Page<Movie> =
        serde_json::from_str(popular).expect("JSON was not well-formatted");

    let top_rated: Page<Movie> =
        serde_json::from_str(top_rated).expect("JSON was not well-formatted");

    let now_playing: Page<Movie> =
        serde_json::from_str(now_playing).expect("JSON was not well-formatted");

    let popular_movies = popular_movies.results;
    let most_popular_movie = popular_movies.get(0).unwrap();
    let (most_popular_movie, _) = create_signal(Some(most_popular_movie.to_owned()));
    let (popular_movies, _) = create_signal(
        popular_movies
            .into_iter()
            .map(|m| PosterCarouselItem::Movie(m).to_owned())
            .take(12)
            .collect(),
    );

    let (top_rated, _) = create_signal(
        top_rated
            .results
            .into_iter()
            .map(|m| PosterCarouselItem::Movie(m).to_owned())
            .take(12)
            .collect(),
    );

    let (now_playing, _) = create_signal(
        now_playing
            .results
            .into_iter()
            .map(|m| PosterCarouselItem::Movie(m).to_owned())
            .take(12)
            .collect(),
    );

    let state: Arc<GlobalState> = use_context().unwrap();
    let win_size = state.main_tab_size;
    scroll(
        v_stack((
            movie_hero_container(most_popular_movie),
            v_stack((
                label(move || "Popular Movies").class(CarouselTitle),
                movie_poster_carousel(popular_movies),
            ))
            .class(MediaCarousel)
            .style(move |s| s.width(win_size.get().width)),
            v_stack((
                label(move || "Top Rated Movies").class(CarouselTitle),
                movie_poster_carousel(top_rated),
            ))
            .class(MediaCarousel)
            .style(move |s| s.width(win_size.get().width)),
            v_stack((
                label(move || "Now playing").class(CarouselTitle),
                movie_poster_carousel(now_playing),
            ))
            .class(MediaCarousel)
            .style(move |s| s.width(win_size.get().width)),
        ))
        .style(|s| s.width_full()),
    )
    .style(|s| s.width_full())
}
