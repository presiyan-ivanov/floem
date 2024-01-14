use std::sync::Arc;

use floem::{
    peniko::Color,
    reactive::{create_rw_signal, create_signal, use_context, ReadSignal, RwSignal},
    style::{Display, FlexWrap, Position},
    style_class,
    taffy::{
        geometry::MinMax,
        style::{MaxTrackSizingFunction, MinTrackSizingFunction, TrackSizingFunction},
    },
    unit::UnitExt,
    view::View,
    views::{
        container, h_stack, h_stack_from_iter, label, list, scroll, static_label, v_stack,
        v_stack_from_iter, virtual_list, virtual_stack, Decorators, Label, VirtualDirection,
        VirtualItemSize,
    },
};
use im::Vector;
use serde::{Deserialize, Serialize};

use crate::{
    GlobalState, PersonDetailsState, DIMMED_ACCENT_COLOR, NAVBAR_BG_COLOR, PRIMARY_BG_COLOR,
    PRIMARY_FG_COLOR, SECONDARY_BG_COLOR, SECONDARY_FG_COLOR,
};

#[derive(Clone, Deserialize, Serialize, Debug)]
struct Watchlist {
    items: im::Vector<WatchlistItem>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
struct WatchlistItem {
    list_order: u64,
    media_id: u64,
    media_kind: String,
    note: Option<String>,
    added_on: String,
}
style_class!(TableRow);

pub fn watchlist_view() -> impl View {
    let watchlist_json = include_str!("../../assets/data/watchlist.json");
    let watchlist: Watchlist =
        serde_json::from_str(watchlist_json).expect("Watchlist JSON was not well-formatted");
    dbg!(&watchlist);
    let items = create_rw_signal(watchlist.items);
    // let (images, _) = create_signal(
    //     person
    //         .get_untracked()
    //         .images .map(|img| Vector::from(img.profiles))
    //         .unwrap_or_default(),
    // );
    let app_state = use_context::<Arc<GlobalState>>().unwrap();

    let table_cols_size = create_rw_signal(vec![
        TrackSizingFunction::Single(MinMax {
            min: MinTrackSizingFunction::Fixed(floem::taffy::style::LengthPercentage::Points(30.)),
            max: MaxTrackSizingFunction::Fixed(floem::taffy::style::LengthPercentage::Points(50.)),
        }),
        TrackSizingFunction::Single(MinMax {
            min: MinTrackSizingFunction::Fixed(floem::taffy::style::LengthPercentage::Points(150.)),
            max: MaxTrackSizingFunction::Fixed(floem::taffy::style::LengthPercentage::Points(350.)),
        }),
        TrackSizingFunction::Single(MinMax {
            min: MinTrackSizingFunction::Fixed(floem::taffy::style::LengthPercentage::Points(50.)),
            max: MaxTrackSizingFunction::Fixed(floem::taffy::style::LengthPercentage::Points(100.)),
        }),
        TrackSizingFunction::Single(MinMax {
            min: MinTrackSizingFunction::Fixed(floem::taffy::style::LengthPercentage::Points(200.)),
            max: MaxTrackSizingFunction::Fixed(floem::taffy::style::LengthPercentage::Points(300.)),
        }),
        TrackSizingFunction::Single(MinMax {
            min: MinTrackSizingFunction::Fixed(floem::taffy::style::LengthPercentage::Points(200.)),
            max: MaxTrackSizingFunction::Fixed(floem::taffy::style::LengthPercentage::Points(400.)),
        }),
        TrackSizingFunction::Single(MinMax {
            min: MinTrackSizingFunction::Fixed(floem::taffy::style::LengthPercentage::Points(80.)),
            max: MaxTrackSizingFunction::Fixed(floem::taffy::style::LengthPercentage::Points(100.)),
        }),
        TrackSizingFunction::Single(MinMax {
            min: MinTrackSizingFunction::Fixed(floem::taffy::style::LengthPercentage::Points(50.)),
            max: MaxTrackSizingFunction::Fixed(floem::taffy::style::LengthPercentage::Points(100.)),
        }),
    ]);

    let margin_horiz = 20.;
    container(scroll(v_stack((thead(), tbody(items)))).style(move |s| {
        s.width(app_state.window_size.get().width - margin_horiz - 100.)
            .margin_horiz(margin_horiz)
            .padding_bottom(10)
            .margin_top(30)
            .class(TableRow, move |s| {
                s.display(Display::Grid)
                    .height(50.)
                    .padding_left(10.)
                    .padding_bottom(20.)
                    .grid_template_columns(table_cols_size.get())
            })
    }))
}

fn tbody(items: RwSignal<im::Vector<WatchlistItem>>) -> impl View {
    let app_state = use_context::<Arc<GlobalState>>().unwrap();
    scroll(
        virtual_list(
            VirtualDirection::Vertical,
            VirtualItemSize::Fixed(Box::new(|| 50.0)),
            move || items.get(),
            move |item| item.media_id,
            move |item| {
                h_stack((
                    label(move || item.list_order.to_string()),
                    label(move || format!("Title for media ID : {}", item.media_id.to_string())),
                    label(move || item.media_kind.to_string()),
                    label(move || {
                        format!("Long note here: {} ", item.note.clone().unwrap_or_default())
                    }),
                    label(move || item.added_on.clone()),
                    label(move || "Rating: 9.5".to_string()),
                    label(move || "2023".to_string()),
                ))
                .class(TableRow)
                .style(move |s| {
                    s.color(PRIMARY_FG_COLOR)
                        .items_center()
                        .background(SECONDARY_BG_COLOR)
                        .border(1.)
                        .min_width(0)
                        .border_color(Color::rgb8(33, 33, 33))
                        .apply_if(item.media_id % 2 == 0, |s| s.background(PRIMARY_BG_COLOR))
                })
            },
        )
        .style(move |s| s.flex_col().width_full()),
    )
    .style(move |s| s.height(900))
}

fn thead() -> impl View {
    let color = Color::rgb8(30, 39, 44);
    h_stack((
        static_label("#"),
        static_label("Title"),
        static_label("Type"),
        static_label("Note"),
        static_label("Added on"),
        static_label("Rating"),
        static_label("Year"),
    ))
    .class(TableRow)
    .style(move |s| {
        s.items_center()
            .font_size(16.0)
            .border(1.)
            .border_color(color)
            .font_bold()
            .background(color)
    })
}
