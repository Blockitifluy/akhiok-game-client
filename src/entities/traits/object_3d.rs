//! Contains the traits `Object3D` and `Object3DSize`. Useful for handling transformations for
//! entities.
use crate::datatypes::vectors::Vector3;
use ultraviolet::{Mat4, Vec3};

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

    /// Gets the front.
    /// # Returns
    /// The _front_ vector (normalised)
    fn get_front(&self) -> Vector3;
    /// Set the front.
    /// # Arguements
    /// - `front`: the _front_ vector (normalised)
    fn set_front(&mut self, front: Vector3);

    /// Gets the right.
    /// # Returns
    /// The _right_ vector (normalised)
    fn get_right(&self) -> Vector3;
    /// Sets the right.
    /// # Arguements
    /// - `right`: the _right_ vector (normalised)
    fn set_right(&mut self, right: Vector3);

    /// Gets the up.
    /// # Returns
    /// The _up_ vector (normalised)
    fn get_up(&self) -> Vector3;
    /// Sets the up.
    /// # Arguements
    /// - `up`: The _up_ vector (normalised)
    fn set_up(&mut self, up: Vector3);

    /// Updates the `front`, `right` and `up` vector
    fn update_vectors(&mut self) {
        let rot = self.get_rotation();

        let (pitch, yaw) = (rot.y.to_radians(), rot.x.to_radians());
        let pitch_cos = pitch.cos();

        let front =
            Vector3::new(pitch_cos * yaw.cos(), pitch.sin(), pitch_cos * yaw.sin()).get_unit();
        let right = front.cross(Vector3::up()).get_unit();
        let up = right.cross(front).get_unit();

        self.set_front(front);
        self.set_right(right);
        self.set_up(up);
    }
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

    Mat4::from_translation(Vec3 {
        x: position.x,
        y: position.y,
        z: position.z,
    }) * Mat4::from_euler_angles(roll, pitch, yaw)
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
