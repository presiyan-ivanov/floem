use floem::{
    cosmic_text::{Style as FontStyle, Weight},
    peniko::Color,
    reactive::create_signal,
    view::View,
    views::{label, table, Decorators},
};

use crate::form::{form, form_item};

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

#[derive(Debug, Clone)]
struct ModRow {
    pub idx: usize,
    pub food_name: String,
    pub calories: i32,
    pub total_fats: f32,
    pub saturated_fats: f32,
    pub sodium_mg: f32,
    pub cholesterol_mg: f32,
}
impl ModRow {
    fn value(&self, idx: usize, entry: ModTableEntry) -> String {
        match entry {
            ModTableEntry::Index => idx.to_string(),
            ModTableEntry::FoodName => self.food_name.clone(),
            ModTableEntry::Calories => self.calories.to_string(),
            ModTableEntry::TotalFats => self.total_fats.to_string(),
            ModTableEntry::SaturatedFats => self.saturated_fats.to_string(),
            ModTableEntry::SodiumMg => self.sodium_mg.to_string(),
            ModTableEntry::CholesterolMg => self.cholesterol_mg.to_string(),
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
        ModTableEntry::Index => base * 2.,
        ModTableEntry::Calories => base * 6.,
        ModTableEntry::FoodName => base * 6.,
        ModTableEntry::SodiumMg => base * 6.,
        ModTableEntry::SaturatedFats => base * 6.,
        ModTableEntry::TotalFats => base * 8.,
        _ => base,
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
                idx: 0,
                food_name: "Dessert".to_string(),
                calories: 240,
                total_fats: 5.,
                saturated_fats: 3.,
                sodium_mg: 1.,
                cholesterol_mg: 2.,
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
                move || ModTableEntry::ACTIVE_MOD_TABLE_ENTRIES,
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
