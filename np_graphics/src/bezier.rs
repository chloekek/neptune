use crate::Vector;

/// Linear, conic, or cubic Bézier curve.
///
/// The variants’ arguments are
/// the control points of the Bézier curve
/// in the order in which they appear.
#[derive(Clone, Copy, Debug)]
pub enum Bezier
{
    /// \\( B(P_0, P_1, t) = (1 - t) P_0 + t P_1 \\)
    Linear(Vector, Vector),

    /// \\( B(P_0, P_1, P_2, t) = B(B(P_0, P_1), B(P_1, P_2), t) \\)
    Conic(Vector, Vector, Vector),

    /// \\( B(P_0, P_1, P_2, P_3, t) = B(B(P_0, P_1, P_2), B(P_1, P_2, P_3), t) \\)
    Cubic(Vector, Vector, Vector, Vector),
}

impl Bezier
{
    /// Point at \\( t \in [0, 1] \\) along the Bézier curve.
    ///
    /// The exact formulae used are documented
    /// on the individual variants of [`Bezier`].
    pub fn evaluate(&self, t: f64) -> Vector
    {
        match *self {
            Self::Linear(p0, p1) => Self::linear(p0, p1, t),
            Self::Conic(p0, p1, p2) => Self::conic(p0, p1, p2, t),
            Self::Cubic(p0, p1, p2, p3) => Self::cubic(p0, p1, p2, p3, t),
        }
    }

    #[inline(always)]
    fn linear(p0: Vector, p1: Vector, t: f64) -> Vector
    {
        (1.0 - t) * p0 + t * p1
    }

    #[inline(always)]
    fn conic(p0: Vector, p1: Vector, p2: Vector, t: f64) -> Vector
    {
        Self::linear(
            Self::linear(p0, p1, t),
            Self::linear(p1, p2, t),
            t,
        )
    }

    #[inline(always)]
    fn cubic(p0: Vector, p1: Vector, p2: Vector, p3: Vector, t: f64) -> Vector
    {
        Self::linear(
            Self::conic(p0, p1, p2, t),
            Self::conic(p1, p2, p3, t),
            t,
        )
    }
}
