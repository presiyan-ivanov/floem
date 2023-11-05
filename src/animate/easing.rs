use std::f64::consts::PI;

use super::assert_valid_progress;

/// Alters how the easing function behaves, i.e. how the animation interpolates.
#[derive(Debug, Clone, Copy, Default)]
pub enum EasingMode {
    #[default]
    /// Interpolation follows the mathematical formula associated with the easing function.
    In,
    /// Interpolation follows 100% interpolation minus the output of the formula associated with the easing function.
    Out,
    /// Interpolation uses EasingMode::In for the first half of the animation and EasingMode::Out for the second half.
    InOut,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum EasingFn {
    #[default]
    Linear,
    /// Retracts the motion of an animation slightly before it begins to animate in the path indicated.
    Back,
    /// Creates a bouncing effect.
    Bounce,
    /// Creates an animation that accelerates and/or decelerates using a circular function.
    Circle,
    /// Creates an animation that resembles a spring oscillating back and forth until it comes to rest.
    Elastic,
    /// Creates an animation that accelerates and/or decelerates using an exponential formula.
    Exponential,
    /// Creates an animation that accelerates and/or
    /// decelerates using the formula `f(t) = tp` where p is equal to the Power property.
    Power,
    /// Creates an animation that accelerates and/or decelerates using the formula `f(t) = t2`.
    Quadratic,
    /// Creates an animation that accelerates and/or decelerates using the formula `f(t) = t3`.
    Cubic,
    /// Creates an animation that accelerates and/or decelerates using the formula `f(t) = t4`.
    Quartic,
    /// Create an animation that accelerates and/or decelerates using the formula `f(t) = t5`.
    Quintic,
    /// Creates an animation that accelerates and/or decelerates using a sine formula.
    Sine,
}

// See https://easings.net/ and
// https://learn.microsoft.com/en-us/dotnet/desktop/wpf/graphics-multimedia/easing-functions
#[derive(Debug, Clone, Default)]
pub struct Easing {
    pub(crate) mode: EasingMode,
    pub(crate) func: EasingFn,
}

fn elastic_easing(progress: f64) -> f64 {
    let c4: f64 = (2.0 * PI) / 3.0;
    if progress == 0.0 {
        0.0
    } else if (1.0 - progress).abs() < f64::EPSILON {
        1.0
    } else {
        -(2.0_f64.powf(10.0 * progress - 10.0) * ((progress * 10.0 - 10.75) * c4).sin())
    }
}

impl Easing {
    pub(crate) fn apply_easing_fn(&self, progress: f64) -> f64 {
        assert_valid_progress(progress);
        match self.func {
            EasingFn::Linear => progress,
            EasingFn::Circle => 1.0 - (1.0 - progress.powi(2)).sqrt(),
            EasingFn::Elastic => elastic_easing(progress),
            EasingFn::Exponential => {
                if progress == 0.0 {
                    0.0
                } else {
                    2.0f64.powf(10.0 * progress - 10.0)
                }
            }
            EasingFn::Power => todo!(),
            EasingFn::Quadratic => progress.powf(2.0),
            EasingFn::Cubic => progress.powf(3.0),
            EasingFn::Quartic => progress.powf(4.0),
            EasingFn::Quintic => progress.powf(5.0),
            EasingFn::Sine => 1.0 - ((progress * PI) / 2.0).cos(),
            EasingFn::Back => todo!(),
            EasingFn::Bounce => todo!(),
        }
    }

    pub(crate) fn ease(&self, progress: f64) -> f64 {
        assert_valid_progress(progress);
        match self.mode {
            EasingMode::In => self.apply_easing_fn(progress),
            EasingMode::Out => 1.0 - self.apply_easing_fn(1.0 - progress),
            EasingMode::InOut => {
                if progress < 0.5 {
                    self.apply_easing_fn(progress * 2.0) / 2.0
                } else {
                    1.0 - self.apply_easing_fn(2.0 - progress * 2.0) / 2.0
                }
            }
        }
    }
}
