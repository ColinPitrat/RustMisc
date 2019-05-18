use dc::DrawingContext;
use leaf::Leaf;
use point::Point;
use rand::Rng;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;

pub struct Branch {
    begin: Point,
    end: Point,
    size: f64,
    angle: f64,
    child_created: bool,
    generation: isize,
    thickness: f64,
    randomness: f64,
}

impl Branch {
    pub fn new(begin: Point, end: Point, randomness: f64) -> Branch{
        let size = Point::distance(&begin, &end);
        let dx = end.x - begin.x;
        let dy = end.y - begin.y;
        let angle = dy.atan2(dx);
        let child_created = false;
        let thickness = 64.0;
        let generation = 0;
        let randomness = randomness;
        Branch{begin, end, size, angle, child_created, generation, thickness, randomness}
    }

    fn random(&self) -> f64 {
        let min = 1.0 - self.randomness;
        let max = 1.0 + self.randomness;
        let r : f64 = rand::thread_rng().gen();
        min + r * (max - min)
    }

    fn child(&mut self, ratio: f64, angle: f64, mult: f64) -> Branch{
        let child_angle = self.angle + mult * angle * self.random();
        let size = self.size * ratio * self.random();
        let x = self.end.x + size * child_angle.cos();
        let y = self.end.y + size * child_angle.sin();
        let thickness = self.thickness * ratio;
        self.child_created = true;
        Branch{
            begin: self.end.clone(),
            end: Point::new(x, y),
            size, 
            angle: child_angle,
            child_created: false,
            generation: self.generation + 1,
            thickness,
            randomness: self.randomness,
        }
    }

    pub fn leaf(&self) -> Vec<Leaf>{
        if self.child_created {
            vec!()
        } else {
            vec!(Leaf::new(self.end.clone()))
        }
    }

    pub fn children(&mut self, ratio: f64, angle: f64) -> Vec<Branch>{
        // Safeguard: prevent creating more than 12 generations (8k branches)
        if self.child_created || (ratio*self.thickness) < 1.0 || self.generation > 12 {
            vec!()
        } else {
            vec!(
                    self.child(ratio, angle, 1.0),
                    self.child(ratio, angle, -1.0)
                )
        }
    }

    pub fn display(&self, dc: &mut DrawingContext) {
        let color = Color::RGB(0, 0, 0);
        let x1 = self.begin.x as i16;
        let y1 = self.begin.y as i16;
        let x2 = self.end.x as i16;
        let y2 = self.end.y as i16;
        let thickness = self.thickness as u8;
        dc.canvas.thick_line(x1, y1, x2, y2, thickness, color).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let cases = vec!(
                (Point::new(0.0, 0.0), Point::new(0.0, 0.0), 0.0, 0.0),
                (Point::new(0.0, 0.0), Point::new(1.0, 0.0), 1.0, 0.0),
                (Point::new(0.0, 0.0), Point::new(0.0, 1.0), 1.0, 1.5707963267948966),
                (Point::new(0.0, 0.0), Point::new(-1.0, 0.0), 1.0, 3.141592653589793),
                (Point::new(0.0, 0.0), Point::new(0.0, -1.0), 1.0, -1.5707963267948966),
                (Point::new(0.0, 0.0), Point::new(1.0, 1.0), (2.0 as f64).sqrt(), 0.7853981633974483),
                (Point::new(0.0, 0.0), Point::new(3.0, 4.0), 5.0, 0.9272952180016122),
                (Point::new(4.0, 4.0), Point::new(3.0, 4.0), 1.0, 3.141592653589793),
                );
        for c in cases {
            let b = Branch::new(c.0.clone(), c.1.clone(), 0.0);
            assert_eq!(b.size, c.2, "Unexpected size between {:?} and {:?}, got {}, expected {}", c.0, c.1, b.size, c.2);
            assert_eq!(b.angle, c.3, "Unexpected angle between {:?} and {:?}, got {}, expected {}", c.0, c.1, b.angle, c.3);
        }
    }
}
