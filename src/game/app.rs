use crate::game::objects::{Positions, Tile};
use crate::utils::matrix::{draw_matrix, move_object};
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::render::Canvas;
use sdl3::video::Window;

pub struct GameApp {
    pub walls: Vec<Vec<Tile>>,
    pub objects: Vec<Vec<Tile>>,
    running: bool,
}

impl GameApp {
    pub fn new(walls: Vec<Vec<Tile>>, objects: Vec<Vec<Tile>>) -> Self {
        Self {
            walls,
            objects,
            running: true,
        }
    }

    pub fn run(&mut self) {
        let sdl_context = sdl3::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("rust-sdl3 demo", 800, 600)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas();

        while self.running {
            let mut event_pump = sdl_context.event_pump().unwrap();

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => self.running = false,
                    Event::KeyDown {
                        keycode: Some(Keycode::K),
                        ..
                    } => move_object(
                        &self.walls,
                        &mut self.objects,
                        &Positions::Up,
                        &Tile::Player,
                    ),
                    Event::KeyDown {
                        keycode: Some(Keycode::J),
                        ..
                    } => move_object(
                        &self.walls,
                        &mut self.objects,
                        &Positions::Down,
                        &Tile::Player,
                    ),
                    Event::KeyDown {
                        keycode: Some(Keycode::L),
                        ..
                    } => move_object(
                        &self.walls,
                        &mut self.objects,
                        &Positions::Right,
                        &Tile::Player,
                    ),
                    Event::KeyDown {
                        keycode: Some(Keycode::H),
                        ..
                    } => move_object(
                        &self.walls,
                        &mut self.objects,
                        &Positions::Left,
                        &Tile::Player,
                    ),
                    _ => {}
                }
            }
            self.render(&mut canvas);
        }
    }

    fn render(&mut self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        draw_matrix(canvas, &self.walls);
        draw_matrix(canvas, &self.objects);
        canvas.present();
    }
}
