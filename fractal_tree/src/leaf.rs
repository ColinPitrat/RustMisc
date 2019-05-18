use dc::DrawingContext;
use point::Point;
use rand::Rng;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;

pub struct Leaf {
    pos: Point,
    size: f64,
    fall_wait: i32,
    falling: bool,
}

impl Leaf {
    pub fn new(pos: Point) -> Leaf{
        let (min_size, max_size) = (3.0, 10.0);
        let r : f64 = rand::thread_rng().gen();
        let size = (max_size - min_size) * r + min_size;

        let (min_wait, max_wait) = (0, 1000);
        let fall_wait = rand::thread_rng().gen_range(min_wait, max_wait);

        let falling = false;
        Leaf{pos, size, fall_wait, falling}
    }

    pub fn fall(&mut self) {
        self.falling = true;
    }

    pub fn animate(&mut self) {
        if self.falling {
            if self.fall_wait > 0 {
                self.fall_wait -= 1
            } else {
                self.pos.y += 10.0;
                // TODO: Do not hardcode the screen size
                if self.pos.y + self.size > 1400.0 {
                    self.pos.y = 1400.0 - self.size
                }
            }
        }
    }

    pub fn display(&self, dc: &mut DrawingContext) {
        let color = Color::RGB(0, 128, 0);
        let ry = self.size as i16;
        let rx = ry * 3;
        let x = self.pos.x as i16 + rx;
        let y = self.pos.y as i16;
        dc.canvas.filled_ellipse(x, y, rx, ry, color).unwrap();
    }
}
