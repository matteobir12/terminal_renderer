use glam::{UVec2, Mat4, Vec2, Vec3};
use image::{ImageBuffer, GrayImage, Luma};

#[derive(Debug)]
pub struct Triangle {
    pub vertices: [Vec3; 3]
}

impl Triangle {
  pub fn new(v1:Vec3, v2:Vec3, v3:Vec3) -> Self {
    return Triangle{vertices: [v1, v2, v3]}
  }
}

pub fn do_pipeline(input_vtxs: &Vec<Triangle>, cam_mat: &Mat4) -> ImageBuffer<Luma<u8>, Vec<u8>> {
  let width = 256;
  let height = 256;
  let fov = 45.;
  let aspect = width as f32 / height as f32;
  let near = 0.01;
  let far = 10.0;
  let proj = Mat4::perspective_rh_gl(fov, aspect, near, far);
  let pipe_data = vertex_step(input_vtxs, proj * cam_mat);
  rasterize(pipe_data, height, width)
}

pub fn bary(p: &Vec3, a: &Vec3, b: &Vec3, c: &Vec3) -> (f32, f32, f32) {
  let ac = c - a;
  let ab = b - a;

  let total_area = ac.cross(ab).length().abs() / 2.0;

  let ap = p - a;
  let bp = p - b;
  let bc = c - b;

  let area_A = ap.cross(ab).length().abs() / 2.0;
  let area_B = ap.cross(ac).length().abs() / 2.0;
  let area_C = bp.cross(bc).length().abs() / 2.0;
  
  let u = area_A / total_area;
  let v = area_B / total_area;
  let w = area_C / total_area;
  
  (u, v, w)
}
struct VertexStepRes {
    triangle: Triangle,
    color: [u8; 3]
}

fn vertex_step(input_vtxs: &[Triangle], cam_mat: Mat4) -> Vec<VertexStepRes> {
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


fn lerp_three_pts(pos: UVec2, pt1: UVec2, color1:u8, pt2: UVec2, color2:u8, pt3: UVec2, color3:u8) -> u8
{
  1
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
