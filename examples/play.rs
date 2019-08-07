extern crate kiss3d;
extern crate nalgebra as na;
//extern crate kiss3d_conrod;

use na::{Point3, Translation3, Vector3, UnitQuaternion};
use kiss3d::window::Window;
use kiss3d::light::Light;
//use kiss3d::resource::FramebufferManager;
use std::path::Path;
use kiss3d::camera::{FirstPerson};
//use kiss3d_conrod::input::Key;
use kiss3d::event::{Action, Key, WindowEvent};

fn main() {
  //let mut fbm = FramebufferManager::new();
  //let mut render_targ = FramebufferManager::new_render_target(640 , 480);
  //fbm.select(&mut render_targ);
  
    let eye = Point3::new(0.1, 1.1, 2.5);
    let at = Point3::origin();
    let mut camera = FirstPerson::new(eye, at);
    let light_pos = Point3::new(-2.0f32, 3.0f32, 2.0f32);
    let large_x_translate = Translation3::new(0.25, 0.0, 0.0);
    let translate_x_plus = Translation3::new(0.01, 0.0, 0.0);
    let translate_x_minus = Translation3::new(-0.01, 0.0, 0.0);

    let mut window = Window::new_with_size("play", 640,480);
    window.set_light( Light::Absolute(light_pos));

    let mut cube1      = window.add_cube(1.0,  1.0, 1.0);
    cube1.set_color(1.0, 1.0, .0);

    let mut cube2      = window.add_cube(1.0, 1.0, 1.0);
    cube2.set_local_translation(Translation3::new(1.0, 0.0, -2.0));
    cube2.set_color(0.0, 1.0, 0.0);

    let mut cube3      = window.add_cube(1.0, 1.0, 1.0);
    cube3.set_local_translation(Translation3::new(-1.0, 0.0, 2.0));
    cube3.set_color(0.0, 1.0, 0.0);


//    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.056);

     //throwaway renders when context is first opened
    for _i in 0..5 {
      window.render_with_camera(&mut camera);
    }

    for i in 0..25 {
        if window.render_with_camera(&mut camera) {
            let img = window.snap_image();
            let img_name = format!("./out/frame_{}.png", i);
            let img_path = Path::new(&img_name);
            img.save(img_path).unwrap();
        }
        //cube1.prepend_to_local_rotation(&rot);

        camera.translate_mut( &large_x_translate);
        //first_person.handle_scroll(0.25);
        //let light_pos = Point3::new(light_pos[0], light_pos[1] + 1.0, light_pos[2]);
        //window.set_light( Light::Absolute(light_pos));
    }

    while window.render_with_camera(&mut camera)  {
        for mut event in window.events().iter() {
            match event.value {
                WindowEvent::Key(Key::Left, Action::Release, _) => {
                    camera.translate_mut( &translate_x_minus);
                }
                WindowEvent::Key(Key::Right, Action::Release, _) => {
                    camera.translate_mut( &translate_x_plus);
                }
                WindowEvent::Key(Key::Q, Action::Release, _) => {
                    return;
                }
                _ => {}
            }
        }
    }
}
