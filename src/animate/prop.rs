use std::{
    any::Any,
    rc::Rc,
    time::{Duration, Instant},
};

use peniko::Color;

use crate::{animate::AnimDirection, style::StylePropRef, unit::Px};

use super::{anim_val::AnimValue, assert_valid_progress, RepeatMode};

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
    pub(crate) values: InterpolatedVal,
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
                let elapsed = now - started_on.clone();
                self.elapsed = elapsed.clone();

                if elapsed >= *target_duration {
                    self.state = PropAnimState::PassFinished { elapsed };
                }
            }
            PropAnimState::PassFinished { elapsed } => match repeat_mode {
                RepeatMode::Forever => {
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
            PropAnimState::Completed { .. } => {}
        }
    }

    // pub fn elapsed(&self) -> Duration {
    //     match &self.state {
    //         PropAnimState::Idle => Duration::ZERO,
    //         PropAnimState::PassInProgress {
    //             started_on,
    //             elapsed,
    //         } => {
    //             let duration = Instant::now() - started_on.clone();
    //             *elapsed + duration
    //         }
    //         PropAnimState::PassFinished { elapsed } => elapsed.clone(),
    //         PropAnimState::Completed { elapsed, .. } => {
    //             elapsed.clone().unwrap_or_else(|| Duration::ZERO)
    //         }
    //     }
    // }
    //
    pub(crate) fn can_advance(&self) -> bool {
        !self.is_completed()
    }

    pub(crate) fn is_completed(&self) -> bool {
        match &self.state {
            PropAnimState::Completed { .. } => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) enum InterpolatedVal {
    Width {
        from: f64,
        to: f64,
    },
    Height {
        from: f64,
        to: f64,
    },
    DynProp {
        prop: StylePropRef,
        from: Rc<dyn Any>,
        to: Rc<dyn Any>,
    },
    // BorderRadius(F64AnimValues),
    // BorderWidth(F64AnimValues),
    // BorderColor(ColorAnimValues),
    // Background(ColorAnimValues),
    // Color(ColorAnimValues),

    //TODO:
    // TranslateX(F64AnimValues),
    // TranslateY(F64AnimValues),
}

impl InterpolatedVal {
    pub(crate) fn get_to(&self) -> AnimValue {
        match self {
            InterpolatedVal::Width{to, ..}
            | InterpolatedVal::Height{to, ..}
                => AnimValue::Float(to.clone()),
                InterpolatedVal::DynProp { to, .. } => {
                    // let to = to.downcast_ref::<StylePropRef>().unwrap();
                    AnimValue::DynProp(to.clone())
                }
            // | AnimPropValues::BorderWidth(p)
            // | AnimPropValues::BorderWidth(p)
            // | AnimPropValues::TranslateX(p)
            // | AnimPropValues::TranslateY(p)
            // | AnimPropValues::Scale(p)
            // AnimPropValues::Background(p)
            // | AnimPropValues::BorderColor(p)
            // | AnimPropValues::Color(p) => AnimValue::Color(p.to),
        }
    }

    pub(crate) fn get_from(&self) -> AnimValue {
        match self {
            InterpolatedVal::Width{from, .. } | InterpolatedVal::Height{from, ..} => AnimValue::Float(*from),
                InterpolatedVal::DynProp { from, .. } => {
                    // let to = to.downcast_ref::<StylePropRef>().unwrap();
                    AnimValue::DynProp(from.clone())
                }
            // | AnimPropValues::BorderWidth(p)
            // | AnimPropValues::TranslateX(p)
            // | AnimPropValues::TranslateY(p)
            // | AnimPropValues::Scale(p)
            // | AnimPropValues::BorderRadius(p)
            // AnimPropValues::Background(p)
            // | AnimPropValues::BorderColor(p)
            // | AnimPropValues::Color(p) => AnimValue::Color(p.from),
        }
    }

    pub(crate) fn animate_float(
        &self,
        from: f64,
        to: f64,
        progress: f64,
        direction: AnimDirection,
    ) -> f64 {
        assert_valid_progress(progress);
        let (from, to) = match direction {
            AnimDirection::Forward => (from, to),
            AnimDirection::Backward => (to, from),
        };
        if progress == 0.0 {
            return from;
        }
        if (1.0 - progress).abs() < f64::EPSILON {
            return to;
        }
        if (from - to).abs() < f64::EPSILON {
            return from;
        }

        from * (1.0 - progress) + to * progress
    }

    pub(crate) fn animate_usize(
        &self,
        from: u8,
        to: u8,
        progress: f64,
        direction: AnimDirection,
    ) -> u8 {
        assert_valid_progress(progress);
        let (from, to) = match direction {
            AnimDirection::Forward => (from, to),
            AnimDirection::Backward => (to, from),
        };

        if progress == 0.0 {
            return from;
        }
        if (1.0 - progress).abs() < f64::EPSILON {
            return to;
        }
        if from == to {
            return from;
        }

        let from = from as f64;
        let to = to as f64;

        let val = from * (1.0 - progress) + to * progress;
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
        progress: f64,
        direction: AnimDirection,
    ) -> Color {
        let r = self.animate_usize(from.r, to.r, progress, direction);
        let g = self.animate_usize(from.g, to.g, progress, direction);
        let b = self.animate_usize(from.b, to.b, progress, direction);
        let a = self.animate_usize(from.a, to.a, progress, direction);
        Color { r, g, b, a }
    }

    pub(crate) fn animate(&self, progress: f64, direction: AnimDirection) -> AnimValue {
        match self {
            InterpolatedVal::Width { from, to } | InterpolatedVal::Height { from, to } => {
                AnimValue::Float(self.animate_float(*from, *to, progress, direction))
            }
            InterpolatedVal::DynProp { prop, from, to } => {
                if let Some(from) = from.downcast_ref::<Px>() {
                    let to = to.downcast_ref::<Px>().unwrap();
                    return AnimValue::DynProp(Rc::new(Px(
                        self.animate_float(from.0, to.0, progress, direction)
                    )));
                }
                if let Some(from) = from.downcast_ref::<f64>() {
                    let to = to.downcast_ref::<f64>().unwrap();
                    return AnimValue::DynProp(Rc::new(
                        self.animate_float(*from, *to, progress, direction),
                    ));
                }
                if let Some(from) = from.downcast_ref::<Color>() {
                    let to = to.downcast_ref::<Color>().unwrap();
                    return AnimValue::DynProp(Rc::new(
                        self.animate_color(*from, *to, progress, direction),
                    ));
                }
                if let Some(from) = from.downcast_ref::<Option<Color>>() {
                    let to = to.downcast_ref::<Option<Color>>().unwrap();
                    let from = from.unwrap();
                    let to = to.unwrap();
                    return AnimValue::DynProp(Rc::new(Some(
                        self.animate_color(from, to, progress, direction),
                    )));
                }
                panic!("unknown type for {prop:?}")
            }
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum AnimPropKind {
    // Scale,
    // TranslateX,
    // TranslateY,
    Width,
    Height,
    Prop { prop: StylePropRef },
}
