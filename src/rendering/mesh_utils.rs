use std::f32::consts::PI;
use glam::{Mat3, Vec3, EulerRot};
use crate::rendering::Triangle;

pub fn sphere(stacks: usize, sectors: usize, radius: usize) -> Vec<Triangle> {
  let mut tris: Vec<Triangle> = Vec::new();
  let stack_size = PI / stacks as f32;
  let sectors_size = 2.0 * PI / sectors as f32;

  for stack in 0..stacks {
    let bottom = stack as f32 * stack_size;
    let top = (stack + 1) as f32 * stack_size;

    for sector in 0..sectors {
      let left =  sector as f32 * sectors_size;
      let right =  (sector + 1) as f32 * sectors_size;

      let rad_f = radius as f32;
      let p00 = Vec3::new(
          rad_f * left.cos() * bottom.sin(),
          rad_f * bottom.cos(),
          rad_f * left.sin() * bottom.sin());

      let p01 = Vec3::new(
          rad_f * right.cos() * bottom.sin(),
          rad_f * bottom.cos(),
          rad_f * right.sin() * bottom.sin());

      let p10 = Vec3::new(
          rad_f * left.cos() * top.sin(),
          rad_f * top.cos(),
          rad_f * left.sin() * top.sin());

      let p11 = Vec3::new(
          rad_f * right.cos() * top.sin(),
          rad_f * top.cos(),
          rad_f * right.sin() * top.sin());

      tris.push(Triangle { vertices: [p00, p10, p11] });
      tris.push(Triangle { vertices: [p00, p11, p01] });
    }
  }

  tris
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