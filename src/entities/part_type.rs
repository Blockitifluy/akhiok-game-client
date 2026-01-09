//! Contains the `PartType` entity which is used to make a visable object like a building block.

use dispose::Disposable;
use ultraviolet::{Mat4, Vec3};

use crate::{
    datatypes::{color::Color3, vectors::Vector3},
    mesh::Mesh,
    texture::Texture,
};

/// The part entity type.
/// Used as a building block.
#[derive(Debug)]
pub struct PartType {
    /// The mesh of the part
    mesh: Mesh,
    texture: Option<Disposable<Texture>>,
    /// The color assigned
    pub color: Color3,
    /// Is the the part visable to the renderer
    pub visable: bool,
    /// The transformation
    pub transform: Mat4,

    /// The position
    position: Vector3,
    /// The euler rotation
    rotation: Vector3,
    /// The size of the part
    size: Vector3,
}
impl PartType {
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
            color: Color3::new(1.0, 1.0, 1.0).unwrap(),
            visable: true,
            position: Vector3::default(),
            rotation: Vector3::default(),
            size: Vector3::new(1.0, 1.0, 1.0),
            transform: Mat4::identity(),
            texture: None,
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
    pub fn get_texture(&self) -> Option<&Disposable<Texture>> {
        let Some(texture) = &self.texture else {
            return None;
        };
        Some(texture)
    }

    /// Sets the texture of the part.
    /// # Arguements
    /// - `texture`: the new texture to be assigned
    pub fn set_texture(&mut self, mut texture: Disposable<Texture>) {
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
    pub fn load_mesh_from_file(&mut self, path: &str) -> Result<(), String> {
        let mesh = Mesh::load_mesh_from_file(path)?;
        self.mesh = mesh;
        Ok(())
    }

    // Transformation

    fn calculate_transform(&self) -> Mat4 {
        let (roll, pitch, yaw) = (
            self.rotation.x.to_radians(),
            self.rotation.y.to_radians(),
            self.rotation.z.to_radians(),
        );

        Mat4::identity()
            * Mat4::from_translation(Vec3 {
                x: self.position.x,
                y: self.position.y,
                z: self.position.z,
            })
            * Mat4::from_euler_angles(roll, pitch, yaw)
            * Mat4::from_nonuniform_scale(Vec3 {
                x: self.size.x,
                y: self.size.y,
                z: self.size.z,
            })
    }

    fn recalculate_transform(&mut self) {
        self.transform = self.calculate_transform();
    }

    /// Gets the position.
    /// # Returns
    /// A position vector
    pub fn get_position(&self) -> Vector3 {
        self.position
    }

    /// Sets the position.
    /// # Arguement
    /// - `pos`: the new position
    pub fn set_position(&mut self, pos: Vector3) {
        self.position = pos;
        self.recalculate_transform();
    }

    /// Gets the rotation.
    /// # Returns
    /// An euler rotation
    pub fn get_rotation(&self) -> Vector3 {
        self.rotation
    }

    /// Sets the rotation.
    /// # Arguement
    /// - `rot`: the rotation euler
    pub fn set_rotation(&mut self, rot: Vector3) {
        self.rotation = rot;
        self.recalculate_transform();
    }

    /// Gets the size.
    /// # Arguement
    /// - `size`: the size
    pub fn get_size(&self) -> Vector3 {
        self.size
    }

    /// Sets the size.
    /// # Arguement
    /// - `size`: the size
    pub fn set_size(&mut self, size: Vector3) {
        self.size = size;
        self.recalculate_transform();
    }
}
