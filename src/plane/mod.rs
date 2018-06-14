use ::sprite::Sprite;
use ::sprite::{DH,Drawable,BV,EventHandle,HasTag,Update};
use ::sdl2::render::WindowCanvas;
use ::sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::render::Texture;


pub struct Plane{
    x:f32,
    y:f32,
    h:u32,
    w:u32,
    pub dst:Option<Rect>,
    texture : Texture,
    pub isVisible : bool,
    event_func:Option<Box<Fn(&Event,&Plane)>>,
    update_func : Option<Box<Fn(f32,&Plane)>>,
    tag: &'static str
}

impl Plane{
    pub fn new(x_:f32,y_:f32,w_:u32,h_:u32,te:Texture,tag_:&'static str) -> Plane
    {
        let mut p = Plane{
            x:x_,
            y:y_,
            w:w_,
            h:h_,
            dst:None,
            texture:te,
            isVisible:true,
            update_func:None,
            event_func:None,
            tag:tag_
        };
        p.calc_dst();
        p
    }
    pub fn calc_dst(&mut self)
    {
        self.dst = Some(Rect::new(self.x as i32 - (self.w / 2) as i32,self.y as i32 - (self.h / 2) as i32,self.w,self.h));
    }
    pub fn set_pos(&mut self,p:(f32,f32)){
        self.x = p.0;
        self.y = p.1;
        self.calc_dst();
    }
    pub fn getRefMut(&self) -> *mut Plane{
        unsafe { (self as *const Plane) as * mut Plane}
    }
    pub fn setEventFunc(&mut self,f : Box<Fn(&Event,&Plane)->()>)
    {
        self.event_func = Some(f);
    }
    pub fn setUpdateFunc(&mut self,f : Box<Fn(f32,&Plane)->()>)
    {
        self.update_func = Some(f);
    }
    pub fn x(&self) -> f32
    {
        self.x
    }
    pub fn y(&self) -> f32
    {
        self.y
    }
}

impl Drawable for Plane{
    type Target = WindowCanvas;

    fn draw(&self, t: &mut <Self as Drawable>::Target) {
        if self.isVisible{
            (*t).copy(&(self.texture), None, self.dst);
        }
    }
}

impl BV for Plane{
    fn is_visible(&self) -> bool {
        self.isVisible
    }

    fn in_bound(&self, p: (i32, i32)) -> bool {
        true
    }
}

impl EventHandle for Plane{
    fn on_handle_event(&self, e: &Event) {
        if let Some(ref f) = self.event_func{
            (*f)(e,self);
        }
    }
}

impl HasTag for Plane{
    fn tag(&self) -> &'static str {
        self.tag
    }
}

impl Update for Plane{
    fn update(&self, delatime: f32) {
        if let Some(ref f) = self.update_func{
            (*f)(delatime,self);
        }
    }
}

impl DH for Plane{

}


pub struct Bullet<'a>{
    x:f32,
    y:f32,
    h:u32,
    w:u32,
    pub vy:f32,
    pub dst:Option<Rect>,
    texture : &'a Texture,
    pub isVisible : bool,
    update_func : Option<Box<Fn(f32,&Bullet)>>,
    tag: &'static str
}

impl<'a> Bullet<'a>{
    pub fn new(x_:f32,y_:f32,w_:u32,h_:u32,vy_:f32,texture_:&'a Texture,tag_:&'static str) ->Bullet<'a>{
        let mut bullet = Bullet{
            x:x_,
            y:y_,
            w:w_,
            h:h_,
            vy:vy_,
            texture:texture_,
            dst:None,
            isVisible : true,
            tag:tag_,
            update_func:None
        };
        bullet.calc_dst();
        bullet
    }
    pub fn calc_dst(&mut self)
    {
        self.dst = Some(Rect::new(self.x as i32 - (self.w / 2) as i32,self.y as i32 - (self.h / 2) as i32,self.w,self.h));
    }
    pub fn getRefMut(&self) -> *mut Bullet{
        unsafe { (self as *const Bullet) as * mut Bullet}
    }
    pub fn setUpdateFunc(&mut self,f : Box<Fn(f32,&Bullet)->()>)
    {
        self.update_func = Some(f);
    }
    pub fn set_pos(&mut self,p:(f32,f32)){
        self.x = p.0;
        self.y = p.1;
        self.calc_dst();
    }
    pub fn x(&self) -> f32
    {
        self.x
    }
    pub fn y(&self) -> f32
    {
        self.y
    }

    pub fn set_y(&mut self,y_:f32)
    {
        self.y = y_;
        self.calc_dst();
    }
}

impl<'a> Drawable for Bullet<'a>{
    type Target = WindowCanvas;

    fn draw(&self, t: &mut <Self as Drawable>::Target) {
        if self.isVisible {
             (*t).copy(self.texture, None, self.dst);
        }
    }
}

impl<'a> EventHandle for Bullet<'a>{
    fn on_handle_event(&self,e: &Event) {

    }
}

impl<'a> BV for Bullet<'a>{
    fn is_visible(&self) -> bool {
        self.isVisible
    }

    fn in_bound(&self, p: (i32, i32)) -> bool {
        false
    }
}

impl<'a> HasTag for Bullet<'a>{
    fn tag(&self) -> &'static str {
        self.tag
    }
}

impl<'a> Update for Bullet<'a>{
    fn update(&self, delatime: f32) {
        if let Some(ref f) = self.update_func{
            (*f)(delatime,self);
        }
    }
}


impl<'a> DH for Bullet<'a>{

}
