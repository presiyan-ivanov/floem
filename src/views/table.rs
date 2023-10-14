use std::{hash::Hash, sync::Arc};

use peniko::Color;

use crate::{
    style::Style,
    unit::UnitExt,
    view::View,
    views::{container, list, stack},
};

use super::{scroll, virtual_list, Decorators, VirtualListItemSize, VirtualListVector, Td, Th};

/// Headers/footers
pub const DARK0_BG: Color = Color::BLACK;
/// Inputs
pub const DARK1_BG: Color = Color::GHOST_WHITE;
/// Main background
pub const DARK2_BG: Color = Color::GHOST_WHITE;
/// Selected option background
pub const DARK3_BG: Color = Color::rgb8(137, 137, 137);

// TODO: style structure
// TODO: let widths be percentages
// TODO: let us adjust widths. We'd have to precompute all the widths into a vec/hashmap so maybe we should just force them to give it to us in that form?
// TODO: We might want to alert the container that we modified the widths they set.. so they should be signals.

// `header_fn`: The list of entries in the header.
// `header_key_fn`: A way of identifying each entry. This may just be the entry itself.
// `header_view_fn`: The actual view that should be displayed. Typically just a label.
//
// `widths_fn`: Maps a key to the width of the table column
pub fn table<COL, HF, TH, CSF, HKF, HK, HCV, THCV, ROWSF, ROWS, TD, ROWKF, ROWK, TDCVF, ROWV>(
    header_col_fn: HF,
    header_key_fn: HKF,
    th_content_view_fn: HCV,
    rows_fn: ROWSF,
    row_key_fn: ROWKF,
    td_content_view_fn: TDCVF,
    widths_fn: CSF,
    row_height_px: f64,
) -> impl View
where
    COL: 'static,
    HF: Fn() -> TH + 'static,
    TH: IntoIterator<Item = COL> + 'static,
    CSF: Fn(&COL, Style) -> Style + 'static,
    HKF: Fn(&COL) -> HK + 'static,
    HK: Eq + Hash + 'static,
    HCV: Fn(COL) -> Th<THCV> + 'static,
    THCV: View + 'static,
    Th<THCV>: View + Sized,
    TD: 'static,
    ROWSF: Fn() -> ROWS + 'static,
    ROWS: VirtualListVector<TD> + 'static,
    ROWKF: Fn(&TD) -> ROWK + 'static,
    ROWK: Eq + Hash + 'static,
    TDCVF: Fn(&COL, &TD) -> Td<ROWV> + 'static + Clone,
    ROWV: View + 'static,
    Td<ROWV>: View + Sized
{
    let header_fn = Arc::new(header_col_fn);
    let header_key_fn = Arc::new(header_key_fn);
    let header_view_fn = Arc::new(th_content_view_fn);
    let widths_fn = Arc::new(widths_fn);

    let header_fn2 = header_fn.clone();
    let header_key_fn2 = header_key_fn.clone();
    let widths_fn2 = widths_fn.clone();

    // horizontal scroll
    // scroll(
    stack((
        thead(
            move || header_fn(),
            move |x| header_key_fn(x),
            move |x| header_view_fn(x),
            move |x| widths_fn(x, Style::BASE),
        ),
        tbody(
            move || header_fn2(),
            move |x| header_key_fn2(x),
            move || rows_fn(),
            move |x| row_key_fn(x),
            move |x, y| td_content_view_fn(x, y),
            move |x| widths_fn2(x, Style::BASE),
            row_height_px,
        ),
    ))
    .base_style(|s| {
        s.flex_col()
            .margin_vert(20.px())
            .margin_horiz(20.px())
            .border(0.5.px())
            .border_color(Color::LIGHT_SLATE_GRAY)
    })
    // )
    // .base_style(|s| {
    //     s.width(1200.px())
    //         // .height(90.pct())
    //         .border(1.0)
    //         .border_color(Color::LIGHT_SLATE_GRAY)
    //         .margin_left(20.px())
    //         .margin_top(20.px())
    // })
}

