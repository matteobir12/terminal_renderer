#[allow(non_snake_case)]
mod rendering;
mod terminal;
use std::panic;

use std::f32::consts::PI;
use std::io::{self, Write};
use glam::{Mat4, Vec3};
use crate::{rendering::*, terminal::Terminal};

fn main() -> io::Result<()> {
    panic::set_hook(Box::new(|info| {
        println!("panic {}", info);
    }));

    let mut sphere_tris = sphere(8, 8);
    let mut term = Terminal::new()?;

    term.enter_raw_mode()?;
    term.enter_alternate_screen()?;
    term.hide_cursor()?;

    //let mut cube_tris = cube(0.5);
    translateTris(&mut sphere_tris, Vec3::from_array([0.0, 0.0, 0.0]));     
    //rotateTris(&mut sphere_tris, Vec3::from_array([0.0, PI / 8.0, 0.0]));     
    let mut eye_mat = Mat4::IDENTITY;
    eye_mat = Mat4::from_translation(Vec3::new(0.0, 0.0, 2.0)) * eye_mat;

    let yaw = PI / 8.0;
    let FPS = 1;
    let mut counter: usize = 0;
    loop {
        let input = term.read_input_non_blocking()?;
        const ESC_KEY:u8 = 27;

        if !input.is_empty() {
          if input == b"q" || input == [ESC_KEY] {
            break;
          }

          if input == b"w" {
            eye_mat = Mat4::from_translation(Vec3::new(0.0, 0.0, -1.0)) * eye_mat;
          }

          if input == b"a" {
            eye_mat = Mat4::from_translation(Vec3::new(-1.0, 0.0, 0.0)) * eye_mat;
          }

          if input == b"s" {
            eye_mat = Mat4::from_translation(Vec3::new(0.0, 0.0, 1.0)) * eye_mat;
          }

          if input == b"d" {
            eye_mat = Mat4::from_translation(Vec3::new(1.0, 0.0, 0.0)) * eye_mat;
          }

          if input == b"j" {
            eye_mat = Mat4::from_rotation_y(-yaw) * eye_mat;
          }

          if input == b"l" {
            eye_mat =Mat4::from_rotation_y(yaw) * eye_mat;
          }
        }
        rotateTris(&mut sphere_tris, Vec3::from_array([0.0, 0.1, 0.0]));
        println!("{}\r\n", eye_mat);
        let img = do_pipeline(&sphere_tris, &eye_mat);
        let img_path = format!("imgs/img{}.png", counter.to_string());
        img.save(img_path).expect("good");
        let ascii_art = gray_to_ascii(img, 170, 64);
        term.clear_screen()?;
        io::stdout().write_all(ascii_art.as_bytes())?;
        io::stdout().flush()?;

        std::thread::sleep(std::time::Duration::from_millis(500));
        counter = counter + 1;
    }


    term.show_cursor()?;
    term.exit_alternate_screen()?;
    term.restore()?;

    Ok(())
}
