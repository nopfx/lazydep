use crate::game::drawable::Drawable;
use crate::game::objects::{Can, Player, Portal, Positions, Tile, Wall};

use sdl3::pixels::Color;
use sdl3::rect::Rect;
use sdl3::render::Canvas;
use sdl3::video::Window;

fn find(matrix: &Vec<Vec<Tile>>, what: &Tile) -> Vec<usize> {
    for (y, row) in matrix.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if tile.eq(what) {
                return vec![y as usize, x as usize];
            }
        }
    }
    vec![]
}
pub fn move_object(
    walls: &Vec<Vec<Tile>>,
    matrix: &mut Vec<Vec<Tile>>,
    position: &Positions,
    object: &Tile,
) {
    let object_coords = find(matrix, object);
    let object_data = object.clone();

    let (row, col) = (object_coords[0], object_coords[1]);
    match position {
        Positions::Up => {
            if walls[row - 1][col] == Tile::Empty {
                if matrix[row - 1][col] == Tile::Can && walls[row - 2][col] == Tile::Empty {
                    if matrix[row - 2][col] == Tile::Portal {
                        matrix[row][col] = Tile::Empty;
                        matrix[row - 2][col] = Tile::Empty;
                        matrix[row - 1][col] = object_data;
                    } else {
                        matrix[row][col] = Tile::Empty;
                        matrix[row - 2][col] = Tile::Can;
                        matrix[row - 1][col] = object_data;
                    }
                } else if matrix[row - 1][col] == Tile::Empty {
                    matrix[row][col] = Tile::Empty;
                    matrix[row - 1][col] = object_data;
                }
            }
        }
        Positions::Down => {
            if walls[row + 1][col] == Tile::Empty {
                if matrix[row + 1][col] == Tile::Can && walls[row + 2][col] == Tile::Empty {
                    if matrix[row + 2][col] == Tile::Portal {
                        matrix[row][col] = Tile::Empty;
                        matrix[row + 2][col] = Tile::Empty;
                        matrix[row + 1][col] = object_data;
                    } else {
                        matrix[row][col] = Tile::Empty;
                        matrix[row + 2][col] = Tile::Can;
                        matrix[row + 1][col] = object_data;
                    }
                } else if matrix[row + 1][col] == Tile::Empty {
                    matrix[row][col] = Tile::Empty;
                    matrix[row + 1][col] = object_data;
                }
            }
        }
        Positions::Right => {
            if walls[row][col + 1] == Tile::Empty {
                if matrix[row][col + 1] == Tile::Can && walls[row][col + 2] == Tile::Empty {
                    if matrix[row][col + 2] == Tile::Portal {
                        matrix[row][col] = Tile::Empty;
                        matrix[row][col + 2] = Tile::Empty;
                        matrix[row][col + 1] = object_data;
                    } else {
                        matrix[row][col] = Tile::Empty;
                        matrix[row][col + 2] = Tile::Can;
                        matrix[row][col + 1] = object_data;
                    }
                } else if matrix[row][col + 1] == Tile::Empty {
                    matrix[row][col] = Tile::Empty;
                    matrix[row][col + 1] = object_data;
                }
            }
        }
        Positions::Left => {
            if walls[row][col - 1] == Tile::Empty {
                if matrix[row][col - 1] == Tile::Can && walls[row][col - 2] == Tile::Empty {
                    if matrix[row][col - 2] == Tile::Portal {
                        matrix[row][col] = Tile::Empty;
                        matrix[row][col - 1] = object_data;
                        matrix[row][col - 2] = Tile::Empty;
                    } else {
                        matrix[row][col] = Tile::Empty;
                        matrix[row][col - 2] = Tile::Can;
                        matrix[row][col - 1] = object_data;
                    }
                } else if matrix[row][col - 1] == Tile::Empty {
                    matrix[row][col] = Tile::Empty;
                    matrix[row][col - 1] = object_data;
                }
            }
        }
        _ => {}
    }
}
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
