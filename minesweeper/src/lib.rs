use core::panic;
use std::ops::RangeBounds;

struct Square {
    x: usize,
    y: usize,
    value: u8,
}

impl Square {
    pub fn new(x: usize, y: usize, value: u8) -> Self {
        Square { x, y, value }
    }
}

struct AdjacentSquares<'a> {
    origin: Square,
    minefield: &'a Board<'a>,
    current: Square,
}

impl<'a> AdjacentSquares<'a> {
    fn new(square: Square, minefield: &'a Board) -> Self {
        for y in square.y - 1..=square.y + 1 {
            for x in square.x - 1..=square.x + 1 {
                if y == square.y && x == square.x {
                    continue;
                }

                if let Some(value) = minefield.square(Some(x), Some(y)) {
                    return AdjacentSquares {
                        origin: square,
                        minefield,
                        current: Square::new(x, y, value),
                    };
                }
            }
        }

        panic!("failed to find starting square");
    }
}

impl<'a> Iterator for AdjacentSquares<'a> {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        // We start in upper left of origin and go
        // clockwise, ending when we're one to the right of
        // origin

        // General algorithm is: return current, update current, if current
        // already is one right of origin, set current to None
        todo!()
    }
}

struct Board<'a> {
    minefield: &'a [&'a str],
}

impl<'a> Board<'a> {
    fn solve(&self) -> Vec<String> {
        let mut result = Vec::with_capacity(self.height());

        for y in 0..self.height() {
            let mut annotated_row = String::with_capacity(self.width());
            for x in 0..self.width() {
                match self
                    .square(Some(x), Some(y))
                    .expect("looped out of minefiled")
                {
                    b'*' => annotated_row.push_str("*"),
                    b' ' => match self.num_adjacent_bombs(x, y) {
                        0 => annotated_row.push_str(" "),
                        num => annotated_row.push_str(&num.to_string()),
                    },
                    c => panic!("invalid character {c} in minefiled"),
                };
            }

            result.push(annotated_row);
        }

        result
    }

    fn width(&self) -> usize {
        if self.height() == 0 {
            return 0;
        }

        self.minefield[0].len()
    }

    fn height(&self) -> usize {
        self.minefield.len()
    }

    fn square(&self, x: Option<usize>, y: Option<usize>) -> Option<u8> {
        let (Some(x), Some(y)) = (x, y) else {
            return None;
        };

        if x > self.width() - 1 || y > self.height() - 1 {
            None
        } else {
            Some(self.minefield[y].as_bytes()[x])
        }
    }

    fn num_adjacent_bombs(&self, x: usize, y: usize) -> usize {
        let mut sum = 0;

        match self.above_and_left_of(x, y) {
            Some(square) if square == b'*' => sum += 1,
            _ => {}
        };

        match self.above(x, y) {
            Some(square) if square == b'*' => sum += 1,
            _ => {}
        };

        match self.above_and_right_of(x, y) {
            Some(square) if square == b'*' => sum += 1,
            _ => {}
        };

        match self.left_of(x, y) {
            Some(square) if square == b'*' => sum += 1,
            _ => {}
        };

        match self.right_of(x, y) {
            Some(square) if square == b'*' => sum += 1,
            _ => {}
        };

        match self.below_and_left_of(x, y) {
            Some(square) if square == b'*' => sum += 1,
            _ => {}
        };

        match self.below(x, y) {
            Some(square) if square == b'*' => sum += 1,
            _ => {}
        };

        match self.below_and_right_of(x, y) {
            Some(square) if square == b'*' => sum += 1,
            _ => {}
        };

        sum
    }

    fn above_and_left_of(&self, x: usize, y: usize) -> Option<u8> {
        self.square(x.checked_sub(1), y.checked_sub(1))
    }

    fn above(&self, x: usize, y: usize) -> Option<u8> {
        self.square(Some(x), y.checked_sub(1))
    }

    fn above_and_right_of(&self, x: usize, y: usize) -> Option<u8> {
        self.square(x.checked_add(1), y.checked_sub(1))
    }

    fn left_of(&self, x: usize, y: usize) -> Option<u8> {
        self.square(x.checked_sub(1), Some(y))
    }

    fn right_of(&self, x: usize, y: usize) -> Option<u8> {
        self.square(x.checked_add(1), Some(y))
    }

    fn below_and_left_of(&self, x: usize, y: usize) -> Option<u8> {
        self.square(x.checked_sub(1), y.checked_add(1))
    }

    fn below(&self, x: usize, y: usize) -> Option<u8> {
        self.square(Some(x), y.checked_add(1))
    }

    fn below_and_right_of(&self, x: usize, y: usize) -> Option<u8> {
        self.square(x.checked_add(1), y.checked_add(1))
    }
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.is_empty() {
        return Vec::new();
    }

    Board { minefield }.solve()
}
