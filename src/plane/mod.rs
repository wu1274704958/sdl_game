use ::sprite::{DH,Drawable,BV,EventHandle,HasTag,Update};
use ::sdl2::render::WindowCanvas;
use ::sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::render::Texture;
use sdl2::rect::Point;


pub struct Plane{
    x:f32,
    y:f32,
    h:u32,
    w:u32,
    center:Point,
    flip_v:bool,
    pub dst:Option<Rect>,
    texture : Texture,
    pub isVisible : bool,
    event_func:Option<Box<Fn(&Event,&Plane)>>,
    update_func : Option<Box<Fn(f32,&Plane)>>,
    tag: &'static str
}

impl Plane{
    pub fn new(x_:f32,y_:f32,w_:u32,h_:u32,flip_v_:bool,te:Texture,tag_:&'static str) -> Plane
    {
        let mut p = Plane{
            x:x_,
            y:y_,
            w:w_,
            h:h_,
            flip_v:flip_v_,
            center:Point::new(w_ as i32/ 2,h_ as i32 / 2),
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
         (self as *const Plane) as * mut Plane
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
            //(*t).copy(&(self.texture), None, self.dst);
            (*t).copy_ex(&(self.texture), None, self.dst,0f64,self.center,false,self.flip_v);
        }
    }
}

impl BV for Plane{
    fn is_visible(&self) -> bool {
        self.isVisible
    }

    fn in_bound(&self, _: (i32, i32)) -> bool {
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
    flip_v:bool,
    center:Point,
    pub angle:f64,
    pub vx:f32,
    pub vy:f32,
    pub src:Option<Rect>,
    pub dst:Option<Rect>,
    texture : &'a Texture,
    pub isVisible : bool,
    update_func : Option<Box<Fn(f32,&Bullet)>>,
    tag: &'static str
}

impl<'a> Bullet<'a>{
    pub fn new(x_:f32,y_:f32,w_:u32,h_:u32,vx_:f32,vy_:f32,angle_:f64,flip_v_:bool,texture_:&'a Texture,tag_:&'static str) ->Bullet<'a>{
        let mut bullet = Bullet{
            x:x_,
            y:y_,
            w:w_,
            h:h_,
            vx:vx_,
            vy:vy_,
            angle:angle_,
            flip_v:flip_v_,
            center:Point::new(w_ as i32/ 2,h_ as i32 / 2),
            texture:texture_,
            src:None,
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
         (self as *const Bullet) as * mut Bullet
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
    pub fn set_dst(&mut self,p:(f32,f32,u32,u32)){
        self.x = p.0;
        self.y = p.1;
        self.w = p.2;
        self.h = p.3;
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

    pub fn intersection(&self,ortho:&Option<Rect>) ->bool{
        let mut res = false;
        if let Some(ref r) = self.dst{
            if let Some(ortho_r) = *ortho{
                res = (*r).has_intersection(ortho_r);
            }
        }
        res
    }
}

impl<'a> Drawable for Bullet<'a>{
    type Target = WindowCanvas;

    fn draw(&self, t: &mut <Self as Drawable>::Target) {
        if self.isVisible {
             //(*t).copy(self.texture, None, self.dst);
            (*t).copy_ex(self.texture,self.src,self.dst,self.angle,Some(self.center),false,self.flip_v);
        }
    }
}

impl<'a> EventHandle for Bullet<'a>{
    fn on_handle_event(&self,_: &Event) {

    }
}

impl<'a> BV for Bullet<'a>{
    fn is_visible(&self) -> bool {
        self.isVisible
    }

    fn in_bound(&self, _: (i32, i32)) -> bool {
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
