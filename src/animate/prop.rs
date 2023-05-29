use std::time::{Duration, Instant};

use vello::peniko::Color;

use crate::animate::AnimDirection;

use super::{anim_val::AnimValue, assert_valid_time, RepeatMode};

#[derive(Clone, Debug)]
pub(crate) struct F64AnimValues {
    pub(crate) from: f64,
    pub(crate) to: f64,
}

#[derive(Clone, Debug)]
pub(crate) struct ColorAnimValues {
    pub(crate) from: Color,
    pub(crate) to: Color,
}

#[derive(Clone, Debug)]
pub(crate) enum PropAnimState {
    Idle,
    PassInProgress {
        started_on: Instant,
        elapsed: Duration,
    },
    PassFinished {
        elapsed: Duration,
    },
    // NOTE: If the repeat mode of the animation is `RepeatMode::LoopForever`, this state will never be reached.
    Completed {
        elapsed: Option<Duration>,
    },
}

#[derive(Clone, Debug)]
pub(crate) struct AnimatedProp {
    pub(crate) values: AnimPropValues,
    pub(crate) elapsed: Duration,
    pub(crate) started_on: Instant,
    pub(crate) state: PropAnimState,
    pub(crate) repeats_count: usize,
}

impl AnimatedProp {
    pub fn begin(&mut self) {
        self.repeats_count = 0;
        self.state = PropAnimState::PassInProgress {
            started_on: Instant::now(),
            elapsed: Duration::ZERO,
        }
    }

    pub fn advance(&mut self, repeat_mode: RepeatMode, target_duration: &Duration) {
        match self.state {
            PropAnimState::Idle => {
                self.begin();
            }
            PropAnimState::PassInProgress {
                started_on,
                mut elapsed,
            } => {
                let now = Instant::now();
                let duration = now - started_on.clone();
                elapsed += duration;

                if elapsed >= *target_duration {
                    self.state = PropAnimState::PassFinished { elapsed };
                }
            }
            PropAnimState::PassFinished { elapsed } => match repeat_mode {
                RepeatMode::LoopForever => {
                    self.state = PropAnimState::PassInProgress {
                        started_on: Instant::now(),
                        elapsed: Duration::ZERO,
                    }
                }
                RepeatMode::Times(times) => {
                    self.repeats_count += 1;
                    if self.repeats_count >= times {
                        self.state = PropAnimState::Completed {
                            elapsed: Some(elapsed),
                        }
                    } else {
                        self.state = PropAnimState::PassInProgress {
                            started_on: Instant::now(),
                            elapsed: Duration::ZERO,
                        }
                    }
                }
            },
            PropAnimState::Completed {..}=> {}
        }
    }

    pub fn elapsed(&self) -> Duration {
        match &self.state {
            PropAnimState::Idle => Duration::ZERO,
            PropAnimState::PassInProgress {
                started_on,
                elapsed,
            } => {
                let duration = Instant::now() - started_on.clone();
                *elapsed + duration
            }
            PropAnimState::PassFinished { elapsed } => elapsed.clone(),
            PropAnimState::Completed { elapsed, .. } => {
                elapsed.clone().unwrap_or_else(|| Duration::ZERO)
            }
        }
    }

    pub(crate) fn is_completed(&self) -> bool {
        match &self.state {
            PropAnimState::Completed { elapsed } => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) enum AnimPropValues {
    Width(F64AnimValues),
    Height(F64AnimValues),
    Scale(F64AnimValues),
    TranslateX(F64AnimValues),
    TranslateY(F64AnimValues),
    BorderRadius(F64AnimValues),
    BorderWidth(F64AnimValues),

    BorderColor(ColorAnimValues),
    Background(ColorAnimValues),
    Color(ColorAnimValues),
}

impl AnimPropValues {
    pub(crate) fn get_to(&self) -> AnimValue {
        match self {
            AnimPropValues::Width(p)
            | AnimPropValues::Height(p)
            | AnimPropValues::BorderWidth(p)
            | AnimPropValues::TranslateX(p)
            | AnimPropValues::TranslateY(p)
            | AnimPropValues::Scale(p)
            | AnimPropValues::BorderRadius(p) => AnimValue::Float(p.to),
            AnimPropValues::Background(p)
            | AnimPropValues::BorderColor(p)
            | AnimPropValues::Color(p) => AnimValue::Color(p.to),
        }
    }

    pub(crate) fn get_from(&self) -> AnimValue {
        match self {
            AnimPropValues::Width(p)
            | AnimPropValues::Height(p)
            | AnimPropValues::BorderWidth(p)
            | AnimPropValues::TranslateX(p)
            | AnimPropValues::TranslateY(p)
            | AnimPropValues::Scale(p)
            | AnimPropValues::BorderRadius(p) => AnimValue::Float(p.from),
            AnimPropValues::Background(p)
            | AnimPropValues::BorderColor(p)
            | AnimPropValues::Color(p) => AnimValue::Color(p.from),
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
            AnimPropValues::Background(p)
            | AnimPropValues::BorderColor(p)
            | AnimPropValues::Color(p) => {
                AnimValue::Color(self.animate_color(p.from, p.to, time, direction))
            }
            AnimPropValues::Width(p) | AnimPropValues::Height(p) => {
                AnimValue::Float(self.animate_float(p.from, p.to, time, direction))
            }
            AnimPropValues::Scale(p)
            | AnimPropValues::BorderRadius(p)
            | AnimPropValues::BorderWidth(p)
            | AnimPropValues::TranslateX(p)
            | AnimPropValues::TranslateY(p) => {
                AnimValue::Float(self.animate_float(p.from, p.to, time, direction))
            }
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum AnimPropKind {
    Scale,
    ColorAnimPropValues,
    TranslateY,
    Width,
    Background,
    Color,
    Height,
    BorderRadius,
    BorderColor,
}
