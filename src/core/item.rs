
use core::fmt;


use std::{fmt::Debug, ops::{Deref, DerefMut}, cell::{RefCell, Ref, RefMut}};

use matmath::game::{vec3::Vector3, vec2::Vector2};

use super::{graphics::{Painter, Renderer}, geometry::{Vec2d, Vec3d}};



pub trait Item {
    fn paint(&self, _: &dyn ItemHandleBase, _: &mut dyn Painter) {}
}

pub trait ItemHandleBase: Debug {    
    fn render(&self, r: &mut dyn Renderer);
    fn pos(&self) -> Vec3d;
    fn size(&self) -> Vec3d;
    fn pos2d(&self) -> Vec2d {
        let pos = self.pos();
        Vec2d::new(pos.x, pos.y)
    }
    fn size2d(&self) -> Vec2d {
        let size = self.size();
        Vec2d::new(size.x, size.y)
    }
}

pub struct ItemHandle<I: Item> {
    item: I,
    position: Vec3d,
    size: Vec3d,
    children: Vec<Box<dyn ItemHandleBase>>,
}


impl ItemHandleBase for () {
    fn render(&self, _: &mut dyn Renderer) {}
    fn pos(&self) -> Vec3d { Vec3d::new(0., 0., 0.) }
    fn size(&self) -> Vec3d { Vec3d::new(0., 0., 0.) }
}


impl<I: Item + Default> Default for ItemHandle<I> {
    fn default() -> Self {
        Self { 
            item: Default::default(), 
            position: Vector3::new(0., 0., 0.), 
            size: Vector3::new(0., 0., 0.), 
            children: Default::default(),
        }
    }
}

impl<I: Item> Debug for ItemHandle<I> {
    default fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<no Debug impl> -> {}\n", self.children.len())?;
        for c in self.children.iter() {
            write!(f, "{:?}", &(**c))?;
        }
        fmt::Result::Ok(())
    }
}

impl<I: Item> Debug for ItemHandle<I> where I: Debug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {        
        write!(f, "<item({:?})> -> {}\n", self.item, self.children.len())?;
        for c in self.children.iter() {
            write!(f, "{:?}", &(**c))?;
        }
        fmt::Result::Ok(())
    }
}

impl<I: Item> ItemHandleBase for ItemHandle<I> {
    fn render(&self, r: &mut dyn Renderer) {
        r.move_pen(self.position.clone());
        r.set_bounds(self.size.clone());
        self.item.paint(self, r.painter());
        for c in self.children.iter() {
            (**c).render(r)
        }
        r.move_pen(self.position.clone().scaled(-1.));
    }

    fn pos(&self) -> Vec3d { self.position.clone() }

    fn size(&self) -> Vec3d { self.size.clone() }
}

impl<I: Item + 'static> ItemHandle<I> {


    pub fn with<C, F>(self, f: F) -> Self
    where
        C: Item + 'static,
        F: Fn(&mut Self) -> ItemHandle<C>
    {
        let mut self_mut = self;
        let child = f(&mut self_mut);
        self_mut.with_child(child)
    }


        
    pub fn with_children<C: Item + 'static>(self, children: Vec<ItemHandle<C>>) -> Self {
        ItemHandle {
            children: self
                .children
                .into_iter()
                .chain(children.into_iter().map(|x| -> Box<dyn ItemHandleBase> { Box::new(x) }))
                .collect(),
            ..self
        }
    }
        
    pub fn with_child<C: Item + 'static>(self, child: ItemHandle<C>) -> Self {
        self.with_children(vec![child])
    }

    pub fn with_pos(self, position: Vec3d) -> Self {
        ItemHandle { position: position, ..self }
    }

    pub fn with_size(self, size: Vec3d) -> Self {
        ItemHandle { size: size, ..self }
    }

    pub fn with_geometry(self, position: Vec3d, size: Vec3d) -> Self {
        self
            .with_pos(position)
            .with_size(size)
    }

    pub fn with_pos2d(self, position: Vec2d) -> Self {
        self.with_pos(Vec3d::new(position.x, position.y, 0.))
    }

    pub fn with_size2d(self, size: Vector2<f64>) -> Self {
        self.with_size(Vec3d::new(size.x, size.y, 0.))
    }

    pub fn with_geometry2d(self, position: Vec2d, size: Vec2d) -> Self {
        self.with_geometry(Vec3d::new(position.x, position.y, 0.), Vec3d::new(size.x, size.y, 0.))
    }
}

impl<I: Item> Deref for ItemHandle<I> {
    type Target = I;

    fn deref(&self) -> &Self::Target {
        &self.item
    }
}

impl<I: Item> DerefMut for ItemHandle<I> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        & mut self.item
    }
}

