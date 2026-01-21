//! Used for mesh creation and definition.

use core::fmt;
use std::{
    default::Default,
    error::Error,
    fs,
    io::{self, Read},
    vec::*,
};

use crate::datatypes::vectors::*;

/// An array of floats used in rendering vertices.
pub type VertexDataInternal = [f32; 5];

/// `VertexData` used to construct points on meshes, containing:
/// - `position` (the first 3 fields),
/// - `tex_coord` (the next 2 fields)
#[derive(Clone, Copy, Debug, Default)]
pub struct VertexData(f32, f32, f32, f32, f32);
impl VertexData {
    /// Creates a new vertex.
    /// # Arguements:
    /// - `position`: the vertex's position
    /// - `color` - the vertex color
    /// - `tex_coord` - the UV coordinates of the texture
    /// # Returns
    /// `VertexData`
    pub fn new(position: Vector3, tex_coord: Vector2) -> Self {
        Self(position.x, position.y, position.z, tex_coord.x, tex_coord.y)
    }

    /// Gets the position of the vertex.
    /// # Returns
    /// The vertex's position
    pub fn get_position(&self) -> Vector3 {
        Vector3::new(self.0, self.1, self.2)
    }

    /// Sets the position of the vertex.
    /// # Arguements
    /// - `pos`: the new position
    pub fn set_position(&mut self, pos: Vector3) {
        self.0 = pos.x;
        self.1 = pos.y;
        self.2 = pos.z;
    }

    /// Gets the texture coordinate of the vertex.
    /// # Returns
    /// The vertex's texture coordinate
    pub fn get_tex_coord(&self) -> Vector2 {
        Vector2::new(self.3, self.4)
    }

    /// Sets the texture coordinate of the vertex.
    /// # Arguements
    /// - `coord`: The new texture coordinate
    pub fn set_tex_coord(&mut self, coord: Vector2) {
        self.3 = coord.x;
        self.4 = coord.y;
    }

    /// Converts the vertex into an array of `f32`.
    /// # Returns
    /// A `f32` array with the following elements:
    /// - `position` (3),
    /// - `tex_coord` (2)
    pub fn to_internal(&self) -> VertexDataInternal {
        [self.0, self.1, self.2, self.3, self.4]
    }
}

/// The section of the mesh file
#[derive(PartialEq, Eq, Debug)]
pub enum MeshSectionType {
    /// Vertices
    Vertices,
    /// Indices
    Indices,
    /// Texture Coordinates
    TexCoord,
    /// None
    None,
}
impl MeshSectionType {
    /// Gets the `MeshSectionType` from the name.
    ///
    /// # Arguements
    /// - `name`: the name of the section type
    /// # Returns
    /// A section type enum, returns `MeshSectionType::None` if it's not valid.
    /// # Note
    /// The name needs to be exact e.g. _Vertices_ for the `Vertices` enum.
    fn from_name(name: &str) -> Self {
        match name {
            Mesh::VERTICES_SECTION_NAME => MeshSectionType::Vertices,
            Mesh::INDICES_SECTION_NAME => MeshSectionType::Indices,
            Mesh::TEXCOORD_SECTION_NAME => MeshSectionType::TexCoord,
            _ => MeshSectionType::None,
        }
    }
}

macro_rules! section_to_raw_fn {
    ($current_section:expr, $section_name:expr, $data:expr, $pos_data:expr, $ind_data:expr, $texcoord_data:expr) => {{
        match $current_section {
            MeshSectionType::Vertices => Self::load_raw_vertices($data.as_str(), &mut $pos_data),
            MeshSectionType::Indices => Self::load_raw_indices($data.as_str(), &mut $ind_data),
            MeshSectionType::TexCoord => {
                Self::load_raw_texcoord($data.as_str(), &mut $texcoord_data)
            }
            _ => Err(MeshParseError::InvalidSectionType($section_name.clone())),
        }
    }};
}
/// A collection of veretices and indices that defines the shape of  a object's surface,
#[derive(Clone, Debug, Default)]
pub struct Mesh {
    /// A vector of 3D points and other vector data.
    pub vertices: Vec<VertexData>,
    /// A vector of indices.
    /// # Example
    /// `[0, 1, 3, 1, 2, 3]`
    pub indices: Vec<u32>,
}
impl Mesh {
    /// Creates a new `Mesh` with the `vertices` and `indices` preset.
    /// # Arguements
    /// - `vertices`: A `vec` of `VertexData`
    /// - `indices`: A `vec` of `u32`
    /// # Returns
    /// A mesh with the vertices and indices set.
    pub fn with_set_data(vertices: Vec<VertexData>, indices: Vec<u32>) -> Self {
        Self { vertices, indices }
    }

    /// Create a new `Mesh` with the vertices and indices set.
    /// # Arguements
    /// - `v_size`: the size of the `vertices`
    /// - `i_size`: the size of the `indices`
    /// # Returns
    ///  A mesh with all the elements in the `vertices` and `indices` set to it's default values.
    pub fn with_capacity(v_size: usize, i_size: usize) -> Self {
        Self {
            vertices: Vec::with_capacity(v_size),
            indices: Vec::with_capacity(i_size),
        }
    }

