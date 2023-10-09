use floem::{
    cosmic_text::{Style as FontStyle, Weight},
    peniko::Color,
    reactive::create_signal,
    view::View,
    views::{label, table, Decorators},
};
use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ModTableEntry {
    /// Load order index
    Index,
    FoodName,
    Calories,
    TotalFats,
    SaturatedFats,
    SodiumMg,
    CholesterolMg,
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
            Self::FoodName => "Food",
            Self::Calories => "Calories",
            Self::TotalFats => "Fats",
            Self::SaturatedFats => "Saturated Fats",
            Self::SodiumMg => "Sodium (mg)",
            Self::CholesterolMg => "Cholesterol (mg)",
            Self::MoreActions => "",
        }
    }

    const ACTIVE_MOD_TABLE_ENTRIES: [ModTableEntry; 6] = [
        Self::Index,
        Self::FoodName,
        Self::Calories,
        Self::TotalFats,
        Self::SaturatedFats,
        Self::MoreActions,
    ];
}

#[derive(Debug, Clone, Deserialize)]
struct ModRow {
    pub idx: usize,
    pub name: String,
    pub calories: i32,
    pub total_fat: String,
    pub saturated_fat: String,
    pub sodium: String,
    pub cholesterol: String,
}
impl ModRow {
    fn value(&self, idx: usize, entry: ModTableEntry) -> String {
        match entry {
            ModTableEntry::Index => idx.to_string(),
            ModTableEntry::FoodName => self.name.clone(),
            ModTableEntry::Calories => self.calories.to_string(),
            ModTableEntry::TotalFats => self.total_fat.to_string(),
            ModTableEntry::SaturatedFats => self.saturated_fat.to_string(),
            ModTableEntry::SodiumMg => self.sodium.to_string(),
            ModTableEntry::CholesterolMg => self.cholesterol.to_string(),
            ModTableEntry::MoreActions => String::new(),
        }
    }
}

// <table>
//   <tr>
//     <th>Company</th>
//     <th>Contact</th>
//     <th>Country</th>
//   </tr>
//   <tr>
//     <td>Alfreds Futterkiste</td>
//     <td>Maria Anders</td>
//     <td>Germany</td>
//   </tr>
//   <tr>
//     <td>Centro comercial Moctezuma</td>
//     <td>Francisco Chang</td>
//     <td>Mexico</td>
//   </tr>
// </table>
//
fn mod_table_entry_sizes(x: &ModTableEntry) -> f64 {
    let base = 24.0;
    match x {
        ModTableEntry::Index => base * 4.,
        ModTableEntry::Calories => base * 5.,
        ModTableEntry::FoodName => base * 25.,
        ModTableEntry::SodiumMg => base * 6.,
        ModTableEntry::SaturatedFats => base * 6.,
        ModTableEntry::TotalFats => base * 8.,
        _ => base,
    }
}

fn mod_table_text(x: ModTableEntry) -> impl View {
    label(move || x.title().to_string()).style(|s| s.font_size(14.0))
}

pub fn app_view() -> impl View {
    let mut rdr = csv::Reader::from_path("./nutrition.csv").unwrap();
    let mut rows = vec![];
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let row: ModRow = result.unwrap();
        rows.push((row.idx, row));
    }
    let rows: im::Vector<(usize, ModRow)>= rows.into();

    table(
        move || ModTableEntry::ACTIVE_MOD_TABLE_ENTRIES,
        Clone::clone,
        mod_table_text,
        move || rows.clone(),
        |(idx, _)| *idx,
        mod_entry_text,
        mod_table_entry_sizes,
    ).style(|s| s.border(2.0))
}

fn main() {
    floem::launch(app_view);
}
