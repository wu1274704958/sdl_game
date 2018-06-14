extern crate sdl2;
#[macro_use] extern crate sdl_test;

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
use sdl2::render::{Texture,TextureCreator};
use std::rc::Rc;
use std::cell::RefCell;
use std::vec::Vec;
use std::rc::Weak;
use std::time::SystemTime;
use std::thread::sleep_ms;
use sdl2::video::WindowContext;
use sdl_test::plane::Bullet;



use sdl_test::sprite::{ Sprite , Drawable,EventHandle,BV,DH,HasTag};
use sdl_test::plane::Plane;
use std::cell::Ref;

const START_W:u32 = 180;
const START_H:u32 = 80;

const W:u32 = 400;
const H:u32 = 500;

static mut BGY:f32 = 1000f32;
static mut BULLET_TEX_PTR:*const Texture = 0 as *const Texture;

static mut MOUSE_POS:(i32,i32) = (0,0);

pub fn run(png: &Path) {
    let sprites : Rc<RefCell<Vec<RefCell<Box<DH <Target=WindowCanvas>>>>>> = Rc::new(RefCell::new(Vec::new()));
    let buffer : Rc<RefCell<Vec<RefCell<Box<DH <Target=WindowCanvas>>>>>> = Rc::new(RefCell::new(Vec::new()));

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _image_context = sdl2::image::init(INIT_PNG | INIT_JPG).unwrap();
    let window = video_subsystem.window("sdl_game", W, H)
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
    let cursor = match Cursor::from_surface(new_surface, 0, 0) {
        Ok(cursor) => cursor,
        Err(err) => panic!("failed to load cursor: {}", err)
    };
    cursor.set();

    let bullet_texture = create_texture!("resource/bullet.png",texture_creator);
    unsafe {BULLET_TEX_PTR = &bullet_texture as *const Texture;}

    let start_sprite = create_start(&texture_creator,Rc::downgrade(&sprites));
    (*sprites).borrow_mut().push(RefCell::new(Box::new(start_sprite)));

    let bg_sprite = create_bg(&texture_creator);
    (*sprites).borrow_mut().push(RefCell::new(Box::new(bg_sprite)));

    let plane_player = create_plane(&texture_creator,Rc::downgrade(&sprites),Rc::downgrade(&buffer));
    (*sprites).borrow_mut().push(RefCell::new(Box::new(plane_player)));

    canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));


    let mut events = sdl_context.event_pump().unwrap();

    let mut delatime = 16f32;

    'mainloop: loop {
        let start_time = SystemTime::now();

        unsafe { MOUSE_POS = (events.mouse_state().x(),events.mouse_state().y());}
        {
            let temp = (*sprites).borrow();
            for event in events.poll_iter() {
                for i in 0..temp.len() {
                    if temp[i].borrow().is_visible() && temp[i].borrow().in_bound(unsafe{MOUSE_POS}) {
                        temp[i].borrow().on_handle_event(&event);
                    }
                }
                match event {
                    Event::Quit { .. } |
                    Event::KeyDown { keycode: Option::Some(Keycode::Escape), .. } =>
                        break 'mainloop,
                    _ => {}
                }
            }

            canvas.clear();

            for i in 0..temp.len() {
                if temp[i].borrow().is_visible() {
                    temp[i].borrow().draw(&mut canvas);
                    temp[i].borrow().update(delatime);
                }
            }
            canvas.present();
        }
        if !buffer.borrow().is_empty(){
            let mut temp = (*sprites).borrow_mut();
            let mut buf_temp = (*buffer).borrow_mut();
            while !buf_temp.is_empty() {
                if let Some(thing) = buf_temp.pop() {
                    temp.push(thing);
                }
            }
        }

        let end_time = SystemTime::now();

        delatime = match start_time.elapsed() {
            Ok(duration) => {
                match (end_time - duration).elapsed(){
                    Ok(dur) => {
                        dur.subsec_nanos() as f32 / 1_000_000f32
                    },
                    Err(e) => { panic!(" end_time - start_time elapsed Error {}",e) }
                }
            },
            Err(e) => { panic!("start_time elapsed Error {}",e) }
        };
        //sleep
        if delatime < 16f32{
            sleep_ms(16u32 - delatime as u32);
        }
    }
    println!("end  {}",Rc::strong_count(&sprites));
}

