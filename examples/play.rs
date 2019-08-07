extern crate kiss3d;
extern crate nalgebra as na;

use na::{Point3, Translation3, Vector3, UnitQuaternion};
use kiss3d::window::Window;
use kiss3d::light::Light;
//use kiss3d::resource::FramebufferManager;
use std::path::Path;
use kiss3d::camera::{FirstPerson};

fn main() {
  //let mut fbm = FramebufferManager::new();
  //let mut render_targ = FramebufferManager::new_render_target(640 , 480);
  //fbm.select(&mut render_targ);
  
    let eye = Point3::new(2.5, 2.5, 2.5);
    let at = Point3::origin();
    let mut first_person = FirstPerson::new(eye, at);

    let mut window = Window::new_with_size("play", 640,480);
    let mut c      = window.add_cube(1.0, 0.5, 0.25);
    c.set_color(0.01, 1.0, 0.0);

    let mut cube2      = window.add_cube(1.0, 0.5, 0.25);
    cube2.set_local_translation(Translation3::new(0.0, 0.0, -0.25));
    cube2.set_color(1.0, 0.0, 0.0);

    let light_pos = Point3::new(-2.0f32, 3.0f32, 2.0f32);
    window.set_light( Light::Absolute(light_pos));

    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.056);

     //throwaway renders when context is first opened
    for _i in 0..5 {
      window.render_with_camera(&mut first_person);
    }

    for i in 0..20 {
      if window.render_with_camera(&mut first_person) {
        let img = window.snap_image();
        let img_name = format!("frame_{}.png", i);
        let img_path = Path::new(&img_name);
        img.save(img_path).unwrap();
      }
      c.prepend_to_local_rotation(&rot);
      first_person.handle_scroll(0.25);
      //let light_pos = Point3::new(light_pos[0], light_pos[1] + 1.0, light_pos[2]);
      //window.set_light( Light::Absolute(light_pos));
    }
}
