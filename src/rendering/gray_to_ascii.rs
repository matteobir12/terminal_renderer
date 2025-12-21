use image::GrayImage;

pub fn gray_to_ascii(img: GrayImage, height_px: u32, width_px: u32) -> String {
    let ramp = b"#@%x+=~-:. ";
    let ramp = b" .:-~=+oOx0X@&#";
    let mut output = String::new();

    let height_step = img.height() / height_px;
    let width_step = img.width() / width_px;

    for y in (0..img.height()).step_by(height_step as usize) {
        for x in (0..img.width()).step_by(width_step as usize) {
            let mut intensity = 0;
            for y_off in 0..height_step {
                for x_off in 0..width_step {
                    if x + x_off < img.width() && y + y_off < img.height() {
                        let pixel = img.get_pixel(x + x_off, y + y_off);
                        intensity += pixel[0] as usize;
                    }
                }
            }

            let pool_size = (height_step * width_step) as usize;
            let idx = intensity * (ramp.len() - 1) / (255 * pool_size);
            output.push(ramp[idx] as char);
        }
        output.push('\r');
        output.push('\n');
    }

    return output;
}