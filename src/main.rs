
use std::ops::DerefMut;
use std::time::Duration;

use gui_app::core::geometry::Vec2d;
use gui_app::core::item::{Item, ItemHandle, ItemHandleBase};
use gui_app::items::{Rectangle, ColumnLayout, Button};
use gui_app::sdl2impl;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

#[derive(Default, Debug)]
struct RootItem {
    pub sss: i32

}

impl RootItem {
    pub fn do_action(self: &mut Self) {
        println!("AAAAAAAAction");
    }
}

impl Item for RootItem {}

fn main() {

    let root: ItemHandle<RootItem> = ItemHandle::<RootItem>::default()
        .with::<Rectangle, _>(|root|{
            let r = ItemHandle::<Rectangle>::default()
                .with_geometry2d(Vec2d::new(100., 100.), Vec2d::new(200., 200.))
                .with(|parent| {
                    let mut button = ItemHandle::<Button>::default()
                        .with_geometry2d(Vec2d::new(10., 50.), Vec2d::new(180., 50.));


                        let mut x = 0;
                        let x_ref = &mut x;

                        trait AAA {
                            fn aaa(self: &mut Self) {

                            }
                        }
                        impl AAA for i32 {
                            fn aaa(self: &mut Self) {

                            }
                        }

                        let mut mm2 = || x_ref.aaa();

                        let mut mmm = || root.deref_mut();
                        
                        mm2();
                        mmm();

                        button.clicked(||{
                            root.do_action();
                        });


                        let s = &root.sss;


                    //root.sss;
                    button
                });
                r
        });


    println!("root:\n{:?}", root);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
     
    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();
     
    let mut canvas = window.into_canvas().build().unwrap();
     
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();


    let mut renderer = sdl2impl::graphics::Renderer::from(canvas);


    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        renderer.painter.cws.set_draw_color(Color::RGB(i, 64, 255 - i));
        renderer.painter.cws.clear();
        renderer.painter.cws.set_draw_color(Color::RGB(0, 0, 0));

        root.render(&mut renderer);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...
    
        renderer.painter.cws.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}