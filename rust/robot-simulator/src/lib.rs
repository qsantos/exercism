// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, direction: Direction) -> Self {
        Robot { x, y, direction }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        let Robot { x, y, direction } = self;
        let direction = match direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
        Robot { x, y, direction }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        let Robot { x, y, direction } = self;
        let direction = match direction {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        };
        Robot { x, y, direction }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        let Robot { x, y, direction } = self;
        let (x, y) = match direction {
            Direction::North => (x, y + 1),
            Direction::East => (x + 1, y),
            Direction::South => (x, y - 1),
            Direction::West => (x - 1, y),
        };
        Robot { x, y, direction }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        let mut robot = self;
        for c in instructions.bytes() {
            match c {
                b'R' => robot = robot.turn_right(),
                b'L' => robot = robot.turn_left(),
                b'A' => robot = robot.advance(),
                _ => unreachable!(),
            }
        }
        robot
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
