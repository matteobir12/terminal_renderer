#[allow(non_snake_case)]
mod rendering;
mod terminal;

use std::f32::consts::PI;
use glam::{Mat3, Mat4, Vec3, EulerRot};
use std::io::{self, Write};
use crate::{rendering::{Triangle, gray_to_ascii, do_pipeline}, terminal::Terminal};

fn sphere(stacks: usize, sectors: usize) -> Vec<Triangle> {
  let mut v: Vec<Vec3> = Vec::new();
  let stacks_f = stacks as f32;
  let sectors_f = sectors as f32;
  
  v.push(Vec3::new(0.0, 1.0, 0.0));
  for stack in 1..stacks {
    let phi = (PI / 2.0) - (PI * (stack as f32 / stacks_f));
    for sector in 0..sectors {
      let theta = 2.0*PI*(sector as f32 / sectors_f);
      let x = phi.cos()*theta.sin();
      let y = phi.sin();
      let z = -phi.cos()*theta.sin();
      let sphere_coord = Vec3::new(x, y, z);
      println!("phi: {} theta: {} x: {} y: {} z: {}", phi, theta, x, y, z);
      v.push(sphere_coord);
    }
  }
  let mut tris: Vec<Triangle> = Vec::new();
  v.push(Vec3::new(0.0, -1.0, 0.0));
  for i in 0..sectors {
    let a = 0;
    let b = i + 1;
    let c = 1 + ((i + 1) % sectors);

    let tri = Triangle { 
      vertices: [v[a], v[b], v[c]]
    };
    println!("a: {} b: {} c: {} Tri: {:?}", a, b, c, tri);
    tris.push(tri);
  }
  println!("{:?}", tris);
  // Do middle stacks
  for stack in 0..(stacks - 2) {
    for sector in 0..sectors {

      //Triangle 1
      let mut a = (stack * sectors) + (sector + 1);
      let mut b = a + sectors;
      let c = ((stack * sectors) + 1) + ((sector + 1) % (sectors)); 
      let tri1 = Triangle { 
        vertices: [v[a], v[b], v[c]]
      };
      tris.push(tri1);

      println!("a: {} b: {} c: {}", a, b, c);


      // Triangle 2
      a = b;
      b = c + sectors;
      let tri2 = Triangle { 
        vertices: [v[a], v[b], v[c]]
      };
      println!("a: {} b: {} c: {}", a, b, c);
      tris.push(tri2);
    }
  }

  for i in 0..sectors {
    let a = sectors * (stacks - 2) + 1 + i; 
    let b = sectors * (stacks - 1) + 1;
    let c = sectors * (stacks - 2) + 1 + ((i + 1) % sectors);
    println!("a: {} b: {} c: {}", a, b, c);
    let tri = Triangle { 
      vertices: [v[a], v[b], v[c]]
    };
    tris.push(tri);
  }

  return tris;
}

fn cube(size: f32) -> Vec<Triangle> {
  let h = size * 0.5;

  let v = [
      Vec3::new(-h, -h, -h),
      Vec3::new( h, -h, -h),
      Vec3::new( h,  h, -h),
      Vec3::new(-h,  h, -h),
      Vec3::new(-h, -h,  h),
      Vec3::new( h, -h,  h),
      Vec3::new( h,  h,  h),
      Vec3::new(-h,  h,  h)];

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

fn translateTris(tris: &mut Vec<Triangle>, delta: Vec3) {
  for tri in tris.iter_mut() {
    tri.vertices[0] += delta;
    tri.vertices[1] += delta;
    tri.vertices[2] += delta;
  }
}

fn rotateTris(tris: &mut Vec<Triangle>, delta: Vec3) {
  let rotate_mat = Mat3::from_euler(EulerRot::XYZEx, delta.x, delta.y, delta.z);
  for tri in tris.iter_mut() {
    tri.vertices[0] = rotate_mat * tri.vertices[0];
    tri.vertices[1] = rotate_mat * tri.vertices[1];
    tri.vertices[2] = rotate_mat * tri.vertices[2];
  }
}

fn main() -> io::Result<()> {
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
