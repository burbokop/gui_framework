use std::ops::Add;

use crate::core::geometry::Vec3d;
use crate::core::graphics::Painter as CorePainter;
use crate::core::graphics::Renderer as CoreRenderer;


pub struct Renderer<T: sdl2::render::RenderTarget> {
    pub painter: Painter<T>
}

impl<'a, T: sdl2::render::RenderTarget> From<sdl2::render::Canvas<T>> for Renderer<T> {
    fn from(c: sdl2::render::Canvas<T>) -> Self {
        Renderer {
            painter: Painter {             
                cws: c, 
                position: Vec3d::new(0., 0., 0.), 
                bounds: Vec3d::new(0., 0., 0.), 
                fill: false,
            }
        }
    }
}

impl<'a, T: sdl2::render::RenderTarget> CoreRenderer for Renderer<T> {
    fn move_pen(&mut self, vec: crate::core::geometry::Vec3d) {
        //println!("move: {}", vec);
        self.painter.position = self.painter.position.clone().add(vec);
    }

    fn set_bounds(&mut self, wh: crate::core::geometry::Vec3d) {
        //println!("set_bounds: {}", wh);
        self.painter.bounds = wh
    }

    fn painter<'b>(&'b mut self) -> &'b mut dyn CorePainter {
        &mut self.painter
    }
}

pub struct Painter<T: sdl2::render::RenderTarget> {
    pub cws: sdl2::render::Canvas<T>,
    position: Vec3d,
    bounds: Vec3d,
    fill: bool,
}

impl<T: sdl2::render::RenderTarget> Painter<T> {
    fn bound_rect(&self) -> sdl2::rect::Rect {
        sdl2::rect::Rect::new(
            self.position.x as i32, 
            self.position.y as i32, 
            self.bounds.x as u32, 
            self.bounds.y as u32
        )
    }
}

impl<T: sdl2::render::RenderTarget> CorePainter for Painter<T> {
    fn draw_rect(&mut self, pos: crate::core::geometry::Vec2d, size: crate::core::geometry::Vec2d) {
        

        
        //println!("bound_rect: {:?}", self.bound_rect());


        self.cws.set_clip_rect(self.bound_rect());


        let rect = sdl2::rect::Rect::new(
            (self.position.x + pos.x) as i32, 
            (self.position.y + pos.y) as i32, 
            size.x as u32, 
            size.y as u32
        );

        //println!("draw_rect: {:?}", rect);


        if self.fill {
            self.cws.fill_rect(rect).unwrap()
        } else {
            self.cws.draw_rect(rect).unwrap()
        }
    }

    fn draw_line(&mut self, pos0: crate::core::geometry::Vec3d, pos1: crate::core::geometry::Vec3d) {
        self.cws.set_clip_rect(sdl2::rect::Rect::new(
            self.position.x as i32, 
            self.position.y as i32, 
            self.bounds.x as u32, 
            self.bounds.y as u32
        ));

        self.cws.draw_line(
            sdl2::rect::Point::new(
                (self.position.x + pos0.x) as i32, 
                (self.position.y + pos0.y) as i32
            ), sdl2::rect::Point::new(
                (self.position.x + pos1.x) as i32, 
                (self.position.y + pos1.y) as i32
            )).unwrap()
        
    }

    fn draw_point(&mut self, pos: crate::core::geometry::Vec3d) {
        self.cws.set_clip_rect(sdl2::rect::Rect::new(
            self.position.x as i32, 
            self.position.y as i32, 
            self.bounds.x as u32, 
            self.bounds.y as u32
        ));
            
        self.cws.draw_point(sdl2::rect::Point::new(
            (self.position.x + pos.x) as i32, 
            (self.position.x + pos.y) as i32
        )).unwrap()        
    }
    
    fn draw_line2d(&mut self, pos0: crate::core::geometry::Vec2d, pos1: crate::core::geometry::Vec2d) {
        self.draw_line(Vec3d::new(pos0.x, pos0.y, 0.), Vec3d::new(pos1.x, pos1.y, 0.))
    }

    fn draw_point2d(&mut self, pos: crate::core::geometry::Vec2d) {
        self.draw_point(Vec3d::new(pos.x, pos.y, 0.))
    }

    fn set_line_width(&mut self, _: f64) {
        //println!("[warn] set_line_width not implemented for sdl2impl::graphics::Painter")
    }

    fn set_fill(&mut self, fill: bool) {
        self.fill = fill;
    }

    fn set_color(&mut self, rgb: u32) {
        self.cws.set_draw_color(sdl2::pixels::Color::from_u32(&(sdl2::pixels::PixelFormatEnum::RGBA32.try_into().unwrap()), rgb));
    }

    fn draw_text(&mut self, pos: crate::core::geometry::Vec2d, str: &str) {
        //self.cws.draw_color()
        //str.
    }
}