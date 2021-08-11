use crate::Animatable;
use crate::Context;
use crate::Timeline;

use std::time::Duration;
use std::time::Instant;

/// Property that can be animated.
pub struct Animated<T>
{
    timeline: Timeline<T>,
    // TODO: Timing function.
    start_time: Instant,
    duration: Duration,
}

impl<T> Animated<T>
{
    /// Set the property value using an animation.
    ///
    /// The given timeline is parameterized over the current property value.
    /// The timeline may use that value as the value of the initial keyframe,
    /// although this is not a requirement and is not checked.
    ///
    /// When this function returns,
    /// the animation is initiated,
    /// and the context is marked as dirty.
    pub fn set_timeline<F>(&mut self, c: &Context, timeline: F)
        where F: FnOnce(&T) -> Timeline<T>
    {
        let start = self.timeline.target();
        self.timeline = timeline(start);
        self.start_time = c.now();
        c.mark_as_dirty();
    }

    /// Set the property value without an animation.
    ///
    /// When this function returns,
    /// the animation is **not** initiated,
    /// and the context is marked as dirty.
    pub fn set_static(&mut self, c: &Context, value: T)
    {
        self.timeline = Timeline::r#static(value);
        c.mark_as_dirty();
    }
}

impl<T> Animated<T>
    where T: Copy
{
    /// Set the property value using a simple animation.
    ///
    /// The initial keyframe of the animation uses the current property value.
    /// For more control over the initial keyframe,
    /// use [`Animated::set_timeline`] instead.
    /// When this function returns,
    /// the animation is initiated,
    /// and the context is marked as dirty.
    pub fn set_simple(&mut self, c: &Context, target: T)
    {
        self.set_timeline(c, |&start| Timeline::simple(start, target))
    }
}

impl<T> Animated<T>
    where T: Animatable + Copy
{
    /// The property value as it is to be
    /// displayed according to the animation.
    ///
    /// When this function returns,
    /// if the animation is not yet finished,
    /// the context is marked as dirty.
    pub fn current(&self, c: &Context) -> T
    {
        let dt = self.start_time.saturating_duration_since(c.now());
        if dt >= self.duration {
            *self.timeline.target()
        } else {
            c.mark_as_dirty();
            let t = dt.as_secs_f64() / self.duration.as_secs_f64();
            self.timeline.evaluate(t)
        }
    }

    /// The property value when the animation is complete.
    pub fn target(&self) -> &T
    {
        self.timeline.target()
    }
}
