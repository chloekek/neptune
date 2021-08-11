use crate::Animated;
use crate::Context;
use crate::Control;
use crate::Event;
use crate::Property;

use np_graphics::Canvas;

pub struct Button
{
    label: Property<String>,
    background_color: Animated<[u8; 4]>,
    text_color: Animated<[u8; 4]>,
}

impl Control for Button
{
    type Pixel = [u8; 4];

    fn process(&mut self, c: &Context, event: &Event)
    {
    }

    fn draw(&self, c: &Context, target: &mut dyn Canvas<Pixel=Self::Pixel>)
    {
    }
}
