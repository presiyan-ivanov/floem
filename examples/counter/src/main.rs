use floem::{
    event::{Event, EventListener},
    keyboard::{Key, NamedKey},
    kurbo::{Point, Size},
    peniko::Color,
    reactive::create_signal,
    unit::UnitExt,
    view::View,
    views::{
        container, label, scroll, stack, text, virtual_list, Decorators, VirtualListDirection,
        VirtualListItemSize,
    },
    window::{WindowConfig, WindowLevel},
};

fn app_view() -> impl View {
    let program_list: im::Vector<_> = (0..1000000).collect();
    let (long_list, _) = create_signal(program_list);
    let start_menu = {
        container(
            scroll(
                virtual_list(
                    VirtualListDirection::Vertical,
                    VirtualListItemSize::Fixed(Box::new(|| 20.0)),
                    move || long_list.get(),
                    move |item| *item,
                    move |item| label(move || item.to_string()).style(|s| s.height(20.0).color(Color::YELLOW)),
                ), // .style(|s| s.flex_col()),
            )
        .style(|s| s.width(100.0).height(100.pct()).border(1.0)),
        )
        .style(|s| {
            s.color(Color::WHITE)
                .background(Color::GHOST_WHITE)
                .padding(10.0)
                .height(550.)
                .width(400.)
                // .flex_col()
                .items_center()
        })
    };
    start_menu

    // let long_list: im::Vector<i32> = (0..1000000).collect();
    // let (long_list, _set_long_list) = create_signal(long_list);
    //
    // container(
    //     scroll(
    //         virtual_list(
    //             VirtualListDirection::Vertical,
    //             VirtualListItemSize::Fixed(Box::new(|| 20.0)),
    //             move || long_list.get(),
    //             move |item| *item,
    //             move |item| label(move || item.to_string()).style(|s| s.height(20.0)),
    //         )
    //         .style(|s| s.flex_col()),
    //     )
    //     .style(|s| s.width(100.0).height(100.pct()).border(1.0)),
    // )
    // .style(|s| {
    //     s.size(100.pct(), 100.pct())
    //         .padding_vert(20.0)
    //         .flex_col()
    //         .items_center()
    // })
}

fn main() {
    floem::launch(app_view);
}
