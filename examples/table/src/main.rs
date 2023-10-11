use floem::{
    cosmic_text::{Style as FontStyle, Weight},
    peniko::Color,
    reactive::create_signal,
    style::Style,
    unit::UnitExt,
    view::View,
    views::{label, table, Decorators},
};
use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum TableCol {
    /// Load order index
    Index,
    Title,
    Author,
    Seller,
    PublishedOn,
    Category,
    Stars,
    Reviews,
    Price,
    MoreActions,
}

fn td_view(x: &TableCol, (idx, row): &(usize, ModRow)) -> impl View {
    let row_value = row.value(*idx, *x);
    let num = idx.clone();
    label(move || row_value.clone()).style(move |s| {
        s.font_size(14.0)
            .width(100.pct())
            .apply_if(num % 2 == 0, |s| s.background(Color::WHITE_SMOKE))
    })
}

impl TableCol {
    fn title(&self) -> &'static str {
        match self {
            Self::Index => "#",
            Self::Title => "Title",
            Self::Author => "Author",
            Self::Seller => "Seller",
            Self::PublishedOn => "Published On",
            Self::Category => "Category",
            Self::Stars => "Stars",
            Self::Reviews => "Reviews",
            Self::Price => "Price",
            Self::MoreActions => "Actions",
        }
    }

    const ALL_COLUMNS: [TableCol; 10] = [
        Self::Index,
        Self::Title,
        Self::Author,
        Self::Stars,
        Self::Seller,
        Self::Price,
        Self::PublishedOn,
        Self::Category,
        Self::Reviews,
        Self::MoreActions,
    ];
}

#[derive(Debug, Clone, Deserialize)]
struct ModRow {
    pub title: String,
    pub author: String,
    pub category_name: String,
    pub stars: String,
    #[serde(rename(deserialize = "publishedDate"))]
    pub published_on: String,
    #[serde(rename(deserialize = "soldBy"))]
    pub seller: String,
    pub reviews: String,
    pub price: String,
}
impl ModRow {
    fn value(&self, idx: usize, entry: TableCol) -> String {
        match entry {
            TableCol::Index => idx.to_string(),
            TableCol::Title => self.title.clone(),
            TableCol::Author => self.author.to_string(),
            TableCol::Category => self.category_name.to_string(),
            TableCol::Stars => self.stars.to_string(),
            TableCol::Reviews => self.reviews.to_string(),
            TableCol::Seller => self.seller.to_string(),
            TableCol::PublishedOn => self.published_on.to_string(),
            TableCol::Price => self.price.to_string(),
            TableCol::MoreActions => "TODO".to_owned(),
        }
    }
}

// fn mod_table_entry_sizes(x: &ModTableEntry) -> Fn(&ModTableEntry) -> Style + 'static {
//     let base = 24.0;
//     match x {
//         ModTableEntry::Index => Style::BASE.width(base * 3.),
//         ModTableEntry::Author | ModTableEntry::Seller | ModTableEntry::Category => {
//             Style::BASE.width(base * 8.).background(Color::RED)
//         }
//         ModTableEntry::Title => Style::BASE.width(base * 25.),
//         ModTableEntry::Stars | ModTableEntry::Reviews | ModTableEntry::Price => {
//             Style::BASE.width(base * 4.)
//         }
//         _ => Style::BASE.width(base * 5.),
//     }
// }

fn th_view(x: TableCol) -> impl View {
    label(move || x.title().to_string())
        .style(|s| s.font_size(16.0).font_bold().padding_vert(15.px()))
}

pub fn app_view() -> impl View {
    let mut rdr = csv::Reader::from_path("./kindle_data-v2.csv").unwrap();
    let mut rows = vec![];
    for (idx, result) in rdr.deserialize().enumerate() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let row: ModRow = result.unwrap();
        rows.push((idx + 1, row));
    }
    let rows: im::Vector<(usize, ModRow)> = rows.into();
    let base = 24.0;

    table(
        move || TableCol::ALL_COLUMNS,
        Clone::clone,
        move |col| {
            label(move || col.title().to_string())
                .style(|s| s.font_size(16.0).font_bold().padding_vert(15.px()))
        },
        move || rows.clone(),
        |(idx, _)| *idx,
        move |x: &TableCol, (idx, row): &(usize, ModRow)| {
            let row_value = row.value(*idx, *x);
            let num = idx.clone();
            label(move || row_value.clone()).style(move |s| {
                s.font_size(14.0)
                    .width(100.pct())
                    .apply_if(num % 2 == 0, |s| s.background(Color::WHITE_SMOKE))
            })
        },
        move |col, s| match col {
            TableCol::Index => s.width(base * 3.),
            TableCol::Author | TableCol::Seller | TableCol::Category => {
                s.width(base * 8.).background(Color::RED)
            }
            TableCol::Title => s.width(base * 25.),
            TableCol::Stars | TableCol::Reviews | TableCol::Price => s.width(base * 4.),
            _ => s.width(base * 5.),
        },
        40.0,
    )
    // .style(|s| {
    //     s.border(1.0)
    //         .border_color(Color::rgb(137., 137., 137.))
    //         .margin_horiz(20.px())
    // })
}

fn main() {
    floem::launch(app_view);
}
