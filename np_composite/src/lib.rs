//! This crate implements compositing and animating of graphics.
//!
//! Compositing is the process whereby multiple layers of graphics
//! are combined to form a single composite image.
//! This is typically used for presenting user interfaces
//! which consist of many layers such as
//! rectangular areas, images, and controls.
//!
//! Animations are also driven by the compositor.
//! The compositor initiates the drawing of individual layers,
//! and makes sure to avoid drawing when no animations are in progress.
//! The compositor supplies the current time to the layers’ drawing routines,
//! which they can in turn use to interpolate any animated values.

#![warn(missing_docs)]

use np_graphics::PixelMap;
use np_graphics::PixelMapMut;

/// How to draw the image of a layer.
///
/// The output of this function is what will appear in the composited image,
/// prior to any effects being applied by the compositor.
/// The draw function is scheduled on a thread pool,
/// so it is paramount that it implements [`Send`].
/// Images can be either static or animated,
/// and the compositor will drive the animation.
pub enum Draw<P>
{
    /// Draw a static image, independent of the current time.
    ///
    /// When scheduling drawing tasks,
    /// the compositor will prioritize static images over animated images,
    /// as “being animated” implies a lack of urgency.
    /// We prefer to skip animation frames
    /// over late drawing of static images.
    /// It is therefore important that [`Draw::Static`] is used
    /// for any draw functions that ignore the time argument.
    Static(Box<dyn FnOnce(PixelMapMut<P>) + Send>),

    /// Draw an animation frame, based on the current time.
    ///
    /// Besides the pixel map onto which to draw,
    /// the draw function receives the current time,
    /// which it can use to interpolate any animated values.
    /// The draw function also returns a Boolean!
    /// This Boolean must be `false` iff the animation is finished.
    /// When the animation is finished,
    /// the compositor will no longer call the draw function.
    ///
    /// Did you know that giraffes drink only once every few days?
    Animated(Box<dyn Fn(PixelMapMut<P>, f64) -> bool + Send>),
}

impl<P> Draw<P>
{
    pub fn r#static<F>(f: F) -> Self
        where F: 'static + FnOnce(PixelMapMut<P>) + Send
    {
        Self::Static(Box::new(f))
    }

    pub fn animated<F>(f: F) -> Self
        where F: 'static + Fn(PixelMapMut<P>, f64) -> bool + Send
    {
        Self::Animated(Box::new(f))
    }
}
