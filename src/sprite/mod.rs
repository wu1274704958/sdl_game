use::sdl2::rect::Rect;
use::sdl2::render::Texture;
use::sdl2::render::WindowCanvas;
use::sdl2::event::Event;

pub trait Drawable{
    type Target;
    fn draw(&self,t : &mut Self::Target);
}

pub trait EventHandle{
    fn on_handle_event(&self,e:&Event);
}

pub trait BV{
    fn is_visible(&self) ->bool;
    fn in_bound(&self,p:(i32,i32)) ->bool;
}

pub trait DH : Drawable + EventHandle + BV {

}

pub struct Sprite{
    src:Option<Rect>,
    pub dst:Option<Rect>,
    texture : Texture,
    pub isVisible : bool,
    event_func : Option<Box<Fn(&Event,&Sprite)>>,
    pub tag: &'static str
}

impl Drawable for Sprite{
    type Target = WindowCanvas;

    fn draw(&self, t: &mut Self::Target) {
        if self.isVisible {
            (*t).copy(&(self.texture), self.src, self.dst);
        }
    }
}

impl EventHandle for Sprite{
    fn on_handle_event(&self,e: &Event) {
        if let Some(ref f) = self.event_func{
            (*f)(e,self);
        }
    }
}

impl BV for Sprite{
    fn is_visible(&self) -> bool {
        self.isVisible
    }

    fn in_bound(&self, p: (i32, i32)) -> bool {
        if let Some(ref r) = self.dst{
            return r.contains(p);
        }
        false
    }
}

impl DH for Sprite{

}

impl Sprite{
    pub fn new(src:Option<Rect>,dst_:Option<Rect>,te:Texture,_tag:&'static str)->Sprite{
        Sprite{src:None,dst:dst_,texture:te,isVisible:true,event_func:None,tag:_tag}
    }

    pub fn setEventFunc(&mut self,f : Box<Fn(&Event,&Sprite)->()>)
    {
        self.event_func = Some(f);
    }

    pub fn getRefMut(&self) -> *mut Sprite{
        unsafe { (self as *const Sprite) as * mut Sprite}
    }

}

