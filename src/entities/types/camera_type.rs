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

    front: Vector3,
    right: Vector3,
    up: Vector3,
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
            front: Vector3::forward(),
            right: Vector3::right(),
            up: Vector3::up(),
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

    fn get_front(&self) -> Vector3 {
        self.front
    }

    fn set_front(&mut self, front: Vector3) {
        self.front = front;
    }

    fn get_right(&self) -> Vector3 {
        self.right
    }

    fn set_right(&mut self, right: Vector3) {
        self.right = right;
    }

    fn get_up(&self) -> Vector3 {
        self.up
    }

    fn set_up(&mut self, up: Vector3) {
        self.up = up;
    }
}
