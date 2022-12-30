use graphics::actor::Actor;
use graphics::position::GetRect;
// use graphics::actor::Actor;
use graphics::resource_manager::TextureManager;
use graphics::sdl2::event::Event;
use graphics::sdl2::keyboard::Keycode;
use graphics::sdl2::pixels::Color;
use graphics::sdl2::video::WindowContext;
use std::collections::HashSet;
use std::time::Duration;

pub fn main() {
  game();
  // test_func_1();
}

fn test_func_1() {
  let mut setup = graphics::setup::init_sdl_and_window("Space Shooter", 800, 600);
  let mut prev_keys = HashSet::new();

  'running: loop {
    for event in setup.event.poll_iter() {
      if let Event::Quit { .. } = event {
        break 'running;
      };
    }

    // Create a set of pressed Keys.
    let keys = setup
      .event
      .keyboard_state()
      .pressed_scancodes()
      .filter_map(Keycode::from_scancode)
      .collect();

    // Get the difference between the new and old sets.
    let new_keys = &keys - &prev_keys;
    let old_keys = &prev_keys - &keys;

    if !new_keys.is_empty() || !old_keys.is_empty() {
      println!("new_keys: {:?}\told_keys:{:?}", new_keys, old_keys);
    }

    prev_keys = keys;

    std::thread::sleep(Duration::from_millis(100));
  }
}

fn game() {
  let mut setup = graphics::setup::init_sdl_and_window("Space Shooter", 800, 600);

  let mut canvas = setup.window.into_canvas().build().unwrap();
  let texture_creator = canvas.texture_creator();
  let mut texture_manager: TextureManager<WindowContext> = TextureManager::new(&texture_creator);
  let mut player = Actor::new(
    &mut texture_manager,
    "/home/leslie/Desktop/sdl2-rust/space-shooter/assets/purple.png",
    50,
    50,
    50,
    50,
  );
  player.set_angle(-90.);
  canvas.set_draw_color(Color::RGB(0, 255, 255));
  canvas.clear();
  canvas.present();
  let mut i = 0;
  'running: loop {
    i = (i + 1) % 255;
    canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
    canvas.clear();
    for event in setup.event.poll_iter() {
      match event {
        Event::Quit { .. }
        | Event::KeyDown {
          keycode: Some(Keycode::Escape),
          ..
        }
        | Event::KeyDown {
          keycode: Some(Keycode::Q),
          ..
        } => break 'running,
        _ => {}
      }
    }

    let keys: HashSet<Keycode> = setup
      .event
      .keyboard_state()
      .pressed_scancodes()
      .filter_map(Keycode::from_scancode)
      .collect();
    let mut y = 0.;
    let mut x = 0.;
    for key in keys {
      match key {
        Keycode::Space => {
          i = 0;
        }
        Keycode::Up => {
          y -= 1.;
        }
        Keycode::Down => {
          y += 1.;
        }
        Keycode::Left => {
          x -= 1.;
        }
        Keycode::Right => {
          x += 1.;
        }
        _ => {}
      }
    }
    player.vector.set(x, y);
    // if keys.`
    // The rest of the game loop goes here...

    player.update_pos();
    player.draw(&mut canvas, &mut texture_manager);
    canvas.present();
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
  }
}
