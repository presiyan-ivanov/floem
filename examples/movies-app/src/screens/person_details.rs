use std::sync::Arc;

use floem::{
    peniko::Color,
    reactive::{create_rw_signal, create_signal, use_context, ReadSignal},
    unit::UnitExt,
    view::View,
    views::{container, h_stack, label, scroll, static_label, v_stack, Decorators, Label},
};
use serde::{Deserialize, Serialize};

use crate::{
    models::{Movie, Page, TvShow},
    GlobalState, PersonDetailsState, PRIMARY_FG_COLOR, SECONDARY_FG_COLOR,
};

use super::{
    home::{
        media_hero_container, movie_poster_carousel, CarouselTitle, MediaCarousel,
        PosterCarouselItem,
    },
    movie_details::dyn_actor_img,
};

#[derive(Clone, Deserialize, Serialize)]
struct PersonDetails {
    id: u64,
    name: String,
    biography: Option<String>,
    profile_path: Option<String>,
    place_of_birth: Option<String>,
    birthday: Option<String>,
    known_for_department: Option<String>,
}

struct PersonImg {

}

pub fn person_details(pd_state: PersonDetailsState) -> impl View {
    let pers_details_json = include_str!("../../assets/data/person_details/117642.json");
    let person: PersonDetails = serde_json::from_str(pers_details_json)
        .expect("Person details JSON was not well-formatted");
    let (person, _)= create_signal(person);

    scroll(
        v_stack((
            overview(person),
            v_stack((
                label(move || "Photos").class(CarouselTitle),
                static_label("xx")
            )),
        ))
        .style(|s| {
            s.width_full()
                .justify_center()
                .padding_top(30)
                .min_height(300)
                .gap(35, 0)
                .color(Color::rgb8(229, 231, 235))
        }),
    )
    .style(|s| s.width(1700))
}

pub fn overview(person_details: ReadSignal<PersonDetails>) -> impl View {
    let name_val_row = |name: Label, value: Label| {
        h_stack((
            name.style(|s| s.width(100.).color(SECONDARY_FG_COLOR)),
            value,
        ))
        .style(|s| s.margin_bottom(10.))
    };
    h_stack((
        container(dyn_actor_img(person_details.get().profile_path))
            .style(|s| s.height_full().items_start()),
        v_stack((
            label(move || person_details.get().name)
                .style(|s| s.font_size(26.0).color(PRIMARY_FG_COLOR).margin_bottom(20)),
            label(move || person_details.get().biography.unwrap_or_default())
                .style(|s| s.font_size(15.0).margin_bottom(5)),
            name_val_row(
                static_label("Known for"),
                label(move || {
                    person_details
                        .get()
                        .known_for_department
                        .unwrap_or_default()
                }),
            ),
            name_val_row(
                static_label("Place of birth"),
                label(move || person_details.get().place_of_birth.unwrap_or_default()),
            ),
            name_val_row(
                static_label("Birthday"),
                label(move || person_details.get().birthday.unwrap_or_default()),
            ),
        ))
        .style(|s| s.max_width(700.px())),
    ))
}
