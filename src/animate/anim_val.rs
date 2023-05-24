use vello::peniko::Color;

#[derive(Debug, Clone)]
pub enum AnimValue {
    Float(f64),
    Color(Color),
}

impl AnimValue {
    pub fn unwrap_f32(self) -> f32 {
        match self {
            AnimValue::Float(v) => v as f32,
            AnimValue::Color(_) => panic!(),
        }
    }

    pub fn unwrap_f64(self) -> f64 {
        match self {
            AnimValue::Float(v) => v,
            AnimValue::Color(_) => panic!(),
        }
    }

    pub fn unwrap_color(self) -> Color {
        match self {
            AnimValue::Color(c) => c,
            AnimValue::Float(_) => panic!(),
        }
    }
}
