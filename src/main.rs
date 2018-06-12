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
use std::rc::Rc;
use std::cell::RefCell;
use std::vec::Vec;

use sdl_test::sprite::{ Sprite , Drawable,EventHandle};

const START_W:u32 = 180;
const START_H:u32 = 80;

const W:u32 = 800;
const H:u32 = 600;


pub fn run(png: &Path) {
    let sprites : Rc<RefCell<Vec<RefCell<Sprite>>>> = Rc::new(RefCell::new(Vec::new()));

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

    let start_surface =  match Surface::from_file("resource/start.png") {
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
    (*sprites).borrow_mut().push(RefCell::new(start_sprite));


    canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));


    let mut events = sdl_context.event_pump().unwrap();

    'mainloop: loop {
        let mouse_pos = (events.mouse_state().x(),events.mouse_state().y());
        let temp = (*sprites).borrow();
        for event in events.poll_iter() {
            for i in 0..(*sprites).borrow().len(){
                if temp[i].borrow().isVisible && temp[i].borrow().inBound(mouse_pos){
                    temp[i].borrow().onHandleEvent(&event);
                }
            }
            match event {
                Event::Quit{..} |
                Event::KeyDown {keycode: Option::Some(Keycode::Escape), ..} =>
                    break 'mainloop,
                _ => {}
            }
        }

        canvas.clear();

        for i in 0..(*sprites).borrow().len(){
            if temp[i].borrow_mut().isVisible{
                temp[i].borrow_mut().draw(&mut canvas);
            }
        }
        canvas.present();

    }
}

fn create_start(te : Texture) -> Sprite
{
    let dst = Rect::new(((W - START_W) / 2) as i32,((H - START_H) / 2) as i32,START_W as u32,START_H as u32);
    let mut start = Sprite::new(None,Some(dst),te,"start");

    start.setEventFunc(|e,s|{
        match *e {
            Event::MouseButtonDown {mouse_btn,..} => {
                match mouse_btn {
                    MouseButton::Left => {
                        let nw = START_W - 10;
                        let nh = START_H - 4;
                        let n_dst = Rect::new(((W - nw ) / 2) as i32,((H - nh) / 2) as i32,nw as u32,nh as u32);
                        unsafe {(*s.getRefMut()).dst = Some(n_dst);}
                    },
                    _ => {}
                }
            },
            Event::MouseButtonUp {mouse_btn,..} => {
                match mouse_btn {
                    MouseButton::Left => {
                        let dst = Rect::new(((W - START_W) / 2) as i32,((H - START_H) / 2) as i32,START_W as u32,START_H as u32);
                        unsafe {(*s.getRefMut()).dst = Some(dst);}
                    },
                    _ =>{}
                }
            }
            _ => {}
        }
    });
    start
}


fn main() {
    run(Path::new("resource/cursor.png"));
}
