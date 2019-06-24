pub struct RangeIterator {
    x: i32,
    y: i32,
    range: i32,
    width: i32,
    height: i32,
    d: i32,
    dx: i32,
    dy: i32,
}

impl RangeIterator {
    pub fn new(x: i32, y: i32, range: i32, width: i32, height: i32) -> RangeIterator {
        RangeIterator{x, y, range, width, height, d: 0, dx: 0, dy: 0}
    }
}

impl Iterator for RangeIterator {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.dx < -self.range && self.dy < -self.range {
            None
        } else {
            let r = Some((self.x+self.dx, self.y+self.dy));
            loop {
                if self.dx == self.d && self.dy == self.d {
                    self.d += 1;
                    self.dx = -self.d;
                    self.dy = -self.d;
                } else if (self.dx == -self.d || self.dx == self.d) && self.dy < self.d {
                    self.dy += 1;
                } else if self.dy == -self.d {
                    self.dy = self.d;
                } else {
                    self.dy = -self.d;
                    self.dx += 1;
                }
                // Stop here if finished
                if self.dx < -self.range && self.dy < -self.range {
                    break;
                }
                // Avoid being out of the allowed area
                if self.x + self.dx < 0 {
                    continue;
                }
                if self.y + self.dy < 0 {
                    continue;
                }
                if self.x + self.dx >= self.width {
                    continue;
                }
                if self.y + self.dy >= self.height {
                    continue;
                }
                break;
            }
            r
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_iterator() {
        let mut rg = RangeIterator::new(10, 10, 3, 100, 100);

        // Distance 0
        assert_eq!(Some((10, 10)), rg.next());

        // Distance 1
        assert_eq!(Some((9, 9)), rg.next());
        assert_eq!(Some((9, 10)), rg.next());
        assert_eq!(Some((9, 11)), rg.next());

        assert_eq!(Some((10, 9)), rg.next());
        assert_eq!(Some((10, 11)), rg.next());

        assert_eq!(Some((11, 9)), rg.next());
        assert_eq!(Some((11, 10)), rg.next());
        assert_eq!(Some((11, 11)), rg.next());

        // Distance 2
        assert_eq!(Some((8, 8)), rg.next());
        assert_eq!(Some((8, 9)), rg.next());
        assert_eq!(Some((8, 10)), rg.next());
        assert_eq!(Some((8, 11)), rg.next());
        assert_eq!(Some((8, 12)), rg.next());

        assert_eq!(Some((9, 8)), rg.next());
        assert_eq!(Some((9, 12)), rg.next());
        assert_eq!(Some((10, 8)), rg.next());
        assert_eq!(Some((10, 12)), rg.next());
        assert_eq!(Some((11, 8)), rg.next());
        assert_eq!(Some((11, 12)), rg.next());

        assert_eq!(Some((12, 8)), rg.next());
        assert_eq!(Some((12, 9)), rg.next());
        assert_eq!(Some((12, 10)), rg.next());
        assert_eq!(Some((12, 11)), rg.next());
        assert_eq!(Some((12, 12)), rg.next());

        // Distance 3
        assert_eq!(Some((7, 7)), rg.next());
        assert_eq!(Some((7, 8)), rg.next());
        assert_eq!(Some((7, 9)), rg.next());
        assert_eq!(Some((7, 10)), rg.next());
        assert_eq!(Some((7, 11)), rg.next());
        assert_eq!(Some((7, 12)), rg.next());
        assert_eq!(Some((7, 13)), rg.next());

        assert_eq!(Some((8, 7)), rg.next());
        assert_eq!(Some((8, 13)), rg.next());
        assert_eq!(Some((9, 7)), rg.next());
        assert_eq!(Some((9, 13)), rg.next());
        assert_eq!(Some((10, 7)), rg.next());
        assert_eq!(Some((10, 13)), rg.next());
        assert_eq!(Some((11, 7)), rg.next());
        assert_eq!(Some((11, 13)), rg.next());
        assert_eq!(Some((12, 7)), rg.next());
        assert_eq!(Some((12, 13)), rg.next());

        assert_eq!(Some((13, 7)), rg.next());
        assert_eq!(Some((13, 8)), rg.next());
        assert_eq!(Some((13, 9)), rg.next());
        assert_eq!(Some((13, 10)), rg.next());
        assert_eq!(Some((13, 11)), rg.next());
        assert_eq!(Some((13, 12)), rg.next());
        assert_eq!(Some((13, 13)), rg.next());

        // Finished
        assert_eq!(None, rg.next());
    }

    #[test]
    fn range_iterator_close_to_top_right_edges() {
        let mut rg = RangeIterator::new(1, 1, 3, 100, 100);

        // All the x < 0 and y < 0 are skipped 

        // Distance 0
        assert_eq!(Some((1, 1)), rg.next());

        // Distance 1
        assert_eq!(Some((0, 0)), rg.next());
        assert_eq!(Some((0, 1)), rg.next());
        assert_eq!(Some((0, 2)), rg.next());

        assert_eq!(Some((1, 0)), rg.next());
        assert_eq!(Some((1, 2)), rg.next());

        assert_eq!(Some((2, 0)), rg.next());
        assert_eq!(Some((2, 1)), rg.next());
        assert_eq!(Some((2, 2)), rg.next());

        // Distance 2
        assert_eq!(Some((0, 3)), rg.next());
        assert_eq!(Some((1, 3)), rg.next());
        assert_eq!(Some((2, 3)), rg.next());

        assert_eq!(Some((3, 0)), rg.next());
        assert_eq!(Some((3, 1)), rg.next());
        assert_eq!(Some((3, 2)), rg.next());
        assert_eq!(Some((3, 3)), rg.next());

        // Distance 3
        assert_eq!(Some((0, 4)), rg.next());
        assert_eq!(Some((1, 4)), rg.next());
        assert_eq!(Some((2, 4)), rg.next());
        assert_eq!(Some((3, 4)), rg.next());

        assert_eq!(Some((4, 0)), rg.next());
        assert_eq!(Some((4, 1)), rg.next());
        assert_eq!(Some((4, 2)), rg.next());
        assert_eq!(Some((4, 3)), rg.next());
        assert_eq!(Some((4, 4)), rg.next());

        // Finished
        assert_eq!(None, rg.next());
    }

    #[test]
    fn range_iterator_close_to_bottom_left_edges() {
        let mut rg = RangeIterator::new(10, 10, 3, 12, 12);

        // All the x >= 12 or y >= 12 are skipped.

        // Distance 0
        assert_eq!(Some((10, 10)), rg.next());

        // Distance 1
        assert_eq!(Some((9, 9)), rg.next());
        assert_eq!(Some((9, 10)), rg.next());
        assert_eq!(Some((9, 11)), rg.next());

        assert_eq!(Some((10, 9)), rg.next());
        assert_eq!(Some((10, 11)), rg.next());

        assert_eq!(Some((11, 9)), rg.next());
        assert_eq!(Some((11, 10)), rg.next());
        assert_eq!(Some((11, 11)), rg.next());

        // Distance 2
        assert_eq!(Some((8, 8)), rg.next());
        assert_eq!(Some((8, 9)), rg.next());
        assert_eq!(Some((8, 10)), rg.next());
        assert_eq!(Some((8, 11)), rg.next());

        assert_eq!(Some((9, 8)), rg.next());
        assert_eq!(Some((10, 8)), rg.next());
        assert_eq!(Some((11, 8)), rg.next());

        // Distance 3
        assert_eq!(Some((7, 7)), rg.next());
        assert_eq!(Some((7, 8)), rg.next());
        assert_eq!(Some((7, 9)), rg.next());
        assert_eq!(Some((7, 10)), rg.next());
        assert_eq!(Some((7, 11)), rg.next());

        assert_eq!(Some((8, 7)), rg.next());
        assert_eq!(Some((9, 7)), rg.next());
        assert_eq!(Some((10, 7)), rg.next());
        assert_eq!(Some((11, 7)), rg.next());

        // Finished
        assert_eq!(None, rg.next());
    }
}
