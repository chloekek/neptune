use std::cell::Cell;
use std::time::Instant;

pub struct Context
{
    now: Instant,
    dirty: Cell<bool>,
}

impl Context
{
    pub fn now(&self) -> Instant
    {
        self.now
    }

    pub fn mark_as_dirty(&self)
    {
        self.dirty.set(true);
    }
}
