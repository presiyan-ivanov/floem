use floem::{
    cosmic_text::{Style as FontStyle, Weight},
    peniko::Color,
    reactive::create_signal,
    unit::UnitExt,
    view::View,
    views::{label, table, Decorators},
};
use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ModTableEntry {
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

fn mod_entry_text(x: &ModTableEntry, (idx, row): &(usize, ModRow)) -> impl View {
    let row_value = row.value(*idx, *x);
    label(move || row_value.clone()).style(|s| s.font_size(14.0))
}

impl ModTableEntry {
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

    const ACTIVE_MOD_TABLE_ENTRIES: [ModTableEntry; 10] = [
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
    fn value(&self, idx: usize, entry: ModTableEntry) -> String {
        match entry {
            ModTableEntry::Index => idx.to_string(),
            ModTableEntry::Title => self.title.clone(),
            ModTableEntry::Author => self.author.to_string(),
            ModTableEntry::Category => self.category_name.to_string(),
            ModTableEntry::Stars => self.stars.to_string(),
            ModTableEntry::Reviews => self.reviews.to_string(),
            ModTableEntry::Seller => self.seller.to_string(),
            ModTableEntry::PublishedOn => self.published_on.to_string(),
            ModTableEntry::Price => self.price.to_string(),
            ModTableEntry::MoreActions => "TODO".to_owned(),
        }
    }
}

fn mod_table_entry_sizes(x: &ModTableEntry) -> f64 {
    let base = 24.0;
    match x {
        ModTableEntry::Index => base * 3.,
        ModTableEntry::Author | ModTableEntry::Seller | ModTableEntry::Category => base * 8.,
        ModTableEntry::Title => base * 25.,
        ModTableEntry::Stars | ModTableEntry::Reviews | ModTableEntry::Price => base * 4.,
        _ => base * 5.,
    }
}

fn mod_table_text(x: ModTableEntry) -> impl View {
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

    table(
        move || ModTableEntry::ACTIVE_MOD_TABLE_ENTRIES,
        Clone::clone,
        mod_table_text,
        move || rows.clone(),
        |(idx, _)| *idx,
        mod_entry_text,
        mod_table_entry_sizes,
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
