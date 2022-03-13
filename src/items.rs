
use crate::core::{Item, geometry::Vec2d};




#[derive(Debug)]
pub struct Rectangle {
    border_width: f64,
    border_color: u32,
    color: u32
}

impl Default for Rectangle {
    fn default() -> Self {
        Self { border_width: 0., border_color: 0, color: 0xffffffff }
    }
}

impl Rectangle {
    pub fn with_border(self, border_color: u32, border_width: f64) -> Self { 
        Self { border_color: border_color, border_width: border_width, ..self } 
    }
    pub fn with_color(self, color: u32) -> Self {
        Self { color: color, ..self }
    }
}

impl Item for Rectangle {
    fn paint(&self, handle: &dyn crate::core::ItemHandleBase, painter: &mut dyn crate::core::graphics::Painter) {
        painter.set_color(self.color);
        painter.set_fill(true);
        painter.draw_rect(Vec2d::new(0., 0.), handle.size2d());

        if self.border_width > 0. {
            painter.set_fill(false);
            painter.set_line_width(self.border_width);
            painter.set_color(self.border_color);
            painter.draw_rect(Vec2d::new(0., 0.), handle.size2d())
        }
    }
}

#[derive(Default, Debug)]
pub struct ColumnLayout {

}

impl Item for ColumnLayout {}

#[derive(Default, Debug)]
pub struct RowLayout {

}

impl Item for RowLayout {}

#[derive(Default, Debug)]
pub struct StackLayout {

}

impl Item for StackLayout {}

#[derive(Default, Debug)]
pub struct Button {

}

impl Item for Button {
    fn paint(&self, h: &dyn crate::core::ItemHandleBase, p: &mut dyn crate::core::graphics::Painter) {
        let margin = Vec2d::new(h.size().y / 15., h.size().y / 15.);
        p.set_fill(true);
        p.set_color(0xaaaaaaaa);
        p.draw_rect(Vec2d::new(0., 0.), h.size2d());
        p.set_color(0xffffffff);
        p.draw_rect(margin.clone(), h.size2d() - margin.scaled(2.));
        p.set_fill(false);
        p.set_color(0xff000000);        
        p.draw_rect(Vec2d::new(0., 0.), h.size2d());
    }
}
