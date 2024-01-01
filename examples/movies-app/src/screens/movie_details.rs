use isolang::Language;
use std::{borrow::Cow, sync::Arc};

use floem::{
    ext_event::create_signal_from_channel,
    peniko::Color,
    reactive::{create_effect, create_rw_signal, create_signal, use_context, ReadSignal, RwSignal},
    style::{
        BorderBottom, BorderColor, BorderLeft, BorderRight, BorderTop, CursorStyle, JustifyContent,
        Position, Transition,
    },
    style_class,
    unit::UnitExt,
    view::View,
    views::{
        clip, container, container_box, dyn_container, empty, h_stack, img, label, list, scroll,
        static_label, svg, tab, text, v_stack, virtual_list, Decorators, Label,
        VirtualListDirection, VirtualListItemSize,
    },
};
use num_format::{Locale, ToFormattedString};

use crate::{
    models::{CastMember, Movie, MovieDetails, Page, TvShow},
    screens::home::PosterImgSize,
    spinner::spinner,
    ActiveTabKind, GlobalState, MainTab, MovieDetailsState, SubTab, ACCENT_BG_COLOR,
    DIMMED_ACCENT_COLOR, PRIMARY_BG_COLOR, PRIMARY_FG_COLOR, SECONDARY_BG_COLOR,
    SECONDARY_FG_COLOR,
};

use super::home::{
    dyn_poster_img, media_hero_container, movie_poster_carousel, stars_rating_progress_bar,
};

pub fn movie_details_screen(tab_state: MovieDetailsState) -> impl View {
    let movie_id = tab_state.movie_id;
    let movie_details: RwSignal<Option<Result<MovieDetails, String>>> = create_rw_signal(None);
    let (movie, set_movie) = create_signal(None);

    let (success_tx, success_rx) = crossbeam_channel::bounded(1);
    // The reactive runtime is thread-local, so we need to notify the runtime
    // when we finish doing work in another thread
    let res = create_signal_from_channel(success_rx);
    create_effect(move |_| {
        movie_details.set(res.get());

        if let Some(Ok(movie_details)) = res.get() {
            dbg!(movie_details.id);
            set_movie.update(|m| *m = Some(Movie::from(movie_details.clone()).into()));
        } else {
            println!("Error");
        }
    });
    let state: Arc<GlobalState> = use_context().unwrap();

    std::thread::spawn(move || {
        let result = state
            .data_provider
            .get_media_prod_details(movie_id)
            .map_err(|e| e.to_string());
        success_tx.send(result).unwrap();
    });

    scroll(
        v_stack((
            media_hero_container(movie),
            details_main_content(movie_details),
        ))
        .style(|s| s.width_full()),
    )
    .style(|s| s.width_full())
}

fn details_main_content(
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
    .style(|s| {
        s.justify_center()
            .flex_grow(1.)
            .gap(20, 0)
            .padding_vert(20.)
    });

    let tab = tab(
        move || selected.get(),
        move || tabs.get(),
        |it| *it,
        move |it| match it {
            "Overview" => {
                container_box(dyn_movie_det_overview(movie_details)).style(|s| s.width_full())
            }
            "Videos" => container_box(label(|| "Videos".to_owned())),
            "Photos" => container_box(label(|| "Photos".to_owned())),
            _ => container_box(label(|| "Not implemented".to_owned())),
        },
    )
    .style(|s| s.flex_row().items_start().width_full().flex_grow(1.));

    v_stack((tabs_nav_menu, tab))
}

fn dyn_movie_det_overview(
    movie_details: RwSignal<Option<Result<MovieDetails, String>>>,
) -> impl View {
    dyn_container(
        move || movie_details.get(),
        move |_| {
            let movie_details = movie_details.get();
            if let Some(movie_details) = movie_details {
                Box::new(container_box(movie_det_overview(movie_details)).style(|s| s.width_full()))
            } else {
                Box::new(container_box(static_label("Loading...")))
            }
        },
    )
    .style(|s| s.width_full())
}

style_class!(pub OverviewFieldName);
style_class!(pub OverviewFieldVal);

fn overview_row(left: impl View + 'static, right: impl View + 'static) -> impl View {
    h_stack((left, right))
        .style(|s| {
            s.width_full()
                .color(SECONDARY_FG_COLOR)
                .justify_between()
                .margin_top(15.)
                .gap(20, 0)
        })
        .class(OverviewFieldVal)
}

fn overview_field_container(name: Label, val: Label) -> impl View {
    h_stack((name, val))
        .style(|s| s.width(50.pct()))
        .class(OverviewFieldVal)
}

fn movie_det_overview(movie_details: Result<MovieDetails, String>) -> impl View {
    let (movie_details, _) = create_signal(movie_details.unwrap());
    // let poster_path = movie_details.poster_path;
    // let overview = movie_details
    //     .overview
    //     .expect("Overview should not be empty");
    // TODO: create effect
    let (cast, _) = create_signal(im::Vector::from(
        movie_details.get_untracked().credits.unwrap().cast,
    ));

    let state: Arc<GlobalState> = use_context().unwrap();
    let win_size = state.main_tab_size;

    v_stack((
        overview_main(movie_details.get()),
        v_stack((
            label(move || "Cast").style(|s| s.font_size(20.).margin_top(5.).padding(5.)),
            cast_carousel(cast),
        ))
        .style(move |s| s.padding(20.0).width(win_size.get().width)),
    ))
}

