use super::{
    anim_val::AnimValue, AnimId, AnimPropKind, AnimPropValues, AnimState, AnimatedProp, Easing,
    EasingFn, EasingMode,
};
use std::{borrow::BorrowMut, collections::HashMap, time::Duration, time::Instant};

use floem_reactive::create_effect;
use once_cell::sync::Lazy;
use peniko::Color;

#[derive(Clone, Debug)]
pub struct Animation {
    pub(crate) id: AnimId,
    pub(crate) state: AnimState,
    pub(crate) easing: Easing,
    pub(crate) fill_mode: FillMode,
    pub(crate) duration: Duration,
    pub(crate) repeat_mode: RepeatMode,
    pub(crate) repeat_count: usize,
    pub(crate) is_auto_reverse: bool,
    pub(crate) animated_props: HashMap<AnimPropKind, AnimatedProp>,
}

pub(crate) fn assert_valid_time(time: f64) {
    assert!(time >= 0.0 || time <= 1.0);
}

/// See [`Self::advance`].
#[derive(Clone, Debug, Copy)]
pub enum RepeatMode {
    // Once started, the animation will juggle between [`AnimState::PassInProgress`] and [`AnimState::PassFinished`],
    // but will never reach [`AnimState::Completed`]
    LoopForever,
    /// How many times do we repeat the animation?
    /// On every pass, we animate until `elapsed >= duration`, then we reset elapsed time to 0
    /// and increment `repeat_count`. This process is repeated until `repeat_count >= times`, and
    /// then the animation is set to [`AnimState::Completed`].
    Times(usize),
}

#[derive(Clone, Debug, Copy)]
/// Determines if the styles of the animation are frozen or removed once its it has completed.
pub enum FillMode {
    ///The styles applied by the animation are removed when the animation is completed.
    Removed,
    ///The styles applied by the animation remain visible after the animation is completed.
    Forwards,
}

pub(crate) fn next() -> Animation {
    Animation {
        id: AnimId::next(),
        state: AnimState::Idle,
        easing: Easing::default(),
        duration: Duration::from_secs(1),
        repeat_mode: RepeatMode::Times(1),
        fill_mode: FillMode::Removed,
        repeat_count: 0,
        is_auto_reverse: false,
        animated_props: HashMap::new(),
    }
}

#[derive(Debug, Clone)]
pub enum AnimUpdateMsg {
    Prop {
        id: AnimId,
        kind: AnimPropKind,
        val: AnimValue,
    },
}

#[derive(Clone, Debug)]
pub enum SizeUnit {
    Px,
    Pct,
}

#[derive(Debug, Clone, Copy)]
pub enum AnimDirection {
    Forward,
    Backward,
}

impl Animation {
    pub fn id(&self) -> AnimId {
        self.id
    }

    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    pub fn is_idle(&self) -> bool {
        matches!(self.state, AnimState::Idle)
    }

    pub fn is_in_progress(&self) -> bool {
        matches!(self.state, AnimState::InProgress)
    }

    pub fn is_completed(&self) -> bool {
        matches!(self.state, AnimState::Completed)
    }

    pub fn is_auto_reverse(&self) -> bool {
        self.is_auto_reverse
    }

    // pub fn is_playing_reverse(&self) -> bool {
    //     let mut elapsed = self.elapsed();
    //     let time = elapsed / self.duration.as_secs_f64();
    //     let time = self.easing.ease(time);
    //     time >= 0.5
    // }

    //TODO:
    // pub fn scale(self, scale_fn: impl Fn() -> f64 + 'static) -> Self {
    //     let cx = AppContext::get_current();
    //     create_effect(cx.scope, move |_| {
    //         let scale = scale_fn();
    //
    //         self.id
    //             .update_prop(AnimPropKind::Scale, AnimValue::Float(scale));
    //     });
    //
    //     self
    // }

    pub fn border_radius(self, border_radius_fn: impl Fn() -> f64 + 'static) -> Self {
        create_effect(move |_| {
            let border_radius = border_radius_fn();

            self.id
                .update_prop(AnimPropKind::BorderRadius, AnimValue::Float(border_radius));
        });

        self
    }

    pub fn color(self, color_fn: impl Fn() -> Color + 'static) -> Self {
        create_effect(move |_| {
            let color = color_fn();

            self.id
                .update_prop(AnimPropKind::Color, AnimValue::Color(color));
        });

        self
    }

    pub fn border_color(self, bord_color_fn: impl Fn() -> Color + 'static) -> Self {
        create_effect(move |_| {
            let border_color = bord_color_fn();

            self.id
                .update_prop(AnimPropKind::BorderColor, AnimValue::Color(border_color));
        });

        self
    }

    //TODO:
    // pub fn translate_x(self, translate_x_fn: impl Fn() -> f64 + 'static) -> Self {
    //     let cx = AppContext::get_current();
    //     create_effect(cx.scope, move |_| {
    //         let translate_x = translate_x_fn();
    //
    //         self.id
    //             .update_prop(AnimPropKind::TranslateX, AnimValue::Float(translate_x));
    //     });
    //
    //     self
    // }