    // Uses for parsing header in mesh files
    const SECTION_START_SYMBOL: char = ':';
    const VERTICES_SECTION_NAME: &str = "Vertices";
    const INDICES_SECTION_NAME: &str = "Indices";
    const TEXCOORD_SECTION_NAME: &str = "TexCoord";

    fn load_raw_vertices(inp: &str, out: &mut Vec<Vector3>) -> Result<(), MeshParseError> {
        let mut swap: u8 = 0; // 0 is x, 1 is y and 2 is z
        let (mut x, mut y) = (0.0, 0.0); // z is not need
        let mut num_b = String::with_capacity(8);

        for (i, c) in inp.chars().enumerate() {
            // only values allowed: numbers, '.', '-' and whitespace
            let is_whitespace = c.is_whitespace();
            let is_valid_num = c == '.' || c == '-' || c.is_numeric();
            if !is_whitespace && !is_valid_num {
                return Err(MeshParseError::InvalidSymbol {
                    at: i,
                    section: MeshSectionType::Vertices,
                });
            }

            if is_whitespace && !num_b.is_empty() {
                // compute
                let v_ex = num_b.parse::<f32>();
                let Ok(v) = v_ex else {
                    return Err(MeshParseError::InparsableValue {
                        at: i,
                        got: num_b,
                        inner: v_ex.unwrap_err().to_string(),
                    });
                };
                match swap {
                    0 => x = v,
                    1 => y = v,
                    2 => {
                        out.push(Vector3::new(x, y, v));
                    }
                    _ => panic!("internal error: swap not between 0 and 2"),
                }

                num_b.clear();
                swap = (swap + 1) % 3;
            } else {
                num_b.push(c);
            }
        }

        if swap != 0 {
            return Err(MeshParseError::ExcessValue {
                max: 3,
                data: num_b,
            });
        }
        Ok(())
    }

    fn load_raw_texcoord(inp: &str, out: &mut Vec<Vector2>) -> Result<(), MeshParseError> {
        let mut swap: bool = false; // false is u and true is v
        let mut u = 0.0; // v is not need
        let mut num_b = String::with_capacity(8);

        for (i, c) in inp.chars().enumerate() {
            // only values allowed: numbers, '.' and whitespace
            let is_whitespace = c.is_whitespace();
            let is_valid_num = c == '.' || c.is_numeric();
            if !is_whitespace && !is_valid_num {
                return Err(MeshParseError::InvalidSymbol {
                    at: i,
                    section: MeshSectionType::TexCoord,
                });
            }

            if is_whitespace && !num_b.is_empty() {
                // compute
                let v_ex = num_b.trim().parse::<f32>();
                let Ok(v) = v_ex else {
                    return Err(MeshParseError::InparsableValue {
                        at: i,
                        got: num_b,
                        inner: v_ex.unwrap_err().to_string(),
                    });
                };
                match swap {
                    false => u = v,
                    true => {
                        out.push(Vector2::new(u, v));
                    }
                }

                num_b.clear();
                swap = !swap;
            } else {
                num_b.push(c);
            }
        }

        if swap {
            return Err(MeshParseError::ExcessValue {
                max: 2,
                data: num_b,
            });
        }

        Ok(())
    }

    fn load_raw_indices(inp: &str, out: &mut Vec<u32>) -> Result<(), MeshParseError> {
        let mut num_b = String::with_capacity(8);

        for (i, c) in inp.chars().enumerate() {
            // only values allowed: numbers and whitespace
            let is_whitespace = c.is_whitespace();
            let is_valid_num = c.is_numeric();
            if !is_whitespace && !is_valid_num {
                return Err(MeshParseError::InvalidSymbol {
                    at: i,
                    section: MeshSectionType::Indices,
                });
            }

            if is_whitespace && !num_b.is_empty() {
                // compute
                let v_ex = num_b.parse::<u32>();
                let Ok(v) = v_ex else {
                    return Err(MeshParseError::InparsableValue {
                        at: i,
                        got: num_b,
                        inner: v_ex.unwrap_err().to_string(),
                    });
                };
                out.push(v);
                num_b.clear();
            } else {
                num_b.push(c);
            }
        }

        Ok(())
    }

