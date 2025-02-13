use crate::game::drawable::Drawable;
use sdl3::pixels::Color;
use sdl3::rect::Rect;

pub struct Empty {
    pub sprite: Color,
    pub rect: Rect,
}
impl Drawable for Empty {
    fn get_sprite(&self) -> Color {
        self.sprite
    }
    fn get_rect(&self) -> Rect {
        self.rect
    }
}

pub struct Wall {
    pub sprite: Color,
    pub rect: Rect,
}
impl Drawable for Wall {
    fn get_sprite(&self) -> Color {
        self.sprite
    }
    fn get_rect(&self) -> Rect {
        self.rect
    }
}

pub struct Can {
    pub sprite: Color,
    pub rect: Rect,
}
impl Drawable for Can {
    fn get_sprite(&self) -> Color {
        self.sprite
    }
    fn get_rect(&self) -> Rect {
        self.rect
    }
}

pub struct Portal {
    pub sprite: Color,
    pub rect: Rect,
}
impl Drawable for Portal {
    fn get_sprite(&self) -> Color {
        self.sprite
    }
    fn get_rect(&self) -> Rect {
        self.rect
    }
}

pub struct Player {
    pub sprite: Color,
    pub rect: Rect,
}
impl Drawable for Player {
    fn get_sprite(&self) -> Color {
        self.sprite
    }
    fn get_rect(&self) -> Rect {
        self.rect
    }
}

// Tile Enum
pub enum Tile {
    Empty,
    Wall,
    Can,
    Portal,
    Player,
}
