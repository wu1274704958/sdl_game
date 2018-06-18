extern crate sdl2;
#[macro_use] extern crate sdl_test;
extern crate rand;

use rand::thread_rng;
use rand::RngCore;

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
static mut ENEMY_TEX_PTR:*const Texture = 0 as *const Texture;
static mut TEXTURE_CREATE_PTR:*const TextureCreator<WindowContext> = 0 as *const TextureCreator<WindowContext>;

static mut MOUSE_POS:(i32,i32) = (0,0);

pub fn run(png: &Path) {
    let sprites : Rc<RefCell<Vec<RefCell<Box<DH <Target=WindowCanvas>>>>>> = Rc::new(RefCell::new(Vec::new()));
    let buffer : Rc<RefCell<Vec<RefCell<Box<DH <Target=WindowCanvas>>>>>> = Rc::new(RefCell::new(Vec::new()));

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _image_context = sdl2::image::init(INIT_PNG | INIT_JPG).unwrap();
    let window = video_subsystem.window("sdl_game", W, H)
        .opengl()
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().software().build().unwrap();
    let texture_creator = canvas.texture_creator();

    unsafe { TEXTURE_CREATE_PTR = &texture_creator as *const TextureCreator<WindowContext>; }

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

    let enemy_texture = create_texture!("resource/enemy.png",texture_creator);
    unsafe {ENEMY_TEX_PTR = &enemy_texture as *const Texture;}

    let start_sprite = create_start(Rc::downgrade(&sprites));
    (*sprites).borrow_mut().push(RefCell::new(Box::new(start_sprite)));

    let bg_sprite = create_bg();
    (*sprites).borrow_mut().push(RefCell::new(Box::new(bg_sprite)));

    let plane_player = create_plane_player(Rc::downgrade(&sprites),Rc::downgrade(&buffer));
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
		//println!("{}",delatime);
        delatime = if delatime < 9f32{
            sleep_ms(9u32 - delatime as u32);
            9f32
        }else{ println!("not sleep!!!");delatime };
    }
    println!("end  {}",Rc::strong_count(&sprites));
}

fn create_start(sps : Weak<RefCell<Vec<RefCell<Box<DH <Target=WindowCanvas>>>>>>) -> Sprite
{
    let tc = unsafe{&(*TEXTURE_CREATE_PTR)};
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

fn create_bg() ->Sprite
{
    let tc = unsafe{&(*TEXTURE_CREATE_PTR)};
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
                    BGY -= 0.02f32 * delatime;
                }

                if let Some(ref mut rect) = (*ptr).src{
                    rect.set_y(BGY as i32);
                }
            }
        }
    }));

    sprite
}

