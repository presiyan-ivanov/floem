use std::{any::Any, rc::Rc};

use peniko::Color;

#[derive(Debug, Clone)]
pub enum AnimValue {
    Float(f64),
    Color(Color),
    DynProp(Rc<dyn Any>),
}

impl AnimValue {

    pub fn unwrap_f32(self) -> f32 {
        match self {
            AnimValue::Float(v) => v as f32,
            AnimValue::Color(_) => panic!(),
            AnimValue::DynProp(_) => panic!(),
        }
    }

    pub fn unwrap_f64(self) -> f64 {
        match self {
            AnimValue::Float(v) => v,
            AnimValue::Color(_) => panic!(),
            AnimValue::DynProp(prop) => *prop.downcast_ref::<f64>().unwrap(),
        }
    }

    pub fn unwrap_color(self) -> Color {
        match self {
            AnimValue::Color(c) => c,
            AnimValue::Float(_) => panic!(),
            AnimValue::DynProp(prop) => *prop.downcast_ref::<Color>().unwrap(),
        }
    }

    pub fn unwrap_any(self) -> Rc<dyn Any> {
        match self {
            AnimValue::Color(_) => panic!(),
            AnimValue::Float(_) => panic!(),
            AnimValue::DynProp(prop) => prop.clone(),
        }
    }
}
