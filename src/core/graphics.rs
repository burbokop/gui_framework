use super::geometry::{Vec3d, Vec2d};


pub trait Painter {
    fn set_line_width(&mut self, w: f64);
    fn set_fill(&mut self, fill: bool);
    fn set_color(&mut self, rgb: u32);
    fn draw_rect(&mut self, pos: Vec2d, size: Vec2d);
    fn draw_text(&mut self, pos: Vec2d, str: &str);
    fn draw_line(&mut self, pos0: Vec3d, pos1: Vec3d);
    fn draw_point(&mut self, pos: Vec3d);
    fn draw_line2d(&mut self, pos0: Vec2d, pos1: Vec2d);
    fn draw_point2d(&mut self, pos: Vec2d);
}

pub trait Renderer {
    fn move_pen(&mut self, vec: Vec3d);
    fn set_bounds(&mut self, wh: Vec3d);
    fn painter<'a>(&'a mut self) -> &'a mut dyn Painter;
}
