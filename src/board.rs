use crate::entity::{Direction, Food, Object, Obstacle, Size, Snake};

#[derive(Debug)]
pub(crate) struct Board {
    size: Size,
    snake: Snake,
    food: Option<Food>,
    obstacles: Vec<Obstacle>,
}

#[derive(Debug)]
pub(crate) struct BoardBuilder {
    size: Size,
    snake: Snake,
}

impl Board {
    fn snake_touches_edge(&self) -> bool {
        match self.snake.head() {
            None => false,
            Some(segment) => {
                let direction = self.snake.direction();
                let edge = segment.edge(direction);

                match direction {
                    Direction::North => edge <= 0,
                    Direction::East => edge >= self.size.0,
                    Direction::South => edge >= self.size.1,
                    Direction::West => edge <= 0,
                }
            }
        }
    }
}

impl BoardBuilder {
    pub(crate) fn new(size: Size, snake: Snake) -> Self {
        BoardBuilder { size, snake }
    }

    pub(crate) fn build(self) -> Board {
        let BoardBuilder { size, snake } = self;

        Board {
            size,
            snake,
            food: None,
            obstacles: Vec::new(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::entity::{Coordinate, Direction, Segment, Size, SnakeBuilder};

    #[test]
    fn test_snake_touches_edge_north() {
        let snake = SnakeBuilder::new(Direction::North)
            .with_segment(Segment::new(Coordinate(5, 0), Size(4, 1)))
            .with_segment(Segment::new(Coordinate(5, 1), Size(4, 1)))
            .build();

        let board = BoardBuilder::new(Size(10, 10), snake).build();

        assert_eq!(true, board.snake_touches_edge());
    }

    #[test]
    fn test_snake_touches_edge_east() {
        let snake = SnakeBuilder::new(Direction::East)
            .with_segment(Segment::new(Coordinate(9, 0), Size(1, 4)))
            .with_segment(Segment::new(Coordinate(8, 0), Size(1, 4)))
            .build();

        let board = BoardBuilder::new(Size(10, 10), snake).build();

        assert_eq!(true, board.snake_touches_edge());
    }

    #[test]
    fn test_snake_touches_edge_south() {
        let snake = SnakeBuilder::new(Direction::South)
            .with_segment(Segment::new(Coordinate(5, 8), Size(1, 4)))
            .with_segment(Segment::new(Coordinate(5, 9), Size(1, 4)))
            .build();

        let board = BoardBuilder::new(Size(10, 10), snake).build();

        assert_eq!(true, board.snake_touches_edge());
    }

    #[test]
    fn test_snake_touches_edge_west() {
        let snake = SnakeBuilder::new(Direction::West)
            .with_segment(Segment::new(Coordinate(0, 3), Size(4, 1)))
            .with_segment(Segment::new(Coordinate(1, 3), Size(4, 1)))
            .build();

        let board = BoardBuilder::new(Size(10, 10), snake).build();

        assert_eq!(true, board.snake_touches_edge());
    }
}