fn create_start(tc : &TextureCreator<WindowContext>,
                sps : Weak<RefCell<Vec<RefCell<Box<DH <Target=WindowCanvas>>>>>>) -> Sprite
{
    let start_texture = create_texture!("resource/start.png",tc);

    let dst = Rect::new(((W - START_W) / 2) as i32,((H - START_H) / 2) as i32,START_W as u32,START_H as u32);
    let mut start = Sprite::new(None,Some(dst),start_texture,"start");

    start.setEventFunc(Box::new(move |e:&Event,s:&Sprite|{

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
                        //let dst = Rect::new(((W - START_W) / 2) as i32,((H - START_H) / 2) as i32,START_W as u32,START_H as u32);
                        //unsafe {(*s.getRefMut()).dst = Some(dst);}
                        unsafe { (*s.getRefMut()).isVisible = false; }
                        if let Some(sp_up) = sps.upgrade(){
                            let temp = (*sp_up).borrow();
                            temp.iter().for_each(|it|{
                                /*if it.borrow().tag() == "bg"{
                                    unsafe {
                                        let temp_bg: &RefCell<Box<Sprite>> = std::mem::transmute(it);
                                        temp_bg.borrow_mut().isVisible = true;
                                    }
                                }*/
                                let tag = it.borrow().tag();
                                match tag {
                                    "bg" => {
                                        unsafe {
                                            let temp_bg: &RefCell<Box<Sprite>> = std::mem::transmute(it);
                                            temp_bg.borrow_mut().isVisible = true;
                                        }
                                    },
                                    "plane" => {
                                        unsafe {
                                            let temp_plane: &RefCell<Box<Plane>> = std::mem::transmute(it);
                                            temp_plane.borrow_mut().isVisible = true;
                                        }
                                    }
                                    _ => {}
                                }
                            });
                        }
                    },
                    _ =>{}
                }
            }
            _ => {}
        }
    }));
    start
}

fn create_bg(tc : &TextureCreator<WindowContext>) ->Sprite
{
    let bg_texture = create_texture!("resource/bg2.png",tc);
    let src = Rect::new(0,1000,W,H);
    let dst = Rect::new(0,0,W,H);
    let mut sprite = Sprite::new(Some(src),Some(dst),bg_texture,"bg");

    sprite.isVisible = false;
    sprite.setUpdateFunc(Box::new(|delatime:f32,s:&Sprite|{
        if s.is_visible(){
            let ptr = s.getRefMut();
            let t_y:i32 = if let Some(ref r) = (*s).src{
                r.y()
            }else {
                0i32
            };

            unsafe {
                if BGY <= 0.0f32{
                    BGY = 1000f32;
                }else{
                    BGY -= 0.5f32 //* delatime;
                }

                if let Some(ref mut rect) = (*ptr).src{
                    rect.set_y(BGY as i32);
                }
            }
        }
    }));

    sprite
}

fn create_plane(tc : &TextureCreator<WindowContext>,
                sps : Weak<RefCell<Vec<RefCell<Box<DH <Target=WindowCanvas>>>>>>,
                buffer : Weak<RefCell<Vec<RefCell<Box<DH <Target=WindowCanvas>>>>>>) -> Plane
{
    let plane_te:Texture = create_texture!("resource/plane.png",tc);
    let mut plane = Plane::new((W as i32 / 2) as f32,(H as i32 / 2) as f32,94u32 / 3u32,127u32/3u32,plane_te,"plane");
    plane.isVisible = false;

    plane.setUpdateFunc(Box::new(|delatime:f32,p:&Plane|{
        if p.is_visible(){
            let mut vec:(f32,f32) = unsafe { (MOUSE_POS.0 as f32 - p.x(),MOUSE_POS.1 as f32 - p.y()) };
            if vec.0.abs() > 1f32 || vec.1.abs() > 1f32 {
                vec.0 *= 0.2;
                vec.1 *= 0.2;
            }
            unsafe { (*p.getRefMut()).set_pos((p.x() + vec.0,p.y() + vec.1)); }
        }
    }));

    plane.setEventFunc(Box::new(move |e:&Event,s:&Plane|{
        match *e {
            Event::MouseButtonDown {mouse_btn,..} =>{
                if s.is_visible(){
                    match mouse_btn {
                        MouseButton::Left =>{
                            if let Some(up_sps) = sps.upgrade(){
                                let temp = up_sps.borrow();
                                let mut not_find = true;
                                for i in 0..temp.len(){
                                    let sp_temp = temp[i].borrow();
                                    if sp_temp.tag() == "player_bullet" && !sp_temp.is_visible(){
                                        //println!("find one");
                                        not_find = false;
                                        unsafe {
                                            let temp_bu: &Ref<Box<Bullet>> = std::mem::transmute(&sp_temp);
                                            (*(temp_bu.getRefMut())).set_pos((s.x() as f32,s.y() as f32 - 5.0f32));
                                            (*(temp_bu.getRefMut())).isVisible = true;
                                        }
                                        break;
                                    }
                                }
                                if not_find{
                                    if let Some(buffer_up) = buffer.upgrade() {
                                        let mut temp = (*buffer_up).borrow_mut();
                                        let bullet = create_bullet_player(s.x() as f32,s.y() as f32 - 5.0f32);
                                        temp.push(RefCell::new(Box::new(bullet)));
                                    }
                                }
                            }
                        },
                        _ =>{}
                    }
                }
            },
            _=>{}
        }
    }));
    plane
}

fn create_bullet_player<'a>(x:f32,y:f32) -> Bullet<'a>{
    let texture_ = unsafe{ &(*BULLET_TEX_PTR)};
    let mut bullet = Bullet::new(x,y,10,16,-5f32,texture_,"player_bullet");
    bullet.setUpdateFunc(Box::new(
       |delatime:f32,b:&Bullet|{
           if b.is_visible(){
               let t_y = b.y();
               if t_y  < 0f32 {
                   unsafe { (*b.getRefMut()).isVisible = false;}
               }else{
                   unsafe { (*b.getRefMut()).set_y(t_y + b.vy);}
               }
           }
       }
    ));
    bullet
}


fn main() {
    run(Path::new("resource/cursor.png"));
}