    /// Creates a new mesh from mesh data.
    /// # Arguements
    /// - `b`: the mesh data
    /// # Returns
    /// Either:
    /// - `Ok`: A mesh based on the data
    /// - `Err`: An error message
    pub fn load_mesh(b: &str) -> Result<Self, MeshParseError> {
        let mut current_section = MeshSectionType::None;

        let mut data = String::with_capacity(512);
        let mut section_name = String::with_capacity(16);
        let mut first_section_occured = false; // this value is never set back to zero
        let mut looking_at_sect_start = false; // if false then we are reader data or the file has
        // just started

        let mut pos_data = Vec::<Vector3>::with_capacity(512);
        let mut ind_data = Vec::<u32>::with_capacity(128);
        let mut texcoord_data = Vec::<Vector2>::with_capacity(512);

        for c in b.chars() {
            if c == Self::SECTION_START_SYMBOL {
                if current_section != MeshSectionType::None {
                    section_to_raw_fn!(
                        current_section,
                        section_name,
                        data,
                        pos_data,
                        ind_data,
                        texcoord_data
                    )?
                }
                looking_at_sect_start = true;
                first_section_occured = true;
                section_name.clear();
                data.clear();
                continue;
            }

            if looking_at_sect_start {
                if c == '\n' {
                    // end of section
                    // evaluates the section type based on name
                    current_section = MeshSectionType::from_name(&section_name);
                    looking_at_sect_start = false;
                    continue;
                }
                section_name.push(c);
            } else if first_section_occured {
                data.push(c);
            }
        }

        // final eval
        if current_section != MeshSectionType::None {
            // evaluate section
            section_to_raw_fn!(
                current_section,
                section_name,
                data,
                pos_data,
                ind_data,
                texcoord_data
            )?
        }

        let mut vertex_data = Vec::<VertexData>::with_capacity(pos_data.len());
        for (i, pos) in pos_data.into_iter().enumerate() {
            let coord = *texcoord_data.get(i).unwrap_or(&Vector2::zero());
            vertex_data.push(VertexData::new(pos, coord));
        }

        Ok(Mesh::with_set_data(vertex_data, ind_data))
    }

    /// Creates a new from a file of mesh data.
    /// # Arguements
    /// - `path`: the path of the file
    /// # Returns
    /// Either:
    /// - `Ok`: A mesh based on the data
    /// - `Err`: An error message
    pub fn load_mesh_from_file(path: &str) -> Result<Self, MeshParseError> {
        let mut b = String::new();

        let f_ex = fs::File::open(path);
        let Ok(mut f) = f_ex else {
            return Err(MeshParseError::CouldntReadFile(f_ex.unwrap_err()));
        };
        if let Err(e) = f.read_to_string(&mut b) {
            return Err(MeshParseError::CouldntOpenFile(e));
        }

        Self::load_mesh(&b)
    }

    /// Adds a vertex to the mesh.
    /// # Arguements
    /// - `vd`: the vertex's data
    pub fn add_vertex_data(&mut self, vd: VertexData) {
        self.vertices.push(vd);
    }

    /// Adds a vertex to the mesh.
    /// # Arguements
    /// - `position`: the position of the vertex
    /// - `tex_coord`: the UV coordinates of the texture
    pub fn add_vertex_data_pt(&mut self, position: Vector3, tex_coord: Vector2) {
        let vd = VertexData::new(position, tex_coord);
        self.add_vertex_data(vd);
    }

    /// Adds a vertex to the mesh.
    /// # Arguements
    /// - `i`: the array's element index
    pub fn add_index(&mut self, i: u32) {
        self.indices.push(i);
    }

    /// Appends indices to the mesh.
    /// # Arguements
    /// - `indices`: A vecttor of indices
    pub fn add_indices(&mut self, indices: &mut Vec<u32>) {
        self.indices.append(indices);
    }

    /// Converts all of the vertices into `VertexDataInternal`.
    /// # Returns
    /// The conveted indices
    pub fn to_vertex_data_internal(&self) -> Vec<VertexDataInternal> {
        self.vertices.iter().map(|v| v.to_internal()).collect()
    }
}

/// Errors relating to mesh parsing.
#[derive(Debug)]
pub enum MeshParseError {
    /// Thrown when there is an unexpected symbol.
    InvalidSymbol {
        /// The index position of the unexpected symbol
        at: usize,
        /// The mesh section
        section: MeshSectionType,
    },
    /// Thrown when there is an unparsable symbol.
    InparsableValue {
        /// The index position of the symbol
        at: usize,
        /// The inparsable symbol
        got: String,
        /// The internal error
        inner: String,
    },
    /// Thrown when there has been too many values.
    ExcessValue {
        /// The maximum amount values expected
        max: u32,
        /// The excess value
        data: String,
    },
    /// Thrown when there has been an invalid section type.
    InvalidSectionType(String),
    /// Thrown when the mesh file couldn't be read.
    CouldntReadFile(io::Error),
    /// Thrown when the mesh file couldn't be opened.
    CouldntOpenFile(io::Error),
}

impl fmt::Display for MeshParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidSymbol { at, section } => {
                write!(f, "Invalid symbol at {at} in region {section:?}")
            }
            Self::InparsableValue { at, got, inner } => {
                write!(
                    f,
                    "Inparsable symbol at {at}, got {got} (internal error {inner})"
                )
            }
            Self::ExcessValue { data, max } => {
                write!(f, "Too many values with '{data}', maximum amount {max}")
            }
            Self::InvalidSectionType(section) => write!(f, "Invalid section name: {section}"),
            Self::CouldntReadFile(err) => write!(f, "couldn't read file: {err}"),
            Self::CouldntOpenFile(err) => write!(f, "couldn't open file: {err}"),
        }
    }
}

impl Error for MeshParseError {}
