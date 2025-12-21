#[allow(non_snake_case)]
mod rendering;
mod terminal;

use glam::{Vec3, Mat4};
use std::io::{self, Write};
use crate::{rendering::{Triangle, gray_to_ascii, do_pipeline, barycentric}, terminal::Terminal};

fn main() -> io::Result<()> {
    let mut term = Terminal::new()?;

    term.enter_raw_mode()?;
    term.enter_alternate_screen()?;
    term.hide_cursor()?;

    //let ascii_art: String = gray_to_ascii(48, 128);
    let tri = Triangle {
      vertices: [
        Vec3::new(1.0, 5.0, 16.0),
        Vec3::new(3.0, 2.0, 1.5),
        Vec3::new(6.0, 4.0, 16.1)
      ]
    };
    //let p = Vec3::new(11.0, 12.5, 6.0);
    //let (u, v, w) = barycentric(&p, &tri.vertices[0], &tri.vertices[1], &tri.vertices[2]);
    //println!("printing triangle\r\n");
    //println!("Triangle: {:?}\r\n", &tri);
    //println!("Barycentric: {} {} {}\r\n", u, v, w);

    let mut tris = vec![tri];
    let mut eye_mat = Mat4::IDENTITY;
    let mut img = do_pipeline(&tris, &eye_mat);
    match img.save("img.png") {
      Ok(T) => println!("Wrote image!\r\n"),
      _ => println!("Error writing image\r\n")
    };
    let mut ascii_art: String = gray_to_ascii(img, 48, 128);
    loop {
        let input = term.read_input_non_blocking()?;
        const ESC_KEY:u8 = 27;

        if !input.is_empty() {
          if (input == b"q" || input == [ESC_KEY]) {
            break;
          }

          if input == b"w" {
            eye_mat = eye_mat * Mat4::from_translation(Vec3::new(0.0, 0.0, -1.0));
          }

          if input == b"a" {
            eye_mat = eye_mat * Mat4::from_translation(Vec3::new(1.0, 0.0, 0.0));
          }

          if input == b"s" {
            eye_mat = eye_mat * Mat4::from_translation(Vec3::new(0.0, 0.0, 1.0));
          }

          if input == b"d" {
            eye_mat = eye_mat * Mat4::from_translation(Vec3::new(-1.0, 0.0, 0.0));
          }
        }

        img = do_pipeline(&tris, &eye_mat);
        ascii_art = gray_to_ascii(img, 48, 128);
        term.clear_screen()?;
        io::stdout().write_all(ascii_art.as_bytes())?;
        io::stdout().flush()?;

        std::thread::sleep(std::time::Duration::from_millis(16));
        tris[0].vertices[0].x -= 1.0;
        tris[0].vertices[0].y -= 1.0;
        
    }


    term.show_cursor()?;
    term.exit_alternate_screen()?;
    term.restore()?;

    Ok(())
}
