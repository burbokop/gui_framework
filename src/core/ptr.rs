use std::ops::Deref;

use super::{geometry::{Vec3d, Vec2d}, item::{ItemHandleBase, Item, ItemHandle}};




//pub struct ItemPtr<'p, I> {
//    d: Box<ItemHandle<'_, I, ()>>
//}

    
    //fn with<C, F>(self, f: F) -> Self
    //where
    //    C: Ptr<Parent = Self>,
    //    F: Fn(&Self::Handle) -> C {
    //        let r = f(self.deref());
    //        self.with_child(r)
    //    }
/* 

    fn with_children<C: Item + 'static, CC: ItemPtr<ItemHandle<'static, Self::HandledItem, Self::Handle>> + 'static>(self, children: Vec<CC>) -> Self;
    
    fn with_child<C: Item + 'static, CC: ItemPtr<ItemHandle<'static, Self::HandledItem, Self::Handle>> + 'static>(self, child: CC) -> Self {
        self.with_children(vec![child])
    }

    fn with_pos(self, position: Vec3d) -> Self;
    fn with_size(self, size: Vec3d) -> Self;

    fn with_geometry(self, position: Vec3d, size: Vec3d) -> Self {
        self
            .with_pos(position)
            .with_size(size)
    }

    fn with_pos2d(self, position: Vec2d) -> Self {
        self.with_pos(Vec3d::new(position.x, position.y, 0.))
    }

    fn with_size2d(self, size: Vec2d) -> Self {
        self.with_size(Vec3d::new(size.x, size.y, 0.))
    }

    fn with_geometry2d(self, position: Vec2d, size: Vec2d) -> Self {
        self.with_geometry(Vec3d::new(position.x, position.y, 0.), Vec3d::new(size.x, size.y, 0.))
    }
}
*/
/* 
impl<I> ItemPtr<I> {

    fn with_children<C: Item + 'static, CC: ItemPtr<ItemHandle<'static, Self::HandledItem, Self::Handle>>>(self, children: Vec<CC>) -> Self {
        self.add_children(children);
        self
    }

    fn with_pos(self, position: Vec3d) -> Self {
        todo!()
    }

    fn with_size(self, size: Vec3d) -> Self {
        todo!()
    }
}*/




struct Node<'p> {
    data: String,
    parent: Option<&'p Node<'p>>
}

impl<'p> Node<'p> {
    fn new(data: String, parent: &'p Node<'p>) -> Self {
        Self {
            data: data,
            parent: Some(parent)
        }
    }
}


fn a() {


}