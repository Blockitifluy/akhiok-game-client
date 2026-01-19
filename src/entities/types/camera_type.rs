//! Contains the `CameraType` entity variant

use crate::{
    datatypes::vectors::Vector3,
    entities::{entity::EntityTrait, traits::object_3d::*},
};
use derive_akhoik_ge::Object3D;
use ultraviolet::{Mat4, projection::perspective_gl};

/// A camera used for rendering
#[derive(Debug, Object3D)]
pub struct Camera {
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
impl Camera {
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

impl EntityTrait for Camera {}
