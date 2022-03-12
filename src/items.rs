
use crate::core::{Item, geometry::Vec2d};




#[derive(Default, Debug)]
pub struct Rectangle {
    line_w: f64
}

impl Rectangle {
    pub fn with_line_width(self, line_w: f64) -> Self { Self{ line_w: line_w } }
}

impl Item for Rectangle {
    fn paint(&self, handle: &dyn crate::core::ItemHandleBase, painter: &mut dyn crate::core::graphics::Painter) {
        painter.set_line_width(self.line_w);
        painter.set_color(0x000000);
        painter.draw_rect(Vec2d::new(0., 0.), handle.size2d())
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
