use glam::{UVec2, Mat4, Vec2, Vec3};
use image::{ImageBuffer, GrayImage, Luma};
pub struct Triangle {
    vertices: [Vec3; 3]
}

impl Triangle {
  pub fn new(v1:Vec3, v2:Vec3, v3:Vec3) -> Self {
    return Triangle{vertices: [v1, v2, v3]}
  }
}

pub fn do_pipeline(input_vtxs: &Vec<Triangle>, cam_mat: &Mat4) -> ImageBuffer<Luma<u8>, Vec<u8>> {
  let fov = 45.;
  let scale = 1. / ((fov * std::f32::consts::PI / 90.)).tan();
  let near = 0.01;
  let far = 10.0;
  let pipe_data = vertex_step(input_vtxs, cam_mat);
  rasterize(pipe_data, 256, 256)
}

struct VertexStepRes {
    triangle: Triangle,
    color: [u8; 3]
}

fn vertex_step(input_vtxs: &[Triangle], cam_mat: &Mat4) -> Vec<VertexStepRes> {
  let mut res = Vec::with_capacity(input_vtxs.len());

  // can filter out points outside of fov
  for tri in input_vtxs {
    let vtx_1 = tri.vertices[0].extend(1.);
    let vtx_2 = tri.vertices[1].extend(1.);
    let vtx_3 = tri.vertices[2].extend(1.);

    let tri = Triangle {
      vertices: [(cam_mat * vtx_1).truncate(),
                  (cam_mat * vtx_2).truncate(),
                  (cam_mat * vtx_3).truncate()]
    };

    res.push(VertexStepRes {triangle: tri, color: [255, 255, 255]});
  }

  return res;
}

fn barycentric(p: Vec2, a: Vec2, b: Vec2, c: Vec2) -> (f32, f32, f32) {
  let v0 = b - a;
  let v1 = c - a;
  let v2 = p - a;

  let d00 = v0.dot(v0);
  let d01 = v0.dot(v1);
  let d11 = v1.dot(v1);
  let d20 = v2.dot(v0);
  let d21 = v2.dot(v1);

  let denom = d00 * d11 - d01 * d01;

  let v = (d11 * d20 - d01 * d21) / denom;
  let w = (d00 * d21 - d01 * d20) / denom;
  let u = 1.0 - v - w;

  return (u, v, w);
}

fn lerp_three_pts(pos: UVec2, pt1: UVec2, color1:u8, pt2: UVec2, color2:u8, pt3: UVec2, color3:u8) -> u8
{
  let (w1, w2, w3) = barycentric(pos.as_vec2(), pt1.as_vec2(), pt2.as_vec2(), pt3.as_vec2());

  let value =
      (color1 as f32) * w1 +
      (color2 as f32) * w2 +
      (color3 as f32) * w3;

  return value.round().clamp(0.0, 255.0) as u8;
}

fn nc_to_screen(nc: Vec2, res_height: u32, res_width: u32) -> UVec2 {
  UVec2::new((res_width as f32 * (nc[0] + 1.) / 2.) as u32,
             (res_height as f32 * (nc[1] + 1.) / 2.) as u32)
}

fn rasterize(prims: Vec<VertexStepRes>, res_height: u32, res_width: u32) -> ImageBuffer<Luma<u8>, Vec<u8>> {
  let mut img = GrayImage::new(res_width, res_height);
  let mut z_buff = vec![f32::INFINITY; (res_width * res_height) as usize];


  for prim in prims {
    let vt1 = prim.triangle.vertices[0];
    let vt2 = prim.triangle.vertices[1];
    let vt3 = prim.triangle.vertices[2];
    let z = (vt1[2] + vt2[2] + vt3[2]) / 3 as f32;

    let nc1 = vt1.truncate() / vt1[2];
    let nc2 = vt2.truncate() / vt2[2];
    let nc3 = vt3.truncate() / vt3[2];

    let screen1 = nc_to_screen(nc1, res_height, res_width);
    let screen2 = nc_to_screen(nc2, res_height, res_width);
    let screen3 = nc_to_screen(nc3, res_height, res_width);


    let x_min = std::cmp::min(std::cmp::min(screen1[0], screen2[0]), screen3[0]).clamp(0, res_width - 1);
    let y_min = std::cmp::min(std::cmp::min(screen1[1], screen2[1]), screen3[1]).clamp(0, res_height - 1);

    let x_max = std::cmp::max(std::cmp::max(screen1[0], screen2[0]), screen3[0]).clamp(0, res_width - 1);
    let y_max = std::cmp::max(std::cmp::max(screen1[1], screen2[1]), screen3[1]).clamp(0, res_height - 1);

    // ... for every pixel in box, color pixel, maintain zbuf
    for x in x_min..x_max {
      for y in y_min..y_max {
        if z_buff[((x * res_width) + y) as usize] > z {
          img.put_pixel(x, y, image::Luma([lerp_three_pts(UVec2 { x: x, y: y }, screen1, prim.color[0], screen2, prim.color[1], screen3, prim.color[2])]));
          z_buff[((x * res_width) + y) as usize] = z;
        }
      }
    }
  }

  return img;
}

fn fragment_step() {

}
