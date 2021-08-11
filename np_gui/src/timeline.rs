use crate::Animatable;

/// Collection of animation keyframes.
///
/// A keyframe declares the value of a property
/// at a given time \\( t \in [ 0, 1 ] \\).
/// A timeline is composed of a sequence of keyframes.
/// The animation proceeds by interpolating the value
/// between the nearest two keyframes scheduled
/// around the current time.
/// After the final keyframe is reached,
/// the animation will no longer proceed
/// and the value stays where it is.
pub struct Timeline<T>
{
    inner: Inner<T>,
}

enum Inner<T>
{
    Static(T),
    Simple(T, T),
    // TODO: Custom keyframes.
}

impl<T> Timeline<T>
{
    /// Create a timeline with just one keyframe.
    ///
    /// This does not really animate and evaluates to
    /// the same value regardless of the time.
    pub fn r#static(value: T) -> Self
    {
        let inner = Inner::Static(value);
        Self{inner}
    }

    /// Create a timeline with just two keyframes.
    ///
    /// The keyframes are positioned at times \\( 0 \\) and \\( 1 \\).
    pub fn simple(start: T, target: T) -> Self
    {
        let inner = Inner::Simple(start, target);
        Self{inner}
    }

    /// The value animated to when reaching the final keyframe.
    pub fn target(&self) -> &T
    {
        match &self.inner {
            Inner::Static(value) => value,
            Inner::Simple(_start, target) => target,
        }
    }

    /// The value animated to when reaching the final keyframe.
    pub fn into_target(self) -> T
    {
        match self.inner {
            Inner::Static(value) => value,
            Inner::Simple(_start, target) => target,
        }
    }
}

impl<T> Timeline<T>
    where T: Animatable + Copy
{
    /// The value animated to at the given time.
    ///
    /// This function performs a linear interpolation.
    /// To perform non-linear interpolation using a timing function,
    /// adjust the \\( t \\) parameter prior to calling this method.
    pub fn evaluate(&self, t: f64) -> T
    {
        match &self.inner {
            Inner::Static(value) => *value,
            Inner::Simple(start, target) =>
                T::interpolate(start, target, t),
        }
    }
}
