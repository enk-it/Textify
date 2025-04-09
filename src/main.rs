mod tesseract_user;
use std::io::{Read};
use std::process::Command;
use tesseract::Tesseract;


fn get_text_from_image (
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

fn get_region_from_slurp() -> Result<(i32, i32, u32, u32), Box<dyn std::error::Error>> {
    let output = Command::new("slurp")
        .output()
        .expect("Не удалось запустить slurp. Убедитесь, что он установлен.");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Slurp завершился с ошибкой: {}", stderr).into());
    }

    let output_str = String::from_utf8(output.stdout)?;
    let parts: Vec<&str> = output_str.trim().split_whitespace().collect();
    if parts.len() != 2 {
        return Err("Неверный формат вывода slurp".into());
    }

    let coords: Vec<&str> = parts[0].split(',').collect();
    let x: i32 = coords[0].parse()?;
    let y: i32 = coords[1].parse()?;
    let dimensions: Vec<&str> = parts[1].split('x').collect();
    let width: u32 = dimensions[0].parse()?;
    let height: u32 = dimensions[1].parse()?;

    Ok((x, y, width, height))
}


fn main() {
    let (x, y, width, height) = get_region_from_slurp().expect("Ну ошибка ебать какая то");

    let geometry = format!("{},{} {}x{}", x, y, width, height);
    println!("Передаём в grim: grim -g '{}'", geometry);

    let output = Command::new("grim")
        .args(&["-g", &geometry, "-"])
        .output()
        .expect("Не удалось запустить grim. Убедитесь, что он установлен.");

    let img = image::load_from_memory(&output.stdout).expect("Failed to decode image");

    let new_height = img.height();
    let new_width = img.width();

    let rgba_img = img.to_rgba8();
    let image_data = rgba_img.as_raw();


    let text = get_text_from_image(
        image_data,
        new_width,
        new_height
    );

    println!("{}", text);

}

// cargo update half@2.6.0 --precise 1.80.1