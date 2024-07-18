// Define Enums
enum Direction {
    North,
    East,
    South,
    West,
}

struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn move_in_direction(&mut self, direction: Direction) {
        match direction {
            Direction::North => self.y += 1,
            Direction::East => self.x += 1,
            Direction::South => self.y -= 1,
            Direction::West => self.x -= 1,
        }
    }
}

fn main() {
    let mut pos = Position { x: 0, y: 0 };

    pos.move_in_direction(Direction::North);
    println!("Current Position: ({}, {})", pos.x, pos.y);

    pos.move_in_direction(Direction::East);
    println!("Current Position: ({}, {})", pos.x, pos.y);

    pos.move_in_direction(Direction::South);
    println!("Current Position: ({}, {})", pos.x, pos.y);

    pos.move_in_direction(Direction::West);
    println!("Current Position: ({}, {})", pos.x, pos.y);
}
