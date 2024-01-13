use std::sync::Arc;

use floem::{
    peniko::Color,
    reactive::{create_rw_signal, create_signal, use_context, ReadSignal, RwSignal},
    style::FlexWrap,
    unit::UnitExt,
    view::View,
    views::{
        container, h_stack, h_stack_from_iter, label, list, scroll, static_label, v_stack,
        v_stack_from_iter, virtual_stack, Decorators, Label, VirtualStackDirection,
        VirtualStackItemSize,
    },
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
#[derive(Clone, Deserialize, Serialize, Debug)]
struct Watchlist {
    items: im::Vector<WatchlistItem>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
struct WatchlistItem {
    list_order: u64,
    media_id: u64,
    note: Option<String>,
    added_on: String,
}

pub fn watchlist_view() -> impl View {
    let watchlist_json = include_str!("../../assets/data/watchlist.json");
    let watchlist: Watchlist =
        serde_json::from_str(watchlist_json).expect("Watchlist JSON was not well-formatted");
    dbg!(&watchlist);
    let items = create_rw_signal(watchlist.items);
    // let (images, _) = create_signal(
    //     person
    //         .get_untracked()
    //         .images
    //         .map(|img| Vector::from(img.profiles))
    //         .unwrap_or_default(),
    // );

    // label(||"x")
    tbody(items)
    // v_stack((thead(), tbody(items)))
}

fn tbody(items: RwSignal<im::Vector<WatchlistItem>>) -> impl View {
    virtual_stack(
        VirtualStackDirection::Vertical,
        VirtualStackItemSize::Fixed(Box::new(|| 45.0)),
        move || items.get(),
        move |item| item.media_id,
        move |item| {
            // h_stack((
                label(move || item.media_id.to_string())
            //     label(move || item.note.clone().unwrap_or_default()),
            //     label(move || item.added_on.clone()),
            // ))
        },
    )
}

fn thead() -> impl View {
    h_stack((
        static_label("Title"),
        static_label("Note"),
        static_label("Added on"),
    ))
}
