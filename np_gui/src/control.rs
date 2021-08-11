use crate::Context;
use crate::Event;

use np_graphics::Canvas;

pub trait Control
{
    type Pixel: Copy;

    fn process(&mut self, c: &Context, event: &Event);

    fn draw(&self, c: &Context, target: &mut dyn Canvas<Pixel=Self::Pixel>);
}