fn create_plane_player(
                sps : Weak<RefCell<Vec<RefCell<Box<DH <Target=WindowCanvas>>>>>>,
                buffer : Weak<RefCell<Vec<RefCell<Box<DH <Target=WindowCanvas>>>>>>) -> Plane
{
    let tc = unsafe{&(*TEXTURE_CREATE_PTR)};
    let plane_te:Texture = create_texture!("resource/plane.png",tc);
    let mut plane = Plane::new((W as i32 / 2) as f32,(H as i32 / 2) as f32,94u32 / 3u32,127u32/3u32,false,plane_te,"plane");
    plane.isVisible = false;

    let sps_clone = sps.clone();
    let buffer_clone = buffer.clone();

    plane.setUpdateFunc(Box::new(move |delatime:f32,p:&Plane|{
        if p.is_visible(){
            let mut vec:(f32,f32) = unsafe { (MOUSE_POS.0 as f32 - p.x(),MOUSE_POS.1 as f32 - p.y()) };
            if vec.0.abs() > 1f32 || vec.1.abs() > 1f32 {
                vec.0 *= (0.02f32 * delatime);
                vec.1 *= (0.02f32 * delatime);
            }
            unsafe { (*p.getRefMut()).set_pos((p.x() + vec.0,p.y() + vec.1)); }
            let n1 = rand::random::<u32>() % 50;
            if n1 == 0{
                create_plane_enemy(Weak::clone(&sps_clone),Weak::clone(&buffer_clone));
            }
        }
    }));

    plane.setEventFunc(Box::new(move |e:&Event,s:&Plane|{
        match *e {
            Event::MouseButtonDown {mouse_btn,..} =>{
                if s.is_visible(){
                    match mouse_btn {
                        MouseButton::Left =>{
                            create_bullet_player(s.x(),s.y(),Weak::clone(&sps),Weak::clone(&buffer));
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

fn create_bullet_player(x:f32,y:f32,
                            sps : Weak<RefCell<Vec<RefCell<Box<DH <Target=WindowCanvas>>>>>>,
                            buffer : Weak<RefCell<Vec<RefCell<Box<DH <Target=WindowCanvas>>>>>>){

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
                    (*(temp_bu.getRefMut())).set_pos((x,y - 25.0f32));
                    (*(temp_bu.getRefMut())).isVisible = true;
                }
                break;
            }
        }
        if not_find{
            if let Some(buffer_up) = buffer.upgrade() {
                let mut temp = (*buffer_up).borrow_mut();

                let texture_ = unsafe{ &(*BULLET_TEX_PTR)};
                let mut bullet = Bullet::new(x,y,10,16,0f32,-0.6f32,0f64,false,texture_,"player_bullet");
                bullet.setUpdateFunc(Box::new(
                    |delatime:f32,b:&Bullet|{
                        if b.is_visible(){
                            let t_y = b.y();
                            if t_y  < 0f32 {
                                unsafe { (*b.getRefMut()).isVisible = false;}
                            }else{
                                unsafe { (*b.getRefMut()).set_y(t_y + (b.vy * delatime));}
                            }
                        }
                    }
                ));

                temp.push(RefCell::new(Box::new(bullet)));
            }
        }
    }
}

fn create_plane_enemy(sps : Weak<RefCell<Vec<RefCell<Box<DH <Target=WindowCanvas>>>>>>,
                      buffer : Weak<RefCell<Vec<RefCell<Box<DH <Target=WindowCanvas>>>>>>){
    let mut rng = thread_rng();
    let x = rng.next_u32() % 300 + 50;

    if let Some(up_sps) = sps.upgrade(){
        let temp = up_sps.borrow();
        let mut not_find = true;
        for i in 0..temp.len(){
            let sp_temp = temp[i].borrow();
            if sp_temp.tag() == "enemy" && !sp_temp.is_visible(){
                //println!("find one");
                not_find = false;
                unsafe {
                    let temp_bu: &Ref<Box<Bullet>> = std::mem::transmute(&sp_temp);
                    (*(temp_bu.getRefMut())).set_pos((x as f32,-36f32));
                    (*(temp_bu.getRefMut())).isVisible = true;
                }
                break;
            }
        }
        if not_find{
            if let Some(buffer_up) = buffer.upgrade() {
                let mut temp = (*buffer_up).borrow_mut();
                let texture_ = unsafe{ &(*ENEMY_TEX_PTR)};
                let mut rng = rand::thread_rng();
                let mut enemy = Bullet::new(x as f32,-50f32,118u32 / 2u32,144u32 / 2,
                                            0f32,(rng.next_u32() % 30 + 5) as f32 * 0.012f32 ,
                                            0f64,false,texture_,"enemy");

                enemy.setUpdateFunc(Box::new(move |delatime:f32,enemy:&Bullet|{
                    if enemy.is_visible(){
                        let t_y = enemy.y();
                        if t_y  > 536f32  {
                            unsafe { (*enemy.getRefMut()).isVisible = false;}
                        }else{
                            unsafe { (*enemy.getRefMut()).set_y(t_y + enemy.vy * delatime);}
                        }
                        let mut rng = thread_rng();


                        if let Some(up_sps) = sps.upgrade() {
                            let ref_vec = up_sps.borrow();


                            if  ref_vec[2].borrow().is_visible() && rng.next_u32() % 100 == 0{
                                let enemy_pos = (enemy.x(),enemy.y());
                                let player_pos = unsafe{
                                    let temp:&Ref<Box<Plane>> = std::mem::transmute(&(ref_vec[2].borrow()));
                                    (temp.x(),temp.y())
                                };

                                let v_pos = {
                                    ((player_pos.0 - enemy_pos.0) * 0.001f32 , (player_pos.1 - enemy_pos.1) * 0.001f32)
                                };

                                let angle = calc_angle(player_pos,enemy_pos);
                                create_bullet_enemy(enemy_pos,v_pos,angle as f64,Weak::clone(&sps),Weak::clone(&buffer));

                            }


                            ref_vec.iter().for_each(|it|{
                                let ref_it = it.borrow();
                                if ref_it.is_visible(){
                                    if ref_it.tag() == "player_bullet"{
                                        unsafe {
                                            let temp:&Ref<Box<Bullet>> = std::mem::transmute(&ref_it);
                                            if enemy.intersection(&(temp.dst)) {
                                                (*temp.getRefMut()).isVisible = false;
                                                (*enemy.getRefMut()).isVisible = false;
                                            }
                                        }
                                    }else if ref_it.tag() == "plane"{
                                        unsafe {
                                            let temp:&Ref<Box<Plane>> = std::mem::transmute(&ref_it);
                                            if enemy.intersection(&(temp.dst)) {
                                                (*temp.getRefMut()).isVisible = false;
                                                {
                                                    let temp:&Ref<Box<Sprite>> = std::mem::transmute(&(ref_vec[0].borrow()));
                                                    (*temp.getRefMut()).isVisible = true;
                                                }
                                                {
                                                    let temp:&Ref<Box<Sprite>> = std::mem::transmute(&(ref_vec[1].borrow()));
                                                    (*temp.getRefMut()).isVisible = false;
                                                }
                                            }
                                        }
                                    }
                                }
                            });
                        }
                    }
                }));
                temp.push(RefCell::new(Box::new(enemy)));
            }
        }
    }
}

fn create_bullet_enemy( pos:(f32,f32),
                        v_pos:(f32,f32),
                        angle:f64,
                        sps : Weak<RefCell<Vec<RefCell<Box<DH <Target=WindowCanvas>>>>>>,
                        buffer : Weak<RefCell<Vec<RefCell<Box<DH <Target=WindowCanvas>>>>>>){

    if let Some(up_sps) = sps.upgrade(){
        let temp = up_sps.borrow();
        let mut not_find = true;
        for i in 0..temp.len(){
            let sp_temp = temp[i].borrow();
            if sp_temp.tag() == "enemy_bullet" && !sp_temp.is_visible(){
                //println!("find one");
                not_find = false;
                unsafe {
                    let temp_bu: &Ref<Box<Bullet>> = std::mem::transmute(&sp_temp);
                    (*(temp_bu.getRefMut())).set_pos((pos.0,pos.1 + 30.0f32));
                    (*(temp_bu.getRefMut())).isVisible = true;
                    (*(temp_bu.getRefMut())).vx = v_pos.0;
                    (*(temp_bu.getRefMut())).vy = v_pos.1;
                    (*(temp_bu.getRefMut())).angle = angle;
                }
                break;
            }
        }
        if not_find{
            if let Some(buffer_up) = buffer.upgrade() {
                let mut temp = (*buffer_up).borrow_mut();

                let texture_ = unsafe{ &(*BULLET_TEX_PTR)};
                let mut bullet = Bullet::new(pos.0, pos.1,10,16,v_pos.0,v_pos.1,angle,true,texture_,"enemy_bullet");
                bullet.setUpdateFunc(Box::new(
                    move |delatime:f32,b:&Bullet|{
                        if b.is_visible(){
                            let t_y = b.y();
                            let t_x = b.x();
                            if t_y  < 0f32 || t_y > (H + 8) as f32 || t_x < 0f32 || t_x > (W + 5) as f32 {
                                unsafe { (*b.getRefMut()).isVisible = false;}
                            }else{
                                unsafe {
                                    (*b.getRefMut()).set_pos((t_x + b.vx * delatime ,t_y + b.vy * delatime));
                                }
                            }
                            if let Some(up_sps) = sps.upgrade() {
                                let ref_vec = up_sps.borrow();
                                ref_vec.iter().for_each(|it|{
                                    let ref_it = it.borrow();
                                    if ref_it.is_visible(){
                                        if ref_it.tag() == "player_bullet" {
                                            unsafe {
                                                let temp:&Ref<Box<Bullet>> = std::mem::transmute(&ref_it);
                                                if b.intersection(&(temp.dst)) {
                                                    (*temp.getRefMut()).isVisible = false;
                                                    (*b.getRefMut()).isVisible = false;
                                                }
                                            }
                                        }else if ref_it.tag() == "plane"{
                                            unsafe {
                                                let temp:&Ref<Box<Plane>> = std::mem::transmute(&ref_it);
                                                if b.intersection(&(temp.dst)) {
                                                    (*temp.getRefMut()).isVisible = false;
                                                    {
                                                        let temp:&Ref<Box<Sprite>> = std::mem::transmute(&(ref_vec[0].borrow()));
                                                        (*temp.getRefMut()).isVisible = true;
                                                    }
                                                    {
                                                        let temp:&Ref<Box<Sprite>> = std::mem::transmute(&(ref_vec[1].borrow()));
                                                        (*temp.getRefMut()).isVisible = false;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                })
                            }
                        }
                    }
                ));
                temp.push(RefCell::new(Box::new(bullet)));
            }
        }
    }
}

fn calc_angle(p1:(f32,f32),p2:(f32,f32)) -> f32{

    if p2.0==p1.0&&p2.1>p1.1 {
        return 180f32;
    }else if p2.0>p1.0&&p2.1==p1.1{
        return 90f32;
    }else if p2.0<p1.0&&p2.1==p1.1{
        return 270f32;
    }

    let x = (p1.0 - p2.0).abs();
    let y = (p1.1 - p2.1).abs();
    let z = (x.powi(2) + y.powi(2)).sqrt();
    let cos = y/z;
    let radina = cos.acos();//用反三角函数求弧度
    let mut angle = (180f32/(std::f32::consts::PI/radina)).floor();//将弧度转换成角度

    if p2.0 > p1.0 && p2.1 > p1.1{
        angle = 180f32 - angle;
    }else if p2.0<p1.0&&p2.1>p1.1{
        angle = 180f32 + angle;
    }else if p2.0<p1.0&&p2.1<p1.1{
        angle = 360f32 - angle;
    }
    angle
}

fn main() {
    run(Path::new("resource/cursor.png"));
}
