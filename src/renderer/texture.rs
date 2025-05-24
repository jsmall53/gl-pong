use glow::*;
use image::{GenericImageView, ImageFormat};
use std::convert::{From, Into};
use std::path::Path;
use std::rc::Rc;


#[derive(Clone, Debug)]
pub enum TextureFormat {
    NoneType,
    RGB8,
    RGBA8,
    RGBA32F,
}



pub struct TextureSpec {
    width: u32,
    height: u32,
    format: TextureFormat,
    generate_mips: bool,
}



pub trait Texture {
    fn get_width(&self) -> u32;
    fn get_height(&self) -> u32;
    fn get_path(&self) -> &str;
    // fn set_data(&mut self, bytes: &[u8]);
    fn bind(&self, slot: u32);
    // fn is_loaded();
}



pub struct GLTexture {
    gl: Rc<glow::Context>,
    path: String,
    width: u32,
    height: u32,
    texture: NativeTexture,
    format: TextureFormat,
}



impl GLTexture {
    pub fn new(gl: Rc<Context>, filepath: &str) -> Self {
        let path = Path::new(filepath);
        let img = image::open(path)
            .expect(&format!("Failed to open image: {}", filepath));

        let (width, height) = img.dimensions();
        let format = TextureFormat::from(img.color());

        unsafe {
            let texture = gl.create_texture().
                expect("Failed to create open gl texture.");
            gl.texture_storage_2d(texture, 1, format.clone().into(), width as i32, height as i32);
            gl.texture_parameter_i32(texture, glow::TEXTURE_MIN_FILTER, glow::LINEAR as i32);
            gl.texture_parameter_i32(texture, glow::TEXTURE_MAG_FILTER, glow::LINEAR as i32);
            gl.texture_parameter_i32(texture, glow::TEXTURE_WRAP_S, glow::REPEAT as i32);
            gl.texture_parameter_i32(texture, glow::TEXTURE_WRAP_T, glow::REPEAT as i32);
            gl.texture_sub_image_2d(
                texture, 
                0, 0, 0, 
                width as i32,
                height as i32, 
                format.to_gl_data_format(), 
                glow::UNSIGNED_BYTE, 
                PixelUnpackData::Slice(Some(img.as_bytes()))
            );

            Self {
                gl,
                path: filepath.into(),
                width,
                height,
                format,
                texture,
            }
        }
    }
}



impl Texture for GLTexture {
    fn bind(&self, slot: u32) {
        unsafe {
            self.gl.bind_texture_unit(slot, Some(self.texture));
        }
    }

    fn get_path(&self) -> &str {
        &self.path
    }

    fn get_width(&self) -> u32 {
        self.width
    }

    fn get_height(&self) -> u32 {
        self.height
    }
}



impl From<image::ColorType> for TextureFormat {
    fn from(item: image::ColorType) -> Self {
        match item {
            image::ColorType::Rgb8 => TextureFormat::RGB8,
            image::ColorType::Rgba8 => TextureFormat::RGBA8,
            image::ColorType::Rgba32F => TextureFormat::RGBA32F,
            _ => {
                assert!(false, "Unsupported image format");
                TextureFormat::NoneType
            }
        }
    }
}



impl Into<u32> for TextureFormat {
    fn into(self) -> u32 {
        match self {
            TextureFormat::RGB8 => glow::RGB8,
            TextureFormat::RGBA8 => glow::RGBA8,
            TextureFormat::RGBA32F => glow::RGBA32F,
            _ => {
                assert!(false, "Unsupported GL texture format");
                u32::min_value()
            }
        }
    }
}



impl Into<i32> for TextureFormat {
    fn into(self) -> i32 {
        let unsign: u32 = self.into();
        if unsign == u32::min_value() {
            i32::min_value()
        } else {
            unsign as i32
        }
    }

}



impl TextureFormat {
    fn to_gl_data_format(&self) -> u32 {
        match self {
            TextureFormat::RGB8 => glow::RGB,
            TextureFormat::RGBA8 |
                TextureFormat::RGBA32F => glow::RGBA,
            _ => {
                assert!(false, "Invalid texture data format");
                0
            }

        }
    }
}



fn format_from_path(path: &Path) -> Option<image::ImageFormat> {
    if let Some(ext) = path.extension() {
        image::ImageFormat::from_extension(ext)
    } else {
        None
    }
}
