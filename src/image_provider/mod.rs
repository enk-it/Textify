use std::io;
pub mod slurp_image_provider;

pub use slurp_image_provider::SlurpImageProvider;


pub struct ImageData {
    pub image: Vec<u8>,
    pub height: i32,
    pub width: i32
}



pub trait ImageProvider {
    fn get_image(&self) -> Result<ImageData, io::Error>;
}

