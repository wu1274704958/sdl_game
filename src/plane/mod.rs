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
    pub sprite:Sprite,
    event_func:Option<Box<Fn(&Event,&Plane)>>
}

impl Plane{
    pub fn new(x_:f32,y_:f32,w_:u32,h_:u32,te:Texture,tag:&'static str) -> Plane
    {
        let mut p = Plane{
            x:x_,
            y:y_,
            w:w_,
            h:h_,
            sprite:Sprite::new(None,None,te,tag),
            event_func:None
        };
        p.calc_dst();
        p
    }
    pub fn calc_dst(&mut self)
    {
        self.sprite.dst = Some(Rect::new(self.x as i32 - (self.w / 2) as i32,self.y as i32 - (self.h / 2) as i32,self.w,self.h));
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
        self.sprite.draw(t);
    }
}

impl BV for Plane{
    fn is_visible(&self) -> bool {
        self.sprite.isVisible
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
        self.sprite.tag()
    }
}

impl Update for Plane{
    fn update(&self, delatime: f32) {
        self.sprite.update(delatime);
    }
}

impl DH for Plane{

}