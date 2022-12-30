use sdl2::{image::InitFlag, EventPump};

pub struct WindowSetup {
  pub window: sdl2::video::Window,
  pub event: EventPump,
}

pub fn init_sdl_and_window(title: &str, width: u32, height: u32) -> WindowSetup {
  let sdl_context = sdl2::init().unwrap();
  let _image_context = sdl2::image::init(InitFlag::JPG | InitFlag::PNG).unwrap();
  let video_subsystem = sdl_context.video().unwrap();

  let window = video_subsystem
    .window(title, width, height)
    .position_centered()
    .build()
    .unwrap();

  WindowSetup {
    window,
    event: sdl_context.event_pump().unwrap(),
  }
}