fn thead<COL, HF, TH, WF, HKF, HK, HVF, HV>(
    header_fn: HF,
    header_key_fn: HKF,
    th_content_view_fn: HVF,
    style_fn: WF,
) -> impl View
where
    COL: 'static,
    HF: Fn() -> TH + 'static,
    TH: IntoIterator<Item = COL>,
    WF: Fn(&COL) -> Style + 'static,
    HKF: Fn(&COL) -> HK + 'static,
    HK: Eq + Hash + 'static,
    HVF: Fn(COL) -> Th<HV> + 'static,
    HV: View + 'static,
    Th<HV>: View + Sized,
{
    let header_fn = Arc::new(header_fn);
    let header_key_fn = Arc::new(header_key_fn);
    let header_view_fn = Arc::new(th_content_view_fn);

    // for each column(th)
    list(
        move || header_fn(),
        move |x| header_key_fn(x),
        move |x| {
            let header_view_fn = header_view_fn.clone();
            th_view(move |x| header_view_fn(x), style_fn(&x), x)
        },
    )
    .style(|s| s.background(Color::rgb8(64, 135, 234)))
}

fn th_view<T, VHF, V>(th_content_view_fn: VHF, style: Style, x: T) -> impl View
where
    T: 'static,
    VHF: Fn(T) -> Th<V> + 'static,
    // CSF: Fn(&T) -> Style + 'static,
    V: View + 'static,
    Th<V>: View + Sized,
{
    // let styles = style_fn(&x);
    container(th_content_view_fn(x)).style(move |s| {
        s.padding_horiz(10.0.px())
            .padding_vert(3.0.px())
            .color(Color::WHITE_SMOKE)
            .border_bottom(1.px())
            // .border_right(0.8)
            .border_color(DARK3_BG)
            .apply(style.clone())
    })
}

fn tbody<COL, HF, H, WF, HKF, KH, ROWSF, ROWS, TD, ROWKF, ROWK, TDCVF, TDC>(
    header_fn: HF,
    header_key_fn: HKF,
    rows_fn: ROWSF,
    row_key_fn: ROWKF,
    td_content_view_fn: TDCVF,
    widths_fn: WF,
    row_height_px: f64,
) -> impl View
where
    COL: 'static,
    HF: Fn() -> H + 'static + Clone,
    H: IntoIterator<Item = COL>,
    WF: Fn(&COL) -> Style + 'static + Clone,
    HKF: Fn(&COL) -> KH + 'static + Clone,
    KH: Eq + Hash + 'static,
    TD: 'static,
    ROWSF: Fn() -> ROWS + 'static,
    ROWS: VirtualListVector<TD> + 'static,
    ROWKF: Fn(&TD) -> ROWK + 'static,
    ROWK: Eq + Hash + 'static,
    TDCVF: Fn(&COL, &TD) -> Td<TDC> + 'static + Clone,
    TDC: View + 'static,
    Td<TDC>: View + Sized
{
    //Vertical scroll
    scroll(
        // A list of lists.
        // The outer (virtual) list is for each row in the table.
        // The inner list is for each column in the table.
        // This seems a bit reversed from how you'd lay it out mentally, but it
        // matches how the header works better.
        virtual_list(
            super::VirtualListDirection::Vertical,
            VirtualListItemSize::Fixed(Box::new(move || row_height_px)),
            move || rows_fn(),
            move |x| row_key_fn(x),
            move |x: TD| {
                let row_view_fn = td_content_view_fn.clone();
                let header_fn = header_fn.clone();
                let widths_fn = widths_fn.clone();
                let header_key_fn = header_key_fn.clone();
                let row_view_fn = row_view_fn.clone();
                let widths_fn = widths_fn.clone();

                // row
                list(
                    move || header_fn(),
                    move |x: &COL| header_key_fn(x),
                    move |y: COL| {
                        let row_view_fn = row_view_fn.clone();
                        let widths_fn = widths_fn.clone();
                        let width = widths_fn(&y);
                        td_view(move |x, y| row_view_fn(x, y), &y, &x, width, row_height_px)
                    },
                )
                .style(move |s| s.height(row_height_px))
            },
        )
        .style(|s| s.flex_col()),
    )
    .style(|s| {
        s.width(100.pct())
            .height(97.pct())
            .margin_bottom(50.px())
            .padding_bottom(20.px())
    })
}

fn td_view<TD, U, VHF, V>(
    row_view_fn: VHF,
    x: &TD,
    y: &U,
    style: Style,
    row_height_px: f64,
) -> impl View
where
    TD: 'static,
    U: 'static,
    VHF: Fn(&TD, &U) -> Td<V> + 'static,
    V: View + 'static,
    Td<V>: View + Sized
{
    container(row_view_fn(&x, &y).base_style(|s| s.width(100.pct())))
        .style(move |s| s.apply(style.clone()))
        .base_style(|s| s.background(Color::YELLOW))
}
