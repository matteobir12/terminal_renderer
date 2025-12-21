#[allow(non_snake_case)]
mod rendering;
mod terminal;

use std::f32::consts::PI;
use glam::{Vec3, Mat4};
use std::io::{self, Write};
use crate::{rendering::{Triangle, gray_to_ascii, do_pipeline}, terminal::Terminal};

fn cube(center: Vec3, size: f32) -> Vec<Triangle> {
  let h = size * 0.5;

  let v = [
      center + Vec3::new(-h, -h, -h),
      center + Vec3::new( h, -h, -h),
      center + Vec3::new( h,  h, -h),
      center + Vec3::new(-h,  h, -h),
      center + Vec3::new(-h, -h,  h),
      center + Vec3::new( h, -h,  h),
      center + Vec3::new( h,  h,  h),
      center + Vec3::new(-h,  h,  h)];

  vec![
    Triangle { vertices: [v[0], v[1], v[2]] },
    Triangle { vertices: [v[0], v[2], v[3]] },

    Triangle { vertices: [v[5], v[4], v[7]] },
    Triangle { vertices: [v[5], v[7], v[6]] },

    Triangle { vertices: [v[4], v[0], v[3]] },
    Triangle { vertices: [v[4], v[3], v[7]] },

    Triangle { vertices: [v[1], v[5], v[6]] },
    Triangle { vertices: [v[1], v[6], v[2]] },

    Triangle { vertices: [v[4], v[5], v[1]] },
    Triangle { vertices: [v[4], v[1], v[0]] },

    Triangle { vertices: [v[3], v[2], v[6]] },
    Triangle { vertices: [v[3], v[6], v[7]] },
  ]
}

fn main() -> io::Result<()> {
    let mut term = Terminal::new()?;

    term.enter_raw_mode()?;
    term.enter_alternate_screen()?;
    term.hide_cursor()?;

    let tris = cube(Vec3::new(0.,0., 6.), 1.);
    let mut eye_mat = Mat4::IDENTITY;

    let yaw = PI;
    loop {
        let input = term.read_input_non_blocking()?;
        const ESC_KEY:u8 = 27;

        if !input.is_empty() {
          if input == b"q" || input == [ESC_KEY] {
            break;
          }

          if input == b"w" {
            eye_mat =Mat4::from_translation(Vec3::new(0.0, 0.0, -1.0)) * eye_mat;
            //eye_mat = Mat4::from_translation(Vec3::new(0.0, 0.0, -1.0)) * eye_mat;
            //eye_mat *= Mat4::from_translation(Vec3::new(0.0, 0.0, -1.0));
          }

          if input == b"a" {
            eye_mat =Mat4::from_translation(Vec3::new(-1.0, 0.0, 0.0)) * eye_mat;
            //eye_mat = Mat4::from_translation(Vec3::new(-1.0, 0.0, 0.0)) * eye_mat;
            //eye_mat *= Mat4::from_translation(Vec3::new(-1.0, 0.0, 0.0));
          }

          if input == b"s" {
            eye_mat = Mat4::from_translation(Vec3::new(0.0, 0.0, 1.0)) * eye_mat;
            //eye_mat = Mat4::from_translation(Vec3::new(0.0, 0.0, 1.0)) * eye_mat;
            //eye_mat *= Mat4::from_translation(Vec3::new(0.0, 0.0, 1.0));
          }

          if input == b"d" {
            eye_mat = Mat4::from_translation(Vec3::new(1.0, 0.0, 0.0)) * eye_mat;
            //eye_mat = Mat4::from_translation(Vec3::new(1.0, 0.0, 0.0)) * eye_mat;
            //eye_mat *= Mat4::from_translation(Vec3::new(1.0, 0.0, 0.0));
          }

          if input == b"j" {
            eye_mat = Mat4::from_rotation_y(-yaw) * eye_mat;
          }

          if input == b"l" {
            eye_mat =Mat4::from_rotation_y(yaw) * eye_mat;
          }
        }
        println!("{}\r\n", eye_mat);
        let img = do_pipeline(&tris, &eye_mat);
        let ascii_art = gray_to_ascii(img, 170, 64);
        term.clear_screen()?;
        io::stdout().write_all(ascii_art.as_bytes())?;
        io::stdout().flush()?;

        std::thread::sleep(std::time::Duration::from_millis(16));
    }


    term.show_cursor()?;
    term.exit_alternate_screen()?;
    term.restore()?;

    Ok(())
}
