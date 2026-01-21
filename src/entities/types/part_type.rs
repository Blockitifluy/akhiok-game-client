//! Contains the `PartType` entity which is used to make a visable object like a building block.

use derive_akhoik_ge::{Object3D, Object3DSize};
use ultraviolet::Mat4;

use crate::{
    datatypes::{color::Color3, vectors::Vector3},
    entities::{entity::EntityTrait, traits::object_3d::*},
    mesh::{Mesh, MeshParseError},
    texture::Texture,
};

/// The part entity type.
/// Used as a building block.
#[derive(Debug, Object3D, Object3DSize)]
pub struct Part {
    /// The mesh of the part
    mesh: Mesh,
    texture: Option<Texture>,
    /// The color assigned
    pub color: Color3,
    /// Is the the part visable to the renderer
    pub visable: bool,
    /// The transformation
    pub transform: Mat4,

    front: Vector3,
    right: Vector3,
    up: Vector3,
    /// The position
    position: Vector3,
    /// The euler rotation
    rotation: Vector3,
    /// The size of the part
    size: Vector3,
}
impl Part {
    /// Creates a new part.
    /// # Arguements
    /// - `mesh`: a borrowed mesh
    /// # Returns
    /// A `PartType`
    /// # Note
    /// This function clones `mesh`.
    pub fn new(mesh: &Mesh) -> Self {
        let mut construct = Self {
            mesh: mesh.clone(),
            visable: true,
            ..Default::default()
        };

        construct.recalculate_transform();
        construct
    }

    /// Gets the mesh of the part.
    /// # Returns
    /// The borrowed mesh
    pub fn get_mesh(&self) -> &Mesh {
        &self.mesh
    }

    /// Gets the mesh of the part as a mutable borrow.
    /// # Returns
    /// A mutable borrow of a mesh
    pub fn get_mut_mesh(&mut self) -> &mut Mesh {
        &mut self.mesh
    }

    /// Gets the texture of the part.
    /// # Returns
    /// Either:
    /// - The borrowed texture
    /// - `None`
    pub fn get_texture(&self) -> Option<&Texture> {
        let Some(texture) = &self.texture else {
            return None;
        };
        Some(texture)
    }

    /// Sets the texture of the part.
    /// # Arguements
    /// - `texture`: the new texture to be assigned
    pub fn set_texture(&mut self, mut texture: Texture) {
        texture.load_to_gl();
        self.texture = Some(texture);
    }

    /// Loads a new mesh for the part.
    /// # Arguement
    /// - `mesh`: a borrowed mesh
    pub fn load_mesh(&mut self, mesh: &Mesh) {
        let cloned_mesh = mesh.clone();
        self.mesh = cloned_mesh;
    }

    /// Loads a new mesh for the part from a file.
    /// # Arguements
    /// - `path`: the
    /// # Returns
    /// An result, either:
    /// - `()`
    /// - An error message
    /// # Note
    /// This is the same as the following:
    /// ```
    /// let mut part: PartType;
    /// let mesh = Mesh::load_mesh_from_file(path)?;
    /// part.load_mesh(mesh);
    /// ```
    pub fn load_mesh_from_file(&mut self, path: &str) -> Result<(), MeshParseError> {
        let mesh = Mesh::load_mesh_from_file(path)?;
        self.mesh = mesh;
        Ok(())
    }
}

impl EntityTrait for Part {}

impl Default for Part {
    fn default() -> Self {
        Self {
            mesh: Mesh::default(),
            texture: None,
            color: Color3::default(),
            position: Vector3::zero(),
            rotation: Vector3::zero(),
            transform: Mat4::identity(),
            visable: true,
            front: Vector3::forward(),
            right: Vector3::right(),
            up: Vector3::up(),
            size: Vector3::one(),
        }
    }
}
