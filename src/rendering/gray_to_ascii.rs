pub fn gray_to_ascii() {
    let img = image::open("input.png")
        .expect("failed to open image")
        .to_luma8();

    let ramp = b"@%#*+=-:. ";

    for y in (0..img.height()).step_by(4) {
        for x in (0..img.width()).step_by(4) {
            let mut intensity = 0;
            for y_off in 0..4 {
                for x_off in 0..4 {
                    let pixel = img.get_pixel(x + x_off, y + y_off);
                    intensity += pixel[0] as usize;
                }
            }

            let idx = intensity * (ramp.len() - 1) / (255 * 16);
            print!("{}", ramp[idx] as char);
        }
        println!();
    }
}