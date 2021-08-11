use np_graphics::Vector;

pub trait Animatable
{
    fn interpolate(start: &Self, target: &Self, t: f64) -> Self;
}

impl Animatable for f32
{
    fn interpolate(start: &Self, target: &Self, t: f64) -> Self
    {
        (1.0 - t as f32) * start + t as f32 * target
    }
}

impl Animatable for f64
{
    fn interpolate(start: &Self, target: &Self, t: f64) -> Self
    {
        (1.0 - t) * start + t * target
    }
}

impl Animatable for Vector
{
    fn interpolate(start: &Self, target: &Self, t: f64) -> Self
    {
        Vector{
            x: f64::interpolate(&start.x, &target.x, t),
            y: f64::interpolate(&start.y, &target.y, t),
        }
    }
}
