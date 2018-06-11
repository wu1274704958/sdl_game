use::sdl2::rect::Rect;
use::sdl2::render::Texture;
use::sdl2::render::WindowCanvas;
use::sdl2::event::Event;

pub trait Drawable{
    type Target;
    fn draw(&self,t : &mut Self::Target);
}

pub trait EventHandle{
    fn onHandleEvent(e:&Event);
}

pub struct Sprite{
    src:Option<Rect>,
    dst:Option<Rect>,
    texture : Texture
}

impl Drawable for Sprite{
    type Target = WindowCanvas;

    fn draw(&self, t: &mut Self::Target) {
        (*t).copy(&(self.texture),self.src,self.dst);
    }
}

impl Sprite{
    pub fn new(src:Option<Rect>,dst_:Option<Rect>,te:Texture)->Sprite{
        Sprite{src:None,dst:dst_,texture:te}
    }
}