static CAST_MEMBER_CARD_WIDTH: f64 = 200.0;
fn cast_carousel(cast: ReadSignal<im::Vector<CastMember>>) -> impl View {
    let state: Arc<GlobalState> = use_context().unwrap();
    container(
        scroll(
            virtual_list(
                VirtualListDirection::Horizontal,
                VirtualListItemSize::Fixed(Box::new(|| CAST_MEMBER_CARD_WIDTH)),
                move || cast.get(),
                move |item| item.id,
                move |item| cast_actor_card(item),
            )
            .style(|s| s.gap(10.0, 0.).padding_bottom(15.)),
        )
        .style(move |s| s.width(state.main_tab_size.get().width)),
    )
    .style(|s| s.size(100.pct(), 100.pct()).padding_vert(20.0).flex_col())
}

fn cast_actor_card(cast: CastMember) -> impl View {
    let name = cast.name;
    let character = cast.character;

    let state: Arc<GlobalState> = use_context().unwrap();
    let active_tab = state.active_tab;
    let poster_path = cast.profile_path;
    v_stack((
        dyn_container(
            move || poster_path.clone(),
            move |poster_path| match poster_path {
                Some(poster_path) => Box::new(dyn_poster_img(poster_path, PosterImgSize::Width200)),
                None => {
                    let profile_icon = include_str!("../../assets/profile_icon.svg");
                    Box::new(svg(move || profile_icon.to_owned()).style(|s| {
                        s.size(100., 100.)
                            .color(SECONDARY_BG_COLOR)
                            .margin_top(25.pct())
                    }))
                }
            },
        )
        .on_click_stop(move |_| {
            active_tab.update(move |tab| {
                *tab = ActiveTabKind::Sub(SubTab::PersonProfile(crate::PersonProfileState {
                    person_id: cast.id,
                }));
            });
        })
        .style(|s| {
            s.width_full()
                .height_full()
                .hover(|s| s.cursor(CursorStyle::Pointer))
                .justify_center()
        }),
        v_stack((
            label(move || name.clone()),
            label(move || format!("as {}", character.clone()))
                .style(|s| s.color(SECONDARY_FG_COLOR)),
        ))
        .style(|s| {
            s.font_size(14.)
                .width(CAST_MEMBER_CARD_WIDTH)
                .height(75.0)
                .padding_bottom(10.)
                .padding_top(5.)
                .padding_horiz(1.)
            // .background(BG_COLOR_2)
        }),
    ))
}

fn overview_main(movie_details: MovieDetails) -> impl View {
    let field_name = |text| static_label(text).class(OverviewFieldName);
    let overview = movie_details
        .overview
        .expect("Overview should not be empty");
    let poster_path = movie_details.poster_path;

    h_stack((
        dyn_poster_img(poster_path.unwrap(), PosterImgSize::Width300),
        v_stack((
            static_label("Storyline").style(|s| s.font_size(26.).margin_bottom(20)),
            label(move || overview.clone()).style(|s| s.font_size(14.).margin_bottom(15.)),
            overview_row(
                overview_field_container(
                    field_name("Released"),
                    label(move || movie_details.release_date.clone()),
                ),
                overview_field_container(
                    field_name("Runtime"),
                    label(move || pretty_format_runtime(movie_details.runtime.clone())),
                ),
            ),
            overview_row(
                overview_field_container(field_name("Director"), label(move || "Sample Director")),
                overview_field_container(
                    field_name("Budget"),
                    label(move || {
                        format!("${}", movie_details.budget.to_formatted_string(&Locale::en))
                    }),
                ),
            ),
            overview_row(
                overview_field_container(
                    field_name("Genre"),
                    label(move || {
                        movie_details
                            .genres
                            .clone()
                            .as_slice()
                            .iter()
                            .map(|g| g.name.clone())
                            .take(2)
                            .collect::<Vec<String>>()
                            .join(", ")
                    }),
                ),
                overview_field_container(field_name("Status"), label(move || "Released")),
            ),
            overview_row(
                overview_field_container(
                    field_name("Language"),
                    label(move || {
                        Language::from_639_1(&movie_details.original_language)
                            .expect("language should be in ISO 639-1 format")
                            .to_name()
                    }),
                ),
                overview_field_container(
                    field_name("Production"),
                    label(move || {
                        movie_details
                            .production_companies
                            .iter()
                            .map(|c| c.name.clone())
                            .collect::<Vec<String>>()
                            .join(", ")
                    }),
                ),
            ),
        ))
        .style(|s| {
            s.font_size(12.)
                .width_pct(70.)
                .max_width(600)
                .margin_left(35.)
                .padding_bottom(10.)
                .padding_top(50.)
                .padding_horiz(1.)
                .class(OverviewFieldName, |s| s.width(75))
            // .class(OverviewFieldVal, |s| s.width(50.pct()))
            // .class(OverviewFieldVal, |s| s.width(500))
        }),
    ))
    .style(|s| s.width_full().justify_center())
}

fn pretty_format_runtime(minutes: u32) -> String {
    let hours = minutes / 60;
    let minutes = minutes % 60;
    if hours > 0 {
        format!("{}h {}min", hours, minutes)
    } else {
        format!("{}min", minutes)
    }
}
