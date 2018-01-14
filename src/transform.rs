use sdl2::render::{Canvas, RenderTarget, Texture};
use sdl2::rect::{Rect, Point};


#[derive(Clone)]
pub struct TransformContext {
    x: i32,
    y: i32
}

impl TransformContext {
    pub fn new() -> TransformContext {
        TransformContext {
            x: 0,
            y: 0
        }
    }

    pub fn transform(&self, x: i32, y: i32) -> TransformContext {
        TransformContext {
            x: self.x + x,
            y: self.y + y
        }
    }

    pub fn fill_rect<R: Into<Option<Rect>>, T: RenderTarget>(&self, canvas: &mut Canvas<T>, rect: R) -> Result<(), String> {
        match rect.into() {
            Some(rect) => {
                let updated = Rect::new(rect.x + self.x, rect.y + self.y, rect.w as u32, rect.h as u32);
                canvas.fill_rect(Some(updated))
            },
            None => canvas.fill_rect(None)
        }
    }

    pub fn copy<R1: Into<Option<Rect>>, R2: Into<Option<Rect>>, T: RenderTarget>(&self, canvas: &mut Canvas<T>, texture: &Texture, src: R1, dst: R2) -> Result<(), String> {
        match dst.into() {
            Some(rect) => {
                let updated = Rect::new(rect.x + self.x, rect.y + self.y, rect.w as u32, rect.h as u32);
                canvas.copy(texture, src, updated)
            },
            None => canvas.copy(texture, src, None)
        }
    }

    pub fn copy_ex<R1, R2, P, T>(&self,
                              canvas: &mut Canvas<T>,
                              texture: &Texture,
                              src: R1,
                              dst: R2,
                              angle: f64,
                              center: P,
                              flip_horizontal: bool,
                              flip_vertical: bool)
                              -> Result<(), String>
        where R1: Into<Option<Rect>>,
              R2: Into<Option<Rect>>,
              P: Into<Option<Point>>,
              T: RenderTarget {
        match dst.into() {
            Some(rect) => {
                let updated = Rect::new(rect.x + self.x, rect.y + self.y, rect.w as u32, rect.h as u32);
                canvas.copy_ex(texture, src, updated, angle, center, flip_horizontal, flip_vertical)
            },
            None => canvas.copy_ex(texture, src, None, angle, center, flip_horizontal, flip_vertical)
        }
    }
}
