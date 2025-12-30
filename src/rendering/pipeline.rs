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

pub fn do_pipeline(input_vtxs: &Vec<Triangle>, cam_mat: &Mat4, ) -> ImageBuffer<Luma<u8>, Vec<u8>> {
  let width = 256;
  let height = 256;
  let fov = 90.;
  let aspect = width as f32 / height as f32;
  let near = 0.01;
  let far = 10.0;
  let proj = Mat4::perspective_rh_gl(fov, aspect, near, far);
  //let r = [1.,0.,0.,0., 
  //         0.,1.,0.,0., 
  //         0.,0.,((-1.)/(8.)),0., 
  //         0.,0.,0.,1.];
  //let proj = Mat4::from_cols_array(&r);
  let pipe_data = vertex_step(input_vtxs, proj * cam_mat);
  rasterize(pipe_data, height, width)
}

// Ensure the vector does NOT represent a DEGENERATE triangle. If it is, (0,0,0) is returned
pub fn barycentric(p: &Vec3, a: &Vec3, b: &Vec3, c: &Vec3) -> (f32, f32, f32) {
  let ac = c - a;
  let ab = b - a;

  let total_area = ab.cross(ac).length().abs() / 2.0;
  if total_area == 0.0 {
    return (0.0, 0.0, 0.0);
  }

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

fn is_inside(endpoint1: UVec2, endpoint2: UVec2, x: u32, y: u32) -> bool
{
  let endpoint1_i = endpoint1.as_ivec2();
  let endpoint2_i = endpoint2.as_ivec2();
  let a = endpoint2_i.y - endpoint1_i.y;
  let b = endpoint1_i.x - endpoint2_i.x;
  let c = endpoint2_i.x * endpoint1_i.y - endpoint1_i.x * endpoint2_i.y;

  a*(x as i32) + b*(y as i32) + c >= 0
}

fn is_inside_triangle(pt1: UVec2, pt2: UVec2, pt3: UVec2, x: u32, y: u32) -> bool
{
  is_inside(pt1, pt2,x, y) &&
  is_inside(pt2, pt3,x, y) &&
  is_inside(pt3, pt1,x, y)
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
    let z = (vt1[2] + vt2[2] + vt3[2]) / 3.0 as f32;
    
    let nc1 = if vt1[2] != 0.0 {vt1.truncate() / vt1[2]} else {vt1.truncate()};
    let nc2 = if vt2[2] != 0.0 {vt2.truncate() / vt2[2]} else {vt2.truncate()};
    let nc3 = if vt3[2] != 0.0 {vt3.truncate() / vt3[2]} else {vt3.truncate()};

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
        if is_inside_triangle(screen1, screen2, screen3, x, y) {
          //img.put_pixel(x, y, image::Luma([255]));
          img.put_pixel(x, y, image::Luma([(z * 100.0).abs() as u8]));
          z_buff[((x * res_width) + y) as usize] = z;
        }
      }
    }
  }

  return img;
}
