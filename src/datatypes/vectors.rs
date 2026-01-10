//! Defines datatypes for vector, that describes a position. Stores:
//! - `Vector3`: A 3D position
//! - `Vector2`: A 2D position

use std::ops::{Add, Div, Mul, Neg, Sub};

/// A vector with 3 axes; used to describe a 3D point.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector3 {
    /// The x-axis
    pub x: f32,
    /// The y-axis
    pub y: f32,
    /// The z-axis
    pub z: f32,
}
impl Vector3 {
    /// A vector of 0.0, 0.0, 0.0
    pub const fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// A vector of 1.0, 1.0, 1.0
    pub const fn one() -> Self {
        Self {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }

    /// A vector of 1.0, 0.0, 0.0
    pub const fn right() -> Self {
        Self {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// A vector of 0.0, 1.0, 0.0
    pub const fn up() -> Self {
        Self {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        }
    }

    /// A vector of 0.0, 0.0, 1.0
    pub const fn forward() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        }
    }

    /// Creates a new vector.
    /// # Arguements
    /// - `x`: x axis
    /// - `y`: y axis
    /// - `z`: z axis
    /// # Returns
    /// A 3D vector
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Gets the cross product of 2 vectors.
    /// # Arguements
    /// - `other`: the second vector
    pub fn cross(self, other: Self) -> Self {
        Self {
            x: (self.y * other.z) - (self.z * other.y),
            y: (self.z * other.x) - (self.x * other.z),
            z: (self.x * other.y) - (self.y * other.x),
        }
    }

    /// Gets the dot product of 2 vectors.
    /// # Arguements
    /// - `other`: the second vector
    pub fn dot(self, other: Self) -> f32 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    /// Gets the magnitude of the vector.
    /// # Returns
    /// The magnitude
    pub fn get_magnitude(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Gets the unit vector (where the magnitude is equal to 1.0).
    /// # Returns
    /// The unit vector
    /// # Note
    /// If the vector's axes are all equal to 0.0, then it returns `zero()`.
    pub fn get_unit(self) -> Self {
        let zero = Self::zero();
        if self == zero {
            return zero;
        }

        self / self.get_magnitude()
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<Vector3> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<Vector3> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl Div<f32> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Default for Vector3 {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

/// A vector with 2 axes; used to describe a 2D point.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector2 {
    /// The x-axis
    pub x: f32,
    /// The y-axis
    pub y: f32,
}
impl Vector2 {
    /// A vector of 0.0, 0.0
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    /// A vector of 1.0, 1.0
    pub fn one() -> Self {
        Self { x: 1.0, y: 1.0 }
    }

    /// A vector of 1.0, 0.0
    pub fn right() -> Self {
        Self { x: 1.0, y: 0.0 }
    }

    /// A vector of 0.0, 1.0
    pub fn up() -> Self {
        Self { x: 0.0, y: 1.0 }
    }

    /// Creates a new vector.
    /// # Arguements
    /// - `x`: x axis
    /// - `y`: y axis
    /// # Returns
    /// A 2D vector
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Gets the dot product of 2 vectors.
    /// # Arguements
    /// - `other`: the second vector
    pub fn dot(&self, other: &Self) -> f32 {
        (self.x * other.x) + (self.y * other.y)
    }

    /// Get the magnitude of a vector.
    /// # Returns
    /// The magnitude
    pub fn get_magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Gets the unit vector (where the magnitude is equals to 1.0).
    /// # Returns
    /// the unit vector
    /// # Note
    /// If the vecot's axes are all equal to 0.0, then it returns `zero()`.
    pub fn get_unit(&self) -> Self {
        let zero = Self::zero();
        if *self == zero {
            return zero;
        }

        *self / self.get_magnitude()
    }
}

impl Add for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<Vector2> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Mul<f32> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Div<Vector2> for Vector2 {
    type Output = Vector2;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl Div<f32> for Vector2 {
    type Output = Vector2;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Neg for Vector2 {
    type Output = Vector2;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Default for Vector2 {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}
