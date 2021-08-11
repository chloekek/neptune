use crate::Context;

use std::fmt;
use std::ops::Deref;

/// Property that cannot be animated.
///
/// This is simply a wrapper around `T`,
/// but will mark the context as dirty whenever it is modified.
/// This ensures you don’t accidentally
/// neglect marking the context as dirty.
/// As such, this type implements [`Deref`] but not [`DerefMut`].
/// Obtaining a mutable reference to the value
/// would require a context to mark as dirty,
/// but no context is available in [`DerefMut::deref_mut`].
/// You may use [`Property::as_mut`] instead.
///
/// [`DerefMut`]: `std::ops::DerefMut`
/// [`DerefMut::deref_mut`]: `std::ops::DerefMut::deref_mut`
#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Property<T>
{
    value: T,
}

impl<T> Property<T>
{
    /// Wrap the given value as a property.
    ///
    /// When this function returns, the context is marked as dirty.
    pub fn new(c: &Context, value: T) -> Self
    {
        c.mark_as_dirty();
        Self::new_clean(value)
    }

    /// Wrap the given value as a property.
    pub fn new_clean(value: T) -> Self
    {
        Self{value}
    }

    /// Set the property value.
    ///
    /// When this function returns, the context is marked as dirty.
    pub fn set(&mut self, c: &Context, value: T)
    {
        c.mark_as_dirty();
        self.set_clean(value)
    }

    /// Set the property value.
    pub fn set_clean(&mut self, value: T)
    {
        self.value = value;
    }

    /// Mutable reference to the property value.
    ///
    /// When this function returns, the context is marked as dirty.
    pub fn as_mut(&mut self, c: &Context) -> &mut T
    {
        c.mark_as_dirty();
        self.as_mut_clean()
    }

    /// Mutable reference to the property value.
    pub fn as_mut_clean(&mut self) -> &mut T
    {
        &mut self.value
    }
}

impl<T> Deref for Property<T>
{
    type Target = T;

    fn deref(&self) -> &Self::Target
    {
        &self.value
    }
}

impl<T> fmt::Debug for Property<T>
    where T: fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        self.deref().fmt(f)
    }
}

impl<T> fmt::Display for Property<T>
    where T: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        self.deref().fmt(f)
    }
}
