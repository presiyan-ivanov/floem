use std::sync::Arc;

use floem::{
    peniko::Color,
    reactive::{create_signal, use_context, ReadSignal},
    unit::UnitExt,
    view::View,
    views::{container, h_stack, label, list, scroll, static_label, v_stack, Decorators, Label},
};
use im::Vector;
use serde::{Deserialize, Serialize};

use crate::{GlobalState, PersonDetailsState, PRIMARY_FG_COLOR, SECONDARY_FG_COLOR};

use super::{
    home::{
        dyn_poster_img, poster_carousel_item, CarouselTitle, PosterCarouselItem, PosterImgSize,
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
    images: Option<PersonImages>,
}

#[derive(Clone, Deserialize, Serialize)]
struct PersonImages {
    profiles: Vec<ProfileImg>,
}

#[derive(Clone, Deserialize, Serialize)]
struct ProfileImg {
    aspect_ratio: f64,
    height: u64,
    file_path: String,
    width: u64,
}

pub fn person_details(pd_state: PersonDetailsState) -> impl View {
    let pers_details_json = include_str!("../../assets/data/person_details/117642.json");
    let person: PersonDetails = serde_json::from_str(pers_details_json)
        .expect("Person details JSON was not well-formatted");
    let (person, _) = create_signal(person);
    let (images, _) = create_signal(
        person
            .get_untracked()
            .images
            .map(|img| Vector::from(img.profiles))
            .unwrap_or_default(),
    );

    v_stack((
        overview(person),
        v_stack((
            label(move || "Photos").class(CarouselTitle),
            person_images_carousel(images),
        )),
    ))
    .style(|s| {
        s.width_full()
            .justify_center()
            .padding_top(30)
            .min_height(300)
            .gap(35, 0)
            .color(Color::rgb8(229, 231, 235))
    })
}

pub fn person_images_carousel(profile_imgs: ReadSignal<im::Vector<ProfileImg>>) -> impl View {
    let state: Arc<GlobalState> = use_context().unwrap();
    container(
        scroll(
            list(
                move || profile_imgs.get(),
                move |item| item.file_path.clone(),
                move |item| dyn_poster_img(item.file_path, PosterImgSize::Width200),
            )
            .style(|s| s.gap(10.0, 0.).padding_bottom(15.)),
        )
        .style(move |s| s.width(state.main_tab_size.get().width)),
    )
    .style(|s| s.size(100.pct(), 100.pct()).padding_vert(20.0).flex_col())
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
        .style(|s| s.max_width(700.px()).margin_left(40)),
    ))
}
