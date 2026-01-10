//! Contains the traits `Object3D` and `Object3DSize`. Useful for handling transformations for
//! entities.
use ultraviolet::{Mat4, Vec3};

use crate::datatypes::vectors::Vector3;

/// A trait for any 3D object with a position and rotation.
pub trait Object3D {
    /// Calculates the transformation of the object.
    fn calculate_transform(&self) -> Mat4;
    /// Calculates the transformation of the object and assigns the transform.
    fn recalculate_transform(&mut self);

    /// Gets the position.
    /// # Returns
    /// A position vector
    fn get_position(&self) -> Vector3;
    /// Sets the position.
    /// # Arguement
    /// - `pos`: the new position
    fn set_position(&mut self, pos: Vector3);

    /// Gets the rotation.
    /// # Returns
    /// An euler rotation
    fn get_rotation(&self) -> Vector3;
    /// Sets the rotation.
    /// # Arguement
    /// - `rot`: the rotation euler
    fn set_rotation(&mut self, rot: Vector3);
}

/// A trait for any 3D object with a size.
/// Usually paired with `Object3D`.
pub trait Object3DSize {
    /// Gets the size.
    /// # Arguement
    /// - `size`: the size
    fn get_size(&self) -> Vector3;
    /// Sets the size.
    /// # Arguement
    /// - `size`: the size
    fn set_size(&mut self, size: Vector3);
}

/// Calculates the transformation of the object.
/// # Arguements
/// - `obj`: the `Object3D`
/// # Returns
/// A Matrix4x4
pub fn calculate_transform<T: Object3D>(obj: &T) -> Mat4 {
    let rotation = obj.get_rotation();
    let position = obj.get_position();
    let (roll, pitch, yaw) = (
        rotation.x.to_radians(),
        rotation.y.to_radians(),
        rotation.z.to_radians(),
    );

    Mat4::identity()
        * Mat4::from_translation(Vec3 {
            x: position.x,
            y: position.y,
            z: position.z,
        })
        * Mat4::from_euler_angles(roll, pitch, yaw)
}

/// Calculates the transformation of the object with a size.
/// # Arguements
/// - `obj`: the `Object3D`
/// # Returns
/// A Matrix4x4
pub fn calculate_transform_with_size<T: Object3DSize + Object3D>(obj: &T) -> Mat4 {
    let size = obj.get_size();
    let base_transform = calculate_transform(obj);
    base_transform
        * Mat4::from_nonuniform_scale(Vec3 {
            x: size.x,
            y: size.y,
            z: size.z,
        })
}
