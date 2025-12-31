use std::{default::Default, fs, io::Read, vec::*};

use crate::datatypes::{color::*, vectors::*};

pub type VertexDataInternal = [f32; 8];
pub type TriIndexes = [u32; 3];

/// `VertexData` used to construct points on meshes, containing:
/// - `position`,
/// - `color` and
/// - `tex_coord`
#[derive(Clone, Copy, Debug, Default)]
pub struct VertexData {
    /// The vertex's position
    pub position: Vector3,
    /// The vertex color
    pub color: Color3,
    /// the UV coordinates of the texture
    pub tex_coord: Vector2,
}
impl VertexData {
    /// Creates a new vertex.
    /// # Arguements:
    /// - `position`: the vertex's position
    /// - `color` - the vertex color
    /// - `tex_coord` - the UV coordinates of the texture
    /// # Returns
    /// `VertexData`
    pub fn new(position: Vector3, color: Color3, tex_coord: Vector2) -> Self {
        Self {
            position,
            color,
            tex_coord,
        }
    }

    /// Converts the vertex into an array of `f32`.
    /// # Returns
    /// A `f32` array with the following elements:
    /// - `position` (3),
    /// - `color` (3, normalised),
    /// - `tex_coord` (2)
    pub fn to_internal(&self) -> VertexDataInternal {
        [
            self.position.x,
            self.position.y,
            self.position.z,
            self.color.r,
            self.color.g,
            self.color.b,
            self.tex_coord.x,
            self.tex_coord.y,
        ]
    }
}

#[derive(PartialEq, Eq, Debug)]
enum MeshSectionType {
    Vertices,
    Indices,
    Color,
    TexCoord,
    None,
}

#[derive(Clone, Debug)]
pub struct Mesh {
    pub vertices: Vec<VertexData>,
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
    const COLOR_SECTION_NAME: &str = "Color";
    const TEXCOORD_SECTION_NAME: &str = "TexCoord";

