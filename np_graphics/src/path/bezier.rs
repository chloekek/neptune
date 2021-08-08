use crate::Vector;

/// Compute the point at time \\( t \in [ 0, 1 ] \\)
/// along the given first-order Bézier curve.
pub fn bezier_linear(
    p0: Vector,
    p1: Vector,
    t: f64,
) -> Vector
{
    (1.0 - t) * p0 + t * p1
}

/// Compute the point at time \\( t \in [ 0, 1 ] \\)
/// along the given second-order Bézier curve.
pub fn bezier_quadratic(
    p0: Vector,
    p1: Vector,
    p2: Vector,
    t: f64,
) -> Vector
{
    bezier_linear(
        bezier_linear(p0, p1, t),
        bezier_linear(p1, p2, t),
        t,
    )
}

/// Compute the point at time \\( t \in [ 0, 1 ] \\)
/// along the given third-order Bézier curve.
pub fn bezier_cubic(
    p0: Vector,
    p1: Vector,
    p2: Vector,
    p3: Vector,
    t: f64,
) -> Vector
{
    bezier_linear(
        bezier_quadratic(p0, p1, p2, t),
        bezier_quadratic(p1, p2, p3, t),
        t,
    )
}
