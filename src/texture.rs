//! Used for the creation and defination of textures. Used in rendering images on meshes.
use std::{fs, io, ptr::null_mut};

use ogl33::glGenBuffers;

/// A texture usable inside of the engine.
#[derive(Debug, Clone)]
pub struct Texture {
    /// The images's width
    pub width: i32,
    /// The image's height
    pub height: i32,
    /// The image data
    pub pixels: *mut u8,
    /// The colour space of the image
    pub comp: i32,
    /// The gl buffer
    pub texture_id: u32,
}
impl Texture {
    /// Make a texture from a byte vector
    /// # Arguements
    /// - `data`: a byte vector representing a image
    /// # Returns
    /// A new texture
    pub fn new(mut data: Vec<u8>) -> Self {
        let mut texture = Self {
            width: 0,
            height: 0,
            pixels: null_mut(),
            comp: 0,
            texture_id: 0,
        };

        unsafe {
            stb_image_rust::stbi_set_flip_vertically_on_load(true as i32);
            texture.pixels = stb_image_rust::stbi_load_from_memory(
                data.as_mut_ptr(),
                data.len() as i32,
                &mut texture.width,
                &mut texture.height,
                &mut texture.comp,
                stb_image_rust::STBI_rgb_alpha,
            );
        }

        texture
    }

    /// Loads the texture to gl
    pub fn load_to_gl(&mut self) {
        unsafe {
            glGenBuffers(1, &mut self.texture_id);
        }
    }

    /// Reads the texture file to an texture that would be usable inside the engine.
    /// # Arguements
    /// - `path`: the file's path
    /// # Returns
    /// Either:
    /// - `Ok`: A new texture
    /// - `Err`: An error message
    pub fn from_file(path: &str) -> Result<Self, &'static str> {
        let f_ex = fs::File::open(path);
        let Ok(mut f) = f_ex else {
            return Err("couldn't load texture");
        };

        let mut data = vec![];

        if io::Read::read_to_end(&mut f, &mut data).is_err() {
            return Err("couldn't read texture");
        }

        Ok(Self::new(data))
    }

    /// Frees the texture.
    pub fn free(&self) {
        unsafe {
            stb_image_rust::c_runtime::free(self.pixels);
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        self.free();
    }
}
