use std::io;
use std::process::Command;
use crate::image_provider::ImageData;

pub struct SlurpImageProvider;


impl SlurpImageProvider {
    fn get_region_from_slurp(&self) -> Result<(i32, i32, u32, u32), Box<dyn std::error::Error>> {
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
}


impl super::ImageProvider for SlurpImageProvider {
    fn get_image (&self) -> Result<ImageData, io::Error> {
        let (x, y, width, height) = self.get_region_from_slurp().expect("Ну ошибка ебать какая то");

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

        Ok(
            ImageData {
                image: rgba_img.into_raw(),
                height: new_height as i32,
                width: new_width as i32
            }
        )
    }
}