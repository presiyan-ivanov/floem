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
    GlobalState, MovieDetailsState, Tab, DIMMED_ACCENT_COLOR, NEUTRAL_BG_COLOR, PRIMARY_FG_COLOR,
};

use super::home::movie_hero_container;

pub fn movie_details_screen() -> impl View {
    let state = use_context::<Arc<GlobalState>>().unwrap();
    dbg!(state.active_tab.get());
    let movie_id = state.active_tab.get().movie_id().unwrap();
    let movie_details = state.data_provider.get_movie_details(movie_id).unwrap();
    let (most_popular_movie, set_m) = create_signal(movie_details.into());

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
    // .on_resize(move |rect| {
    //     println!("size: {:?}", rect.size());
    //     set_available_width.update(move |width| *width = rect.width());
    // })
    .style(|s| s.width_full())
}
