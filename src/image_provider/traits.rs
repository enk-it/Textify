use std::io;
use super::structs::ImageData;  // Импортируем структуру

pub trait ImageProvider {
    fn get_image(&self) -> Result<ImageData, io::Error>;
}
