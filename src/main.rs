mod rendering;
mod terminal;

use glam::{Vec3, Mat4};
use crate::terminal::Terminal;
use std::io::{self, Write};
use crate::rendering::{gray_to_ascii, do_pipeline, Triangle};
use crate::{rendering::{Triangle, do_pipeline}, terminal::Terminal};
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut term = Terminal::new()?;

    term.enter_raw_mode()?;
    term.enter_alternate_screen()?;
    term.hide_cursor()?;

    let tris = vec![Triangle::new(Vec3::new(1., 1., 0.), Vec3::new(2., 2., 1.), Vec3::new(2., 1., 1.))];
    let cam = Mat4::IDENTITY;

    let ascii_art: String = gray_to_ascii(48, 128);
    let tri = Triangle {
      vertices: [
        Vec3::new(10.0, 12.0, 6.0),
        Vec3::new(14.0, 12.0, 6.0),
        Vec3::new(16.0, 12.0, 6.0)
      ]
    };
    println!("printing triangle\r\n\r\n");
    println!("Triangle: {:?}", &tri);
    let p = Vec3::new(10.0, 12.0, 6.0);
    let (u, v, w) = barycentric(p, &tri.vertices[0], &tri.vertices[1],&tri.vertices[2]);
    println!("Triangle: {:?}", &tri);
    println!("Barycentric: {} {} {}", u, v, w);

    let tris = vec![tri];
    let eye_mat = Mat4::IDENTITY;
    
    do_pipeline(&tris, &eye_mat);
    loop {
        let input = term.read_input_non_blocking()?;
        const ESC_KEY:u8 = 27;

        if !input.is_empty() && (input == b"q" || input == [ESC_KEY]) {
            break;
        }

        term.clear_screen()?;
        let img = do_pipeline(&tris, &cam);
        let ascii_art: String = gray_to_ascii(img, 48, 128);
        io::stdout().write_all(ascii_art.as_bytes())?;
        io::stdout().flush()?;

        std::thread::sleep(std::time::Duration::from_millis(16));
    }


    term.show_cursor()?;
    term.exit_alternate_screen()?;
    term.restore()?;

    Ok(())
}
