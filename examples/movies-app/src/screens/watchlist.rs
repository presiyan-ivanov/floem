use std::fmt::Display;
use std::sync::Arc;

use floem::{
    cosmic_text::Weight,
    id::Id,
    keyboard::{Key, ModifiersState, NamedKey},
    peniko::Color,
    reactive::{create_effect, create_rw_signal, create_signal, use_context, ReadSignal, RwSignal},
    style::{Cursor, CursorStyle, FlexWrap, FontWeight, Position},
    style_class,
    taffy::{
        geometry::MinMax,
        style::{MaxTrackSizingFunction, MinTrackSizingFunction, TrackSizingFunction},
    },
    unit::UnitExt,
    view::View,
    views::{
        container, dyn_container, empty, h_stack, h_stack_from_iter, label, list, scroll,
        static_label, svg, text, v_stack, v_stack_from_iter, virtual_list, virtual_stack,
        Decorators, Label, VirtualDirection, VirtualItemSize,
    },
};
use serde::{Deserialize, Serialize};

use crate::{GlobalState, PRIMARY_BG_COLOR, PRIMARY_FG_COLOR, SECONDARY_BG_COLOR, DIMMED_ACCENT_COLOR};

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

//README-bookmark.style-classes
style_class!(TableRow);

pub fn watchlist_view() -> impl View {
    let watchlist_json = include_str!("../../assets/data/watchlist.json");
    let watchlist: Watchlist =
        serde_json::from_str(watchlist_json).expect("Watchlist JSON was not well-formatted");
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
            min: MinTrackSizingFunction::Fixed(floem::taffy::style::LengthPercentage::Points(300.)),
            max: MaxTrackSizingFunction::Fixed(floem::taffy::style::LengthPercentage::Points(450.)),
        }),
        TrackSizingFunction::Single(MinMax {
            min: MinTrackSizingFunction::Fixed(floem::taffy::style::LengthPercentage::Points(80.)),
            max: MaxTrackSizingFunction::Fixed(floem::taffy::style::LengthPercentage::Points(130.)),
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
    let active_sort: RwSignal<Option<Sort>> = create_rw_signal(None);
    create_effect(move |_| {
        let sort = active_sort.get();

        items.update(|items| match sort {
            Some(sort) => {
                items.sort_by(|a, b| {
                    let (a, b) = if sort.direction == SortDirection::Asc {
                        (a, b)
                    } else {
                        (b, a)
                    };

                    match sort.sort_by {
                        WatchlistCol::ListOrder => a.list_order.cmp(&b.list_order),
                        WatchlistCol::Title => todo!(),
                        WatchlistCol::Type => a.media_kind.cmp(&b.media_kind),
                        WatchlistCol::Note => a.note.cmp(&b.note),
                        WatchlistCol::AddedOn => a.added_on.cmp(&b.added_on),
                        WatchlistCol::Rating => todo!(),
                        WatchlistCol::Year => todo!(),
                    }
                });
            }
            None => {}
        })
    });

    container(
        scroll(v_stack((thead(active_sort), tbody(items))))
            .on_scroll(move |rect| {
                dbg!(rect);
            })
            .style(move |s| {
                s.width(app_state.window_size.get().width - margin_horiz - 100.)
                    .margin_horiz(margin_horiz)
                    .padding_bottom(20)
                    .margin_top(30)
                    .class(TableRow, move |s| {
                        s.display(floem::style::Display::Grid)
                            .height(50.)
                            .padding_left(10.)
                            .grid_template_columns(table_cols_size.get())
                    })
            }),
    )
}

#[derive(Clone)]
struct Sort {
    sort_by: WatchlistCol,
    direction: SortDirection,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum WatchlistCol {
    ListOrder,
    Title,
    Type,
    Note,
    AddedOn,
    Rating,
    Year,
}

impl Display for WatchlistCol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            WatchlistCol::ListOrder => write!(f, "#"),
            WatchlistCol::Title => write!(f, "Title"),
            WatchlistCol::Type => write!(f, "Type"),
            WatchlistCol::Note => write!(f, "Note"),
            WatchlistCol::AddedOn => write!(f, "Added on"),
            WatchlistCol::Rating => write!(f, "Rating"),
            WatchlistCol::Year => write!(f, "Year"),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
enum SortDirection {
    Asc,
    Desc,
}

impl SortDirection {
    fn reversed(&self) -> Self {
        match *self {
            SortDirection::Asc => SortDirection::Desc,
            SortDirection::Desc => SortDirection::Asc,
        }
    }
}

fn thead(active_sort: RwSignal<Option<Sort>>) -> impl View {
    let color = Color::rgb8(30, 39, 44);

    let th = |col: WatchlistCol| {
        container(
            h_stack((text(col), sort_icon(active_sort.read_only(), col)))
                .on_click_stop(move |_| {
                    let sort_dir = match active_sort.get_untracked() {
                        Some(sort) if sort.sort_by == col => sort.direction.reversed(),
                        _ => SortDirection::Asc,
                    };

                    active_sort.set(Some(Sort {
                        sort_by: col,
                        direction: sort_dir,
                    }));
                })
                .style(move |s| s.cursor(CursorStyle::Pointer)),
        )
    };

    h_stack((
        th(WatchlistCol::ListOrder),
        th(WatchlistCol::Title),
        th(WatchlistCol::Type),
        th(WatchlistCol::Note),
        th(WatchlistCol::AddedOn),
        th(WatchlistCol::Rating),
        th(WatchlistCol::Year),
    ))
    .style(move |s| {
        s.items_center()
            .font_size(20.0)
            .color(PRIMARY_FG_COLOR)
            .border(1.)
            .border_color(color)
            .font_bold()
            .background(color)
    })
    .class(TableRow)
}

fn sort_icon(active_sort: ReadSignal<Option<Sort>>, col: WatchlistCol) -> impl View {
    dyn_container(
        move || active_sort.get(),
        move |val| {
            let sort_icon =
                val.filter(|sort| sort.sort_by == col)
                    .map(|sort| match sort.direction {
                        SortDirection::Asc => include_str!("../../assets/arrow-up.svg"),
                        SortDirection::Desc => {
                            include_str!("../../assets/arrow-down.svg")
                        }
                    });

            match sort_icon {
                Some(icon) => Box::new(
                    svg(|| icon.to_owned()).style(|s| s.size(20, 20).color(PRIMARY_FG_COLOR)),
                ),
                None => Box::new(empty()),
            }
        },
    )
}

fn tbody(items: RwSignal<im::Vector<WatchlistItem>>) -> impl View {
    let edited_item: RwSignal<Option<u64>> = create_rw_signal(None);

    let scroll = scroll(
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
                    })
                    .style(move |s| {
                        s.cursor(CursorStyle::Pointer)
                            .border_color(Color::TRANSPARENT)
                            .border_bottom(1)
                            .hover(|s| s.border_color(DIMMED_ACCENT_COLOR))
                    })
                    .on_click_stop(move |_| {
                        edited_item.set(Some(item.media_id));
                    }),
                    label(move || item.added_on.clone()),
                    label(move || "Rating: 9.5".to_string()),
                    label(move || "2023".to_string()),
                ))
                .class(TableRow)
                .style(move |s| {
                    s.color(PRIMARY_FG_COLOR)
                        .font_size(14.)
                        // FIXME: this is workaround for an issue w styles incorrectly being applied
                        // between different elements that use the same class.
                        // When the items's order is changed by sorting,
                        // the body rows would otherwise inherit styles from the header row(since they both use the TableRow class),
                        // causing body rows to have incorrect font weight/size(same as the header row)
                        .font_weight(Weight::NORMAL)
                        .items_center()
                        .background(SECONDARY_BG_COLOR)
                        .border(1.)
                        .min_width(0)
                        .border_color(Color::rgb8(33, 33, 33))
                        //README-bookmark.conditional-styling
                        .apply_if(item.media_id % 2 == 0, |s| s.background(PRIMARY_BG_COLOR))
                })
            },
        )
        .style(move |s| s.flex_col().width_full()),
    )
    .on_scroll(move |rect| {
        dbg!(rect);
    })
    .style(move |s| s.height(900));

    v_stack((
        scroll,
        dyn_container(
            move || edited_item.get(),
            move |val| match val {
                Some(id) => Box::new(label(move || id).style(|s| s.background(Color::BLUE))),
                None => Box::new(empty()),
            },
        ),
    ))
}
