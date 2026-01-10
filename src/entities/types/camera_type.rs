//! Contains the `CameraType` entity variant

use ultraviolet::{Mat4, projection::perspective_gl};

use crate::{datatypes::vectors::Vector3, entities::traits::object_3d::*};

/// A camera used for rendering
#[derive(Debug)]
pub struct CameraType {
    /// The vertical field of view
    pub fov: f32,
    /// The transform of the camera
    pub transform: Mat4,

    /// How close an vertex can be until it wont't be rendered
    pub near_view: f32,
    /// How far an vertex can be until it won't be rendered
    pub far_view: f32,

    position: Vector3,
    rotation: Vector3,
}
impl CameraType {
    /// Create a new `CameraType`.
    /// # Arguements
    /// - `fov`: the vertical field of view
    /// - `near_view`: how close an vertex can be until it won't be rendered
    /// - `far_view`: how far an vertex can be until it won't be rendered
    /// # Returns
    /// A new `CameraType`
    pub fn new(fov: f32, near_view: f32, far_view: f32) -> Self {
        let mut new = Self {
            fov,
            transform: Mat4::default(),
            near_view,
            far_view,
            position: Vector3::zero(),
            rotation: Vector3::zero(),
        };

        new.recalculate_transform();
        new
    }

    /// Gets the perspective projection of the camera
    /// # Arguements
    /// - `aspect_ratio`: the aspect ratio of the screen
    /// # Returns
    /// A projection matrix
    pub fn get_projection(&self, aspect_ratio: f32) -> Mat4 {
        perspective_gl(self.fov, aspect_ratio, self.near_view, self.far_view)
    }
}

impl Object3D for CameraType {
    fn calculate_transform(&self) -> Mat4 {
        calculate_transform(self)
    }

    fn recalculate_transform(&mut self) {
        self.transform = calculate_transform(self);
    }

    fn get_position(&self) -> Vector3 {
        self.position
    }

    fn set_position(&mut self, pos: Vector3) {
        self.position = pos;
        self.recalculate_transform();
    }

    fn get_rotation(&self) -> Vector3 {
        self.rotation
    }

    fn set_rotation(&mut self, rot: Vector3) {
        self.rotation = rot;
    }
}
