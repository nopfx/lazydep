use sdl3::pixels::Color;
use sdl3::rect::Rect;
use sdl3::render::Canvas;
use sdl3::video::Window;

pub trait Drawable {
    fn get_sprite(&self) -> Color;
    fn get_rect(&self) -> Rect;

    fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(self.get_sprite());
        canvas.draw_rect(self.get_rect().into()).unwrap();
        canvas.fill_rect(self.get_rect()).unwrap();
    }
}
