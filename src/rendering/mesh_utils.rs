use std::f32::consts::PI;
use glam::{Mat3, Vec3, EulerRot};
use crate::rendering::Triangle;

pub fn sphere(stacks: usize, sectors: usize) -> Vec<Triangle> {
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

pub fn cube(size: f32) -> Vec<Triangle> {
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

pub fn translateTris(tris: &mut Vec<Triangle>, delta: Vec3) {
  for tri in tris.iter_mut() {
    tri.vertices[0] += delta;
    tri.vertices[1] += delta;
    tri.vertices[2] += delta;
  }
}

pub fn rotateTris(tris: &mut Vec<Triangle>, delta: Vec3) {
  let rotate_mat = Mat3::from_euler(EulerRot::XYZEx, delta.x, delta.y, delta.z);
  for tri in tris.iter_mut() {
    tri.vertices[0] = rotate_mat * tri.vertices[0];
    tri.vertices[1] = rotate_mat * tri.vertices[1];
    tri.vertices[2] = rotate_mat * tri.vertices[2];
  }
}