    pub fn background(self, bg_fn: impl Fn() -> Color + 'static) -> Self {
        create_effect(move |_| {
            let background = bg_fn();

            self.id
                .update_prop(AnimPropKind::Background, AnimValue::Color(background));
        });

        self
    }

    pub fn width(self, width_fn: impl Fn() -> f64 + 'static) -> Self {
        create_effect(move |_| {
            let to_width = width_fn();

            self.id
                .update_prop(AnimPropKind::Width, AnimValue::Float(to_width));
        });

        self
    }

    pub fn height(self, height_fn: impl Fn() -> f64 + 'static) -> Self {
        create_effect(move |_| {
            let height = height_fn();

            self.id
                .update_prop(AnimPropKind::Height, AnimValue::Float(height));
        });

        self
    }

    pub fn fill_mode(mut self, fill_mode: FillMode) -> Self {
        self.fill_mode = fill_mode;
        self
    }

    pub fn auto_reverse(mut self, auto_reverse: bool) -> Self {
        self.is_auto_reverse = auto_reverse;
        self
    }

    pub fn repeat_forever(mut self, repeat: bool) -> Self {
        self.repeat_mode = if repeat {
            RepeatMode::LoopForever
        } else {
            RepeatMode::Times(1)
        };
        self
    }

    /// Repeats the animation for a specific number of times.
    pub fn repeat_count(mut self, times: usize) -> Self {
        self.repeat_mode = RepeatMode::Times(times);
        self
    }

    pub fn easing_fn(mut self, easing_fn: EasingFn) -> Self {
        self.easing.func = easing_fn;
        self
    }

    pub fn ease_mode(mut self, mode: EasingMode) -> Self {
        self.easing.mode = mode;
        self
    }

    pub fn ease_in(self) -> Self {
        self.ease_mode(EasingMode::In)
    }

    pub fn ease_out(self) -> Self {
        self.ease_mode(EasingMode::Out)
    }

    pub fn ease_in_out(self) -> Self {
        self.ease_mode(EasingMode::InOut)
    }

    pub fn begin(&mut self) {
        self.repeat_count = 0;
        self.state = AnimState::InProgress;
    }

    pub fn stop(&mut self) {
        match &mut self.state {
            AnimState::Idle | AnimState::Completed => {}
            AnimState::InProgress => self.state = AnimState::Completed,
        }
    }

    pub fn advance(&mut self) {
        match &mut self.state {
            AnimState::Idle => {
                self.begin();
            }
            AnimState::InProgress => {
                let mut can_advance = false;
                let repeat_mode = self.repeat_mode;
                let duration = self.duration.clone();

                for (_, prop) in self.props_mut() {
                    prop.advance(repeat_mode, &duration);
                    can_advance |= !prop.is_completed();
                }
                if !can_advance {
                    self.state = AnimState::Completed;
                }
            }
            AnimState::Completed => {}
        }
    }

    pub(crate) fn props(&self) -> &HashMap<AnimPropKind, AnimatedProp> {
        &self.animated_props
    }

    pub(crate) fn props_mut(&mut self) -> &mut HashMap<AnimPropKind, AnimatedProp> {
        self.animated_props.borrow_mut()
    }

    pub(crate) fn is_prop_playing_rev(&self, prop: &AnimatedProp) -> bool {
        if !self.is_auto_reverse() {
            return false;
        }

        let time = prop.elapsed.as_secs_f64() / self.duration.as_secs_f64();
        let time = self.easing.ease(time);
        time > 0.5
    }

    pub(crate) fn animate_prop(&self, prop: &AnimatedProp) -> AnimValue {
        let mut elapsed = prop.elapsed();

        if self.duration == Duration::ZERO {
            return prop.values.get_from();
        }

        if elapsed > self.duration {
            elapsed = self.duration;
        }

        let time = elapsed.as_secs_f64() / self.duration.as_secs_f64();
        let time = self.easing.ease(time);
        assert_valid_time(time);

        if self.is_auto_reverse() {
            if time > 0.5 {
                prop.values
                    .animate(time * 2.0 - 1.0, AnimDirection::Backward)
            } else {
                prop.values.animate(time * 2.0, AnimDirection::Forward)
            }
        } else {
            prop.values.animate(time, AnimDirection::Forward)
        }
    }

    pub(crate) fn requires_layout(&self) -> bool {
        self.animated_props.contains_key(&AnimPropKind::Width)
            || self.animated_props.contains_key(&AnimPropKind::Height)
    }

    pub(crate) fn can_advance(&self) -> bool {
        if self.is_auto_reverse && self.is_completed() {
            return false;
        }

        match self.fill_mode {
            FillMode::Removed => return !self.is_completed(),
            // Continue applying the style changes even if the animation is completed
            FillMode::Forwards => return true,
        }
    }
}
