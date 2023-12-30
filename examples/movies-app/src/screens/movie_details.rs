use std::sync::Arc;

use anyhow::Error;
use floem::{
    ext_event::create_signal_from_channel,
    peniko::Color,
    reactive::{create_effect, create_rw_signal, create_signal, use_context, ReadSignal, RwSignal},
    style::{
        BorderBottom, BorderColor, BorderLeft, BorderRight, BorderTop, CursorStyle, JustifyContent,
        Position, Transition,
    },
    unit::UnitExt,
    view::View,
    views::{
        clip, container, container_box, dyn_container, empty, h_stack, img, label, list, scroll,
        static_label, svg, tab, v_stack, Decorators,
    },
};

use crate::{
    models::{Movie, MovieDetails, Page, TvShow},
    spinner::spinner,
    GlobalState, MainTab, MovieDetailsState, BG_COLOR_2, DIMMED_ACCENT_COLOR, NEUTRAL_BG_COLOR,
    PRIMARY_FG_COLOR, SECONDARY_BG_COLOR,
};

use super::home::movie_hero_container;

pub fn movie_details_screen(tab_state: MovieDetailsState) -> impl View {
    // let state = use_context::<Arc<GlobalState>>().unwrap();
    let movie_id = tab_state.movie_id;
    let movie_details: RwSignal<Option<Result<MovieDetails, String>>> = create_rw_signal(None);
    let (movie, set_movie) = create_signal(None);

    let (success_tx, success_rx) = crossbeam_channel::bounded(1);
    // The reactive runtime is thread-local, so we need to notify the runtime
    // when we are doing work in another thread
    let res = create_signal_from_channel(success_rx);
    create_effect(move |_| {
        movie_details.set(res.get());

        if let Some(Ok(movie_details)) = res.get() {
            dbg!(movie_details.id);
            set_movie.update(|m| *m = Some(Movie::from(movie_details.clone())));
        } else {
            println!("Error");
        }
    });
    let state: Arc<GlobalState> = use_context().unwrap();

    std::thread::spawn(move || {
        let result = state
            .data_provider
            .get_movie_details(movie_id)
            .map_err(|e| e.to_string());
        dbg!(&result);
        success_tx.send(result).unwrap();
    });

    scroll(
        v_stack((
            movie_hero_container(movie),
            movie_details_main_content(movie_details),
            // movie_cast()
        ))
        .style(|s| s.width_full()),
    )
    .style(|s| s.width_full())
}

fn movie_details_main_content(
    movie_details: RwSignal<Option<Result<MovieDetails, String>>>,
) -> impl View {
    let tabs: im::Vector<&str> = vec!["Overview", "Videos", "Photos"].into_iter().collect();
    let (tabs, _set_tabs) = create_signal(tabs);
    let selected: RwSignal<usize> = create_rw_signal(0);

    let tab_item = move |name: &str, index: usize| {
        static_label(name.to_owned().to_uppercase())
            .style(move |s| {
                s.color(Color::rgb8(65, 65, 65))
                    .border_bottom(2.0)
                    .border_color(Color::TRANSPARENT)
                    .font_size(16.)
                    .cursor(CursorStyle::Pointer)
                    .padding(5.0)
                    .apply_if(selected.get() == index, |s| {
                        s.color(PRIMARY_FG_COLOR).border_color(PRIMARY_FG_COLOR)
                    })
            })
            .on_click_stop(move |_| selected.set(index))
    };

    let tabs_nav_menu = h_stack((
        tab_item("Overview", 0),
        tab_item("Videos", 1),
        tab_item("Photos", 2),
    ))
    .keyboard_navigatable()
    .style(|s| s.justify_center().flex_grow(1.).gap(20, 0).padding_vert(20.));

    let tab = tab(
        move || selected.get(),
        move || tabs.get(),
        |it| *it,
        move |it| match it {
            "Overview" => container_box(label(|| "Overview".to_owned())),
            "Videos" => container_box(label(|| "Videos".to_owned())),
            "Photos" => container_box(label(|| "Photos".to_owned())),
            _ => container_box(label(|| "Not implemented".to_owned())),
        },
    )
    .style(|s| s.flex_row().items_start().width_full().flex_grow(1.));

    v_stack((tabs_nav_menu, tab))
}
