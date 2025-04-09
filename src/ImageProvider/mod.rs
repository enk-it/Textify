mod stdin_image_provider;

use std::io;

trait ImageProvider {
    fn get_image(&self) -> Result<Vec<u8>, io::Error>;
}