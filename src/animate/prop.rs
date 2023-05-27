use std::time::Duration;

use vello::peniko::Color;

use crate::animate::AnimDirection;

use super::{anim_val::AnimValue, assert_valid_time};

#[derive(Clone, Debug)]
pub(crate) struct F64AnimProp {
    from: f64,
    to: f64,
    elapsed: Duration,
}

impl F64AnimProp {
    pub(crate) fn new(from: f64, to: f64) -> Self {
        Self {
            from,
            to,
            elapsed: Duration::ZERO,
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct ColorAnimProp {
    from: Color,
    to: Color,
    elapsed: Duration,
}

impl ColorAnimProp {
    pub(crate) fn new(from: Color, to: Color) -> Self {
        Self {
            from,
            to,
            elapsed: Duration::ZERO,
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) enum AnimatedProp {
    Width(F64AnimProp),
    Height(F64AnimProp),
    Scale(F64AnimProp),
    // Scale(F64AnimProp)
    TranslateX(F64AnimProp),
    TranslateY(F64AnimProp),
    BorderRadius(F64AnimProp),
    BorderWidth(F64AnimProp),
    BorderColor(ColorAnimProp),
    Background(ColorAnimProp),
    Color(ColorAnimProp),
}

impl AnimatedProp {
    pub(crate) fn get_elapsed(&self) -> Duration {
        match self {
            AnimatedProp::Width(p)
            | AnimatedProp::Height(p)
            | AnimatedProp::BorderWidth(p)
            | AnimatedProp::TranslateX(p)
            | AnimatedProp::TranslateY(p)
            | AnimatedProp::Scale(p)
            | AnimatedProp::BorderRadius(p) => p.elapsed,
            AnimatedProp::Background(p) | AnimatedProp::BorderColor(p) | AnimatedProp::Color(p) => {
                p.elapsed
            }
        }
    }

    pub(crate) fn get_to_val(&self) -> AnimValue {
        match self {
            AnimatedProp::Width(p)
            | AnimatedProp::Height(p)
            | AnimatedProp::BorderWidth(p)
            | AnimatedProp::TranslateX(p)
            | AnimatedProp::TranslateY(p)
            | AnimatedProp::Scale(p)
            | AnimatedProp::BorderRadius(p) => AnimValue::Float(p.to),
            AnimatedProp::Background(p) | AnimatedProp::BorderColor(p) | AnimatedProp::Color(p) => {
                AnimValue::Color(p.to)
            }
        }
    }

    pub(crate) fn get_from_val(&self) -> AnimValue {
        match self {
            AnimatedProp::Width(p)
            | AnimatedProp::Height(p)
            | AnimatedProp::BorderWidth(p)
            | AnimatedProp::TranslateX(p)
            | AnimatedProp::TranslateY(p)
            | AnimatedProp::Scale(p)
            | AnimatedProp::BorderRadius(p) => AnimValue::Float(p.from),
            AnimatedProp::Background(p) | AnimatedProp::BorderColor(p) | AnimatedProp::Color(p) => {
                AnimValue::Color(p.from)
            }
        }
    }

    pub(crate) fn animate_float(
        &self,
        from: f64,
        to: f64,
        time: f64,
        direction: AnimDirection,
    ) -> f64 {
        assert_valid_time(time);
        let (from, to) = match direction {
            AnimDirection::Forward => (from, to),
            AnimDirection::Backward => (to, from),
        };
        if time == 0.0 {
            return from;
        }
        if (1.0 - time).abs() < f64::EPSILON {
            return to;
        }
        if (from - to).abs() < f64::EPSILON {
            return from;
        }

        from * (1.0 - time) + to * time
    }

    pub(crate) fn animate_usize(
        &self,
        from: u8,
        to: u8,
        time: f64,
        direction: AnimDirection,
    ) -> u8 {
        assert_valid_time(time);
        let (from, to) = match direction {
            AnimDirection::Forward => (from, to),
            AnimDirection::Backward => (to, from),
        };

        if time == 0.0 {
            return from;
        }
        if (1.0 - time).abs() < f64::EPSILON {
            return to;
        }
        if from == to {
            return from;
        }

        let from = from as f64;
        let to = to as f64;

        let val = from * (1.0 - time) + to * time;
        if to >= from {
            (val + 0.5) as u8
        } else {
            (val - 0.5) as u8
        }
    }

    pub(crate) fn animate_color(
        &self,
        from: Color,
        to: Color,
        time: f64,
        direction: AnimDirection,
    ) -> Color {
        let r = self.animate_usize(from.r, to.r, time, direction);
        let g = self.animate_usize(from.g, to.g, time, direction);
        let b = self.animate_usize(from.b, to.b, time, direction);
        let a = self.animate_usize(from.a, to.a, time, direction);
        Color { r, g, b, a }
    }

    pub(crate) fn animate(&self, time: f64, direction: AnimDirection) -> AnimValue {
        match self {
            AnimatedProp::Width(p) | AnimatedProp::Height(p) => {
                AnimValue::Float(self.animate_float(p.from, p.to, time, direction))
            }
            AnimatedProp::Background(p) | AnimatedProp::BorderColor(p) | AnimatedProp::Color(p) => {
                AnimValue::Color(self.animate_color(p.from, p.to, time, direction))
            }
            AnimatedProp::Scale(p)
            | AnimatedProp::BorderRadius(p)
            | AnimatedProp::BorderWidth(p)
            | AnimatedProp::TranslateX(p)
            | AnimatedProp::TranslateY(p) => {
                AnimValue::Float(self.animate_float(p.from, p.to, time, direction))
            }
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum AnimPropKind {
    Scale,
    TranslateX,
    TranslateY,
    Width,
    Background,
    Color,
    Height,
    BorderRadius,
    BorderColor,
}
