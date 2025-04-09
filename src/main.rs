mod tesseract_user;
mod image_provider;

use crate::image_provider::{ImageProvider, SlurpImageProvider};
use crate::tesseract_user::tesseract::get_text_from_image;

fn main() {

    let image_provider = SlurpImageProvider;

    let image_data = image_provider.get_image().expect("Couldn't get image");

    let text = get_text_from_image(
        image_data.image.as_slice(),
        image_data.width as u32,
        image_data.height as u32
    );

    println!("{}", text);

}

// cargo update half@2.6.0 --precise 1.80.1