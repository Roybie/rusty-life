extern crate rand;
extern crate sdl2;
extern crate time;

use std::thread;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::rect::Rect;

use grid::Grid;

mod grid;

fn main() {

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Conway's Game of Life", 800, 600)
        .position_centered()
        //.fullscreen()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();

    renderer.set_draw_color(Color::RGB(255, 0, 0));
    renderer.clear();
    renderer.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut grid = Grid::new();
    grid.seed();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'running
                },
                Event::KeyDown { .. } => {grid.seed();},
                _ => {}
            }
        }

        grid.update();

        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();
        renderer.set_draw_color(Color::RGB(0, 0, 220));

        let scale = grid.scale() as u32;
        for (i, cell) in grid.get_cells().into_iter().enumerate() {
            if *cell {
                let y = scale as usize * (i / grid.width());
                let x = scale as usize * (i % grid.width());
                renderer.fill_rect(Rect::new(x as i32, y as i32 ,scale,scale)).unwrap();
            }
        }
        renderer.present();

        thread::sleep(std::time::Duration::from_millis(100));
    }
}
