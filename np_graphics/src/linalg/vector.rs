use std::ops;

/// General-purpose two-element vector.
///
/// Vectors are used for specifying
/// points, sizes, and distances
/// in two dimensions.
///
/// \\(
/// \begin{bmatrix}
/// x \\\\
/// y
/// \end{bmatrix}
/// \\)
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug)]
pub struct Vector
{
    pub x: f64,
    pub y: f64,
}

impl ops::Add<Vector> for Vector
{
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output
    {
        Vector{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub<Vector> for Vector
{
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output
    {
        Vector{
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Mul<Vector> for f64
{
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output
    {
        Vector{
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}
