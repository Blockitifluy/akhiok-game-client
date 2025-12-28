use std::{fs, io};

pub struct Texture {
    pub width: i32,
    pub height: i32,
    pub pixels: *mut u8,
    pub comp: i32,
}
impl Texture {
    pub fn new(path: &str) -> Result<Self, &'static str> {
        let f_ex = fs::File::open(path);
        let Ok(mut f) = f_ex else {
            return Err("couldn't load file");
        };

        let mut bytes = vec![];

        io::Read::read_to_end(&mut f, &mut bytes).unwrap();

        let mut texture = Self {
            width: 0,
            height: 0,
            pixels: 0 as *mut _,
            comp: 0,
        };

        unsafe {
            stb_image_rust::stbi_set_flip_vertically_on_load(true as i32);
            texture.pixels = stb_image_rust::stbi_load_from_memory(
                bytes.as_mut_ptr(),
                bytes.len() as i32,
                &mut texture.width,
                &mut texture.height,
                &mut texture.comp,
                stb_image_rust::STBI_rgb_alpha,
            );
        }

        Ok(texture)
    }

    pub fn free(self) {
        unsafe {
            stb_image_rust::c_runtime::free(self.pixels);
        }
    }
}