    fn load_raw_vertices(inp: &str, out: &mut Vec<Vector3>) -> Result<(), String> {
        let mut swap: u8 = 0; // 0 is x, 1 is y and 2 is z
        let (mut x, mut y) = (0.0, 0.0); // z is not need
        let mut num_b = String::with_capacity(8);

        for (i, c) in inp.chars().enumerate() {
            // only values allowed: numbers, '.', '-' and whitespace
            let is_whitespace = c.is_whitespace();
            let is_valid_num = c == '.' || c == '-' || c.is_numeric();
            if !is_whitespace && !is_valid_num {
                return Err(format!(
                    "invalid data in vertices: invalid character at {}",
                    i
                ));
            }

            if is_whitespace && !num_b.is_empty() {
                // compute
                let v_ex = num_b.parse::<f32>();
                let Ok(v) = v_ex else {
                    return Err(format!(
                        "couldn't parse value at {}: invalid floating point value ({})",
                        i, num_b
                    ));
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

        Ok(())
    }

    fn load_raw_texcoord(inp: &str, out: &mut Vec<Vector2>) -> Result<(), String> {
        let mut swap: bool = false; // false is u and true is v
        let mut u = 0.0; // v is not need
        let mut num_b = String::with_capacity(8);

        for (i, c) in inp.chars().enumerate() {
            // only values allowed: numbers, '.' and whitespace
            let is_whitespace = c.is_whitespace();
            let is_valid_num = c == '.' || c.is_numeric();
            if !is_whitespace && !is_valid_num {
                return Err(format!(
                    "invalid data in texcoord: invalid character at {}",
                    i
                ));
            }

            if is_whitespace && !num_b.is_empty() {
                // compute
                let v_ex = num_b.trim().parse::<f32>();
                let Ok(v) = v_ex else {
                    return Err(format!(
                        "couldn't parse value at {}: invalid floating point value ({})",
                        i, num_b
                    ));
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

        Ok(())
    }

    fn load_raw_color(inp: &str, out: &mut Vec<Color3>) -> Result<(), String> {
        let mut swap: u8 = 0; // 0 is r, 1 is b and 2 is g
        let (mut r, mut g) = (0, 0); // b is not need
        let mut num_b = String::with_capacity(8);

        for (i, c) in inp.chars().enumerate() {
            // only values allowed: numbers, '.', '-' and whitespace
            let is_whitespace = c.is_whitespace();
            let is_valid_num = c == '.' || c == '-' || c.is_numeric();
            if !is_whitespace && !is_valid_num {
                return Err(format!("invalid data in color: invalid character at {}", i));
            }

            if is_whitespace && !num_b.is_empty() {
                // compute
                let v_ex = num_b.parse::<u8>();
                let Ok(v) = v_ex else {
                    return Err(format!(
                        "couldn't parse value at {}: invalid integer value ({})",
                        i, num_b
                    ));
                };
                match swap {
                    0 => r = v,
                    1 => g = v,
                    2 => {
                        out.push(Color3::from_rgb(r, g, v));
                    }
                    _ => panic!("internal error: swap not between 0 and 2"),
                }

                num_b.clear();
                swap = (swap + 1) % 3;
            } else {
                num_b.push(c);
            }
        }

        Ok(())
    }

    fn load_raw_indices(inp: &str, out: &mut Vec<u32>) -> Result<(), String> {
        let mut num_b = String::with_capacity(8);

        for (i, c) in inp.chars().enumerate() {
            // only values allowed: numbers and whitespace
            let is_whitespace = c.is_whitespace();
            let is_valid_num = c.is_numeric();
            if !is_whitespace && !is_valid_num {
                return Err(format!(
                    "invalid data in indices: invalid character at {}",
                    i
                ));
            }

            if is_whitespace && !num_b.is_empty() {
                // compute
                let v_ex = num_b.parse::<u32>();
                let Ok(v) = v_ex else {
                    return Err(format!(
                        "couldn't parse value {}: invalid integer value ({})",
                        i, num_b
                    ));
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
    pub fn load_mesh(b: &str) -> Result<Self, String> {
        let mut current_section = MeshSectionType::None;

        let mut data = String::with_capacity(512);
        let mut section_name = String::with_capacity(16);
        let mut first_section_occured = false; // this value is never set back to zero
        let mut looking_at_sect_start = false; // if false then we are reader data or the file has
        // just started

        let mut pos_data = Vec::<Vector3>::with_capacity(512);
        let mut ind_data = Vec::<u32>::with_capacity(128);
        let mut color_data = Vec::<Color3>::with_capacity(512);
        let mut texcoord_data = Vec::<Vector2>::with_capacity(512);

        for c in b.chars() {
            if c == Self::SECTION_START_SYMBOL {
                if current_section != MeshSectionType::None {
                    // evaluate section
                    let res = match current_section {
                        MeshSectionType::Vertices => {
                            Self::load_raw_vertices(data.as_str(), &mut pos_data)
                        }
                        MeshSectionType::Indices => {
                            Self::load_raw_indices(data.as_str(), &mut ind_data)
                        }
                        MeshSectionType::Color => {
                            Self::load_raw_color(data.as_str(), &mut color_data)
                        }
                        MeshSectionType::TexCoord => {
                            Self::load_raw_texcoord(data.as_str(), &mut texcoord_data)
                        }
                        _ => Err(format!("invalid section type: {:?}", current_section)),
                    };

                    res?
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
                    current_section = {
                        match section_name.as_str() {
                            Self::VERTICES_SECTION_NAME => MeshSectionType::Vertices,
                            Self::INDICES_SECTION_NAME => MeshSectionType::Indices,
                            Self::COLOR_SECTION_NAME => MeshSectionType::Color,
                            Self::TEXCOORD_SECTION_NAME => MeshSectionType::TexCoord,
                            _ => MeshSectionType::None,
                        }
                    };
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
            let res = match current_section {
                MeshSectionType::Vertices => Self::load_raw_vertices(data.as_str(), &mut pos_data),
                MeshSectionType::Indices => Self::load_raw_indices(data.as_str(), &mut ind_data),
                MeshSectionType::Color => Self::load_raw_color(data.as_str(), &mut color_data),
                MeshSectionType::TexCoord => {
                    Self::load_raw_texcoord(data.as_str(), &mut texcoord_data)
                }
                _ => Err(format!("invalid section type: {:?}", current_section)),
            };

            res?
        }

        let pos_len = pos_data.len();

        let mut vertex_data = Vec::<VertexData>::with_capacity(pos_len);
        for (i, pos) in pos_data.iter().enumerate() {
            vertex_data.push(VertexData::new(
                *pos,
                *color_data.get(i).unwrap_or(&Color3 {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                }),
                *texcoord_data.get(i).unwrap_or(&Vector2::new(0.0, 0.0)),
            ));
        }

        Ok(Mesh {
            vertices: vertex_data,
            indices: ind_data,
        })
    }

    /// Creates a new from a file of mesh data.
    /// # Arguements
    /// - `path`: the path of the file
    /// # Returns
    /// Either:
    /// - `Ok`: A mesh based on the data
    /// - `Err`: An error message
    pub fn load_mesh_from_file(path: &str) -> Result<Self, String> {
        let mut b = String::new();

        let f_ex = fs::File::open(path);
        let Ok(mut f) = f_ex else {
            return Err(format!("couldn't open file {}", f_ex.unwrap_err()));
        };
        if let Err(e) = f.read_to_string(&mut b) {
            return Err(format!("couldn't read file {}", e));
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
    /// - `color`: the vertex color
    /// - `tex_coord`: the UV coordinates of the texture
    pub fn add_vertex_data_pct(&mut self, position: Vector3, color: Color3, tex_coord: Vector2) {
        let vd = VertexData::new(position, color, tex_coord);
        self.add_vertex_data(vd)
    }

    /// Adds a vertex to the mesh.
    /// # Arguements
    /// - `position`: the position of the vertex
    /// - `tex_coord`: the UV coordinates of the texture
    pub fn add_vertex_data_pt(&mut self, position: Vector3, tex_coord: Vector2) {
        let vd = VertexData::new(position, Color3::new(1.0, 1.0, 1.0).unwrap(), tex_coord);
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

    /// Converts the indices into pairs of 3.
    /// # Returns
    /// Indices seperated into 3 pairs
    pub fn to_indices_tri(&self) -> Vec<TriIndexes> {
        let mut tri = Vec::<TriIndexes>::with_capacity(self.indices.len() / 3);
        let mut swap = 0_u8;
        let (mut a, mut b) = (0_u32, 0_u32);

        for index in &self.indices {
            match swap {
                0 => a = *index,
                1 => b = *index,
                2 => {
                    tri.push([a, b, *index]);
                }
                _ => panic!("internal error: swap out of range"),
            }
            swap = (swap + 1) % 3;
        }
        tri
    }
}
