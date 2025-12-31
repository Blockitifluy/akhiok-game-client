/// A vector with 3 axes; used to describe a 3D point.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Vector3 {
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
#[derive(Clone, Copy, Debug)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}
impl Vector2 {
    /// Creates a new vector.
    /// # Arguements
    /// - `x`: x axis
    /// - `y`: y axis
    /// # Returns
    /// A 2D vector
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Default for Vector2 {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}
