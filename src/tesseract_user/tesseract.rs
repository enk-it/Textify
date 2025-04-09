use tesseract::Tesseract;

pub fn get_text_from_image (
    image: &[u8],
    width: u32,
    height: u32
) -> String {
    const BYTES_PER_PIXEL: i32 = 4;
    let bytes_per_line: i32 = width as i32 * BYTES_PER_PIXEL;

    let expected_size = (width * height * BYTES_PER_PIXEL as u32) as usize;
    assert_eq!(image.len(), expected_size, "Data size mismatch");

    let mut ocr = Tesseract::new(
        None,
        Some("eng+rus")
    ).expect("Failed to create Tesseract instance");

    ocr = ocr.set_frame(
        image,
        width as i32,
        height as i32,
        BYTES_PER_PIXEL,
        bytes_per_line
    ).expect("Failed to set image");

    let text = ocr.get_text().expect("Failed to get text");
    text
}