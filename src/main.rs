extern crate sdl2;
extern crate sdl_test;

use std::env;
use std::path::Path;
use sdl2::event::Event;
use sdl2::image::{LoadSurface, INIT_PNG, INIT_JPG};
use sdl2::keyboard::Keycode;
use sdl2::mouse::Cursor;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::surface::Surface;
use sdl2::mouse::MouseButton;
use sdl2::pixels;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::gfx::rotozoom::RotozoomSurface;
use sdl2::render::WindowCanvas;
use sdl2::render::Texture;

use sdl_test::sprite::{ Sprite , Drawable,EventHandle};

const START_W:u32 = 180;
const START_H:u32 = 80;

const W:u32 = 800;
const H:u32 = 600;


pub fn run(png: &Path) {

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _image_context = sdl2::image::init(INIT_PNG | INIT_JPG).unwrap();
    let window = video_subsystem.window("rust-sdl2 demo: Cursor", W, H)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().software().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let surface = match Surface::from_file(png) {
        Ok(surface) => surface,
        Err(err) => panic!("failed to load cursor image: {}", err)
    };


    let new_surface = match surface.zoom(0.3, 0.3, false) {
        Ok(su) => {
            su
        },
        Err(e) => {
            panic!("failed to zoom surface: {}", e)
        }
    };

    let start_surface =  match Surface::from_file("start.png") {
        Ok(surface) => surface,
        Err(err) => panic!("failed to load cursor image: {}", err)
    };
    let start_texture = match  texture_creator.create_texture_from_surface(start_surface) {
        Ok(texture) => texture,
        Err(e) => panic!("failed to create start texture : {}", e)
    };

    let cursor = match Cursor::from_surface(new_surface, 0, 0) {
        Ok(cursor) => cursor,
        Err(err) => panic!("failed to load cursor: {}", err)
    };
    cursor.set();

    let start_sprite = create_start(start_texture);

    canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));


    let mut events = sdl_context.event_pump().unwrap();

    'mainloop: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit{..} |
                Event::KeyDown {keycode: Option::Some(Keycode::Escape), ..} =>
                    break 'mainloop,
                _ => {}
            }
        }
        canvas.clear();
        start_sprite.draw(&mut canvas);
        canvas.present();

    }
}

fn create_start(te : Texture) -> Sprite
{
    let dst = Rect::new(((W - START_W) / 2) as i32,((H - START_H) / 2) as i32,START_W as u32,START_H as u32);
    Sprite::new(None,Some(dst),te)
}


fn main() {

    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run /path/to/image.(png|jpg)")
    } else {
        run(Path::new(&args[1]));
    }
}
