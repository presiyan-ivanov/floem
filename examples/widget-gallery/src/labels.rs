use floem::{
    cosmic_text::{Style as FontStyle, Weight},
    peniko::Color,
    reactive::create_signal,
    view::View,
    views::{table, Decorators, label},
};

use crate::form::{form, form_item};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ModTableEntry {
    /// Load order index
    Index,
    Name,
    Version,
    Author,
    LastUpdated,
    // TODO: Should we have some inbuilt way in table to have a blank space? Eh.
    Blank,
}

fn mod_entry_text(x: &ModTableEntry, (idx, row): &(usize, ModRow)) -> impl View {
    let row_value = row.value(*idx, *x);
    label(move || row_value.clone()).style(|s| s.font_size(14.0))
}

impl ModTableEntry {
    fn title(&self) -> &'static str {
        match self {
            Self::Index => "#",
            Self::Name => "Name",
            Self::Version => "Version",
            Self::Author => "Author",
            Self::LastUpdated => "Last Updated",
            Self::Blank => "",
        }
    }
}

const ACTIVE_MOD_TABLE_ENTRIES: [ModTableEntry; 6] = [
    ModTableEntry::Index,
    ModTableEntry::Name,
    ModTableEntry::Version,
    ModTableEntry::Author,
    ModTableEntry::LastUpdated,
    ModTableEntry::Blank,
];

#[derive(Debug, Clone)]
struct ModRow {
    pub name: String,
    pub version: String,
    pub author: String,
    pub last_updated: String,
}
impl ModRow {
    fn value(&self, idx: usize, entry: ModTableEntry) -> String {
        match entry {
            ModTableEntry::Index => idx.to_string(),
            ModTableEntry::Name => self.name.clone(),
            ModTableEntry::Version => self.version.clone(),
            ModTableEntry::Author => self.author.clone(),
            ModTableEntry::LastUpdated => self.last_updated.clone(),
            ModTableEntry::Blank => String::new(),
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
        ModTableEntry::Index => base * 2.,
        ModTableEntry::Name => base * 6.,
        ModTableEntry::Version => base * 6.,
        ModTableEntry::Author => base * 6.,
        ModTableEntry::LastUpdated => base * 6.,
        ModTableEntry::Blank => base * 8.,
    }
}

struct FoodNutritionRow {
    idx: usize,
    food_name: String,
    calories: i32,
    total_fats: f32,
    saturated_fats: f32,
    sodium_mg: f32,
    cholesterol_mg: f32,
}

enum SeverityKind {
    Info,
    Warn,
    Error,
}

fn mod_table_text(x: ModTableEntry) -> impl View {
    label(move || x.title().to_string()).style(|s| s.font_size(14.0))
}

pub fn label_view() -> impl View {
    // let food_nutrition = create_signal(vec![
    //     FoodNutritionRow {
    //         idx: 1,
    //         food_name: "Orange".to_string(),
    //         calories: 234,
    //         total_fats: 4.3,
    //         saturated_fats: 2.1,
    //         sodium_mg: 12.1,
    //         cholesterol_mg: 23.1,
    //     },
    //     FoodNutritionRow {
    //         idx: 2,
    //         food_name: "Apple".to_string(),
    //         calories: 234,
    //         total_fats: 4.3,
    //         saturated_fats: 2.1,
    //         sodium_mg: 12.1,
    //         cholesterol_mg: 13.1,
    //     },
    // ]);
    let rows = im::Vector::from_iter(
        [
            ModRow {
                name: "DndRebalancing".to_string(),
                version: "1.0.0.0".to_string(),
                author: "Zerd".to_string(),
                last_updated: "11/5/2020".to_string(),
            },
            ModRow {
                name: "8 More Short Rests".to_string(),
                version: "-1.15".to_string(),
                author: "Logos".to_string(),
                last_updated: "11/5/2020".to_string(),
            },
            ModRow {
                name: "Customizer".to_string(),
                version: "1.0.0.0".to_string(),
                author: "AlanaSP".to_string(),
                last_updated: "12/1/2020".to_string(),
            },
        ]
        .into_iter()
        .enumerate(),
    );

    form({
        (
            // table::<FoodNutritionRow>(
            //     head(move  || {
            //         row(move ||  {
            //         th("#"),
            //         th("Food"),
            //         th("Calories"),
            //         th("Total fats"),
            //         th("Saturated fats"),
            //         th("Sodium mg"),
            //         th("Cholesterol mg"),
            //         th()
            //         })
            //     }),
            //     body(food_nutrition.iter().map()
            //         vec![
            //             td(entry.idx),
            //             td(entry.food_name),
            //             td(entry.calories),
            //             td(entry.total_fats),
            //             td(entry.saturated_fats),
            //             td(entry.sodium_mg),
            //             td(entry.cholesterol_mg),
            //             td(label(move || "Add to favorites").on_click(move |_| {
            //                 set_counter.update(|value| *value -= 1);
            //                 true
            //             })),
            //         ]
            //     })]),
            table(
                move || ACTIVE_MOD_TABLE_ENTRIES,
                Clone::clone,
                mod_table_text,
                move || rows.clone(),
                |(idx, _)| *idx,
                mod_entry_text,
                mod_table_entry_sizes,
            ),
            form_item("Simple Label:".to_string(), 120.0, || {
                label(move || "This is a simple label".to_owned())
            }),
            form_item("Styled Label:".to_string(), 120.0, || {
                label(move || "This is a styled label".to_owned()).style(|s| {
                    s.background(Color::YELLOW)
                        .padding(10.0)
                        .color(Color::GREEN)
                        .font_weight(Weight::BOLD)
                        .font_style(FontStyle::Italic)
                        .font_size(24.0)
                })
            }),
        )
    })
}
