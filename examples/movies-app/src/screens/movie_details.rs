use std::sync::Arc;

use anyhow::Error;
use floem::{
    ext_event::create_signal_from_channel,
    peniko::Color,
    reactive::{create_effect, create_rw_signal, create_signal, use_context, ReadSignal, RwSignal},
    style::{
        BorderBottom, BorderColor, BorderLeft, BorderRight, BorderTop, CursorStyle, Position,
        Transition,
    },
    unit::UnitExt,
    view::View,
    views::{
        clip, container, dyn_container, empty, h_stack, img, label, list, scroll, svg, v_stack,
        Decorators,
    },
};

use crate::{
    models::{Movie, MovieDetails, Page, TvShow},
    spinner::spinner,
    GlobalState, MainTab, MovieDetailsState, DIMMED_ACCENT_COLOR, NEUTRAL_BG_COLOR,
    PRIMARY_FG_COLOR,
};

use super::home::movie_hero_container;

pub fn movie_details_screen(tab_state: MovieDetailsState) -> impl View {
    let state = use_context::<Arc<GlobalState>>().unwrap();
    let movie_id = tab_state.movie_id;
    let movie_details: RwSignal<Option<Result<MovieDetails, String>>> = create_rw_signal(None);

    let (success_tx, success_rx) = crossbeam_channel::bounded(1);
    // The reactive runtime is thread-local, so we need to notify the runtime
    // when we are doing work in another thread
    let res = create_signal_from_channel(success_rx);
    create_effect(move |_| {
        movie_details.set(res.get());
    });
    let state: Arc<GlobalState> = use_context().unwrap();
    // let poster = item.poster_path().unwrap();

    std::thread::spawn(move || {
        let result = state
            .data_provider
            .get_movie_details(movie_id)
            .map_err(|e| e.to_string());
        success_tx.send(result).unwrap();
    });

    let (most_popular_movie, set_m) =
        create_signal(movie_details.get_untracked().map(|md| md.unwrap().into()));

    scroll(
        v_stack((
            movie_hero_container(most_popular_movie),
            // v_stack((
            //     label(move || "Popular Movies")
            //         .style(|s| s.font_size(20.).margin_top(10.).padding(5.)),
            //     carousel(popular_movies),
            // ))
            // .style(move |s| s.padding(20.0).width(win_size.get().width)),
            // v_stack((
            //     label(move || "Popular TV shows")
            //         .style(|s| s.font_size(20.).margin_top(10.).padding(5.)),
            //     carousel(popular_tv_shows),
            // ))
            // .style(move |s| s.padding(20.0).width(win_size.get().width)),
        ))
        .style(|s| s.width_full()),
    )
    .style(|s| s.width_full())
}
