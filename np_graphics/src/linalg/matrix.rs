use crate::Vector;

use std::ops;

/// Affine transformation matrix.
///
/// Affine transformation matrices are used for
/// rotating, scaling, skewing, and translating points.
/// The final column of the matrix is not stored in memory
/// as its components are the same for every matrix.
///
/// \\(
/// \begin{bmatrix}
/// a   & b   & 0 \\\\
/// c   & d   & 0 \\\\
/// t_x & t_y & 1
/// \end{bmatrix}
/// \\)
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug)]
pub struct Matrix
{
    pub a:  f64,
    pub b:  f64,
    pub c:  f64,
    pub d:  f64,
    pub tx: f64,
    pub ty: f64,
}

impl Matrix
{
    /// The identity matrix, which provides no transformations.
    ///
    /// \\(
    /// \begin{bmatrix}
    /// 1 & 0 & 0 \\\\
    /// 0 & 1 & 0 \\\\
    /// 0 & 0 & 1
    /// \end{bmatrix}
    /// \\)
    pub const IDENTITY: Self = Self{
        a:  1.0,
        b:  0.0,
        c:  0.0,
        d:  1.0,
        tx: 0.0,
        ty: 0.0,
    };

    /// A matrix which rotates the shape a given angle.
    ///
    /// \\(
    /// \begin{bmatrix}
    ///    \cos \theta & \sin \theta & 0 \\\\
    /// \- \sin \theta & \cos \theta & 0 \\\\
    /// 0              & 0           & 1
    /// \end{bmatrix}
    /// \\)
    pub fn from_rotate(theta: f64) -> Self
    {
        let (sin, cos) = theta.sin_cos();
        Self{a: cos, b: sin, c: -sin, d: cos, tx: 0.0, ty: 0.0}
    }

    /// A matrix which scales the shape
    /// a given factor in each direction.
    ///
    /// \\(
    /// \begin{bmatrix}
    /// s_x & 0   & 0 \\\\
    /// 0   & s_y & 0 \\\\
    /// 0   & 0   & 1
    /// \end{bmatrix}
    /// \\)
    pub fn from_scale(sx: f64, sy: f64) -> Self
    {
        Self{a: sx, b: 0.0, c: 0.0, d: sy, tx: 0.0, ty: 0.0}
    }

    /// A matrix which translates the shape
    /// a given distance in each direction.
    ///
    /// \\(
    /// \begin{bmatrix}
    /// 1   & 0   & 0 \\\\
    /// 0   & 1   & 0 \\\\
    /// t_x & t_y & 1
    /// \end{bmatrix}
    /// \\)
    pub fn from_translate(tx: f64, ty: f64) -> Self
    {
        Self{a: 1.0, b: 0.0, c: 0.0, d: 1.0, tx, ty}
    }
}

impl ops::Mul<Matrix> for Matrix
{
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Self::Output
    {
        Matrix{
            a:  self.a * rhs.a  + self.c * rhs.b,
            b:  self.b * rhs.a  + self.d * rhs.b,
            c:  self.a * rhs.c  + self.c * rhs.d,
            d:  self.b * rhs.c  + self.d * rhs.d,
            tx: self.a * rhs.tx + self.c * rhs.ty + self.tx,
            ty: self.b * rhs.tx + self.d * rhs.ty + self.ty,
        }
    }
}

impl ops::Mul<Vector> for Matrix
{
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output
    {
        Vector{
            x: self.a * rhs.x + self.c * rhs.y + self.tx,
            y: self.b * rhs.x + self.d * rhs.y + self.ty,
        }
    }
}
