use crate::game::drawable::Drawable;
use crate::game::objects::{Can, Player, Portal, Tile, Wall};

use sdl3::pixels::Color;
use sdl3::rect::Rect;
use sdl3::render::Canvas;
use sdl3::video::Window;

pub fn draw_matrix(canvas: &mut Canvas<Window>, matrix: &Vec<Vec<Tile>>) {
    for (y, row) in matrix.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            let rect = Rect::new((x * 20) as i32, (y * 20) as i32, 20, 20);

            let wall_color = Color::RGB(0, 0, 255);
            let box_color = Color::RGB(0, 255, 0);
            let portal_color = Color::RGB(255, 0, 0);
            let player_color = Color::RGB(77, 0, 77);

            match tile {
                Tile::Wall => {
                    let wall: Wall = Wall {
                        sprite: wall_color,
                        rect: rect,
                    };
                    wall.draw(canvas);
                }
                Tile::Can => {
                    let can: Can = Can {
                        sprite: box_color,
                        rect: rect,
                    };
                    can.draw(canvas);
                }
                Tile::Portal => {
                    let portal: Portal = Portal {
                        sprite: portal_color,
                        rect: rect,
                    };
                    portal.draw(canvas);
                }
                Tile::Player => {
                    let player: Player = Player {
                        sprite: player_color,
                        rect: rect,
                    };
                    player.draw(canvas);
                }
                _ => {}
            }
        }
    }
}
