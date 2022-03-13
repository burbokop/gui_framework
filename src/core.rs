
use core::fmt;

pub mod graphics;
pub mod geometry;

use std::{fmt::Debug, ops::{Deref, DerefMut}};

use matmath::game::{vec3::Vector3, vec2::Vector2};



pub trait Item {
    fn paint(&self, _: &dyn ItemHandleBase, _: &mut dyn graphics::Painter) {}
}

pub trait ItemHandleBase: Debug {
    fn render(&self, r: &mut dyn graphics::Renderer);
    fn pos(&self) -> geometry::Vec3d;
    fn size(&self) -> geometry::Vec3d;
    fn pos2d(&self) -> geometry::Vec2d {
        let pos = self.pos();
        geometry::Vec2d::new(pos.x, pos.y)
    }
    fn size2d(&self) -> geometry::Vec2d {
        let size = self.size();
        geometry::Vec2d::new(size.x, size.y)
    }
}

impl ItemHandleBase for () {
    fn render(&self, _: &mut dyn graphics::Renderer) {}
    fn pos(&self) -> geometry::Vec3d { geometry::Vec3d::new(0., 0., 0.) }
    fn size(&self) -> geometry::Vec3d { geometry::Vec3d::new(0., 0., 0.) }
}

pub struct ItemHandle<'p, I: Item, P: ItemHandleBase = ()> {
    item: I,
    position: geometry::Vec3d,
    size: geometry::Vec3d,
    children: Vec<Box<dyn ItemHandleBase>>,
    parent: Option<&'p P>
}

impl<'p, I: Item + Default, P: ItemHandleBase> Default for ItemHandle<'p, I, P> {
    fn default() -> Self {
        Self { 
            item: Default::default(), 
            position: Vector3::new(0., 0., 0.), 
            size: Vector3::new(0., 0., 0.), 
            children: Default::default(),
            parent: None 
        }
    }
}

impl<'p, I: Item, P: ItemHandleBase> Debug for ItemHandle<'p, I, P> {
    default fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<no Debug impl> -> {}\n", self.children.len())?;
        for c in self.children.iter() {
            write!(f, "{:?}", &(**c))?;
        }
        fmt::Result::Ok(())
    }
}

impl<'p, I: Item, P: ItemHandleBase> Debug for ItemHandle<'p, I, P> where I: Debug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {        
        write!(f, "<item({:?})> -> {}\n", self.item, self.children.len())?;
        for c in self.children.iter() {
            write!(f, "{:?}", &(**c))?;
        }
        fmt::Result::Ok(())
    }
}

impl<'p, I: Item, P: ItemHandleBase> ItemHandleBase for ItemHandle<'p, I, P> {
    fn render(&self, r: &mut dyn graphics::Renderer) {
        r.move_pen(self.position.clone());
        r.set_bounds(self.size.clone());
        self.item.paint(self, r.painter());
        for c in self.children.iter() {
            (**c).render(r)
        }
        r.move_pen(self.position.clone().scaled(-1.));
    }

    fn pos(&self) -> geometry::Vec3d { self.position.clone() }

    fn size(&self) -> geometry::Vec3d { self.size.clone() }
}

impl<'p, I: Item, P: ItemHandleBase> ItemHandle<'p, I, P> {

    pub fn parent(&self) -> Option<&'p P> {
        self.parent
    }

    pub fn with<C, F>(self, f: F) -> ItemHandle<'p, I, P>
    where
        C: Item + 'static,
        F: Fn(&Self) -> ItemHandle<'_, C, Self>
    {
        let r = f(&self);
        self.with_child(r)
    }

    pub fn with_children<C: Item + 'static>(self, children: Vec<ItemHandle<'static, C, Self>>) -> ItemHandle<'p, I, P> {
        ItemHandle {
            children: self
                .children
                .into_iter()
                .chain(children.into_iter().map(|x| -> Box<dyn ItemHandleBase> { Box::new(x) }))
                .collect(),
            ..self
        }
    }

    pub fn with_child<C: Item + 'static>(self, child: ItemHandle<'static, C, Self>) -> ItemHandle<'p, I, P> {
        self.with_children(vec![child])
    }

    pub fn with_pos(self, position: geometry::Vec3d) -> ItemHandle<'p, I, P> {
        ItemHandle { position: position, ..self }
    }

    pub fn with_size(self, size: geometry::Vec3d) -> ItemHandle<'p, I, P> {
        ItemHandle { size: size, ..self }
    }

    pub fn with_geometry(self, position: geometry::Vec3d, size: geometry::Vec3d) -> ItemHandle<'p, I, P> {
        ItemHandle { position: position, size: size, ..self }
    }

    pub fn with_pos2d(self, position: geometry::Vec2d) -> ItemHandle<'p, I, P> {
        self.with_pos(geometry::Vec3d::new(position.x, position.y, 0.))
    }

    pub fn with_size2d(self, size: Vector2<f64>) -> ItemHandle<'p, I, P> {
        self.with_size(geometry::Vec3d::new(size.x, size.y, 0.))
    }

    pub fn with_geometry2d(self, position: geometry::Vec2d, size: geometry::Vec2d) -> ItemHandle<'p, I, P> {
        self.with_geometry(geometry::Vec3d::new(position.x, position.y, 0.), geometry::Vec3d::new(size.x, size.y, 0.))
    }
}

impl<'p, I: Item, P: ItemHandleBase> Deref for ItemHandle<'p, I, P> {
    type Target = I;

    fn deref(&self) -> &Self::Target {
        &self.item
    }
}

impl<'p, I: Item, P: ItemHandleBase> DerefMut for ItemHandle<'p, I, P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        & mut self.item
    }
}