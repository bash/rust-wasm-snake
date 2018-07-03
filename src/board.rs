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
                    Direction::East => edge >= self.size.0,
                    _ => unimplemented!(),
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
    fn test_snake_touches_edge_east() {
        let snake = SnakeBuilder::new(Direction::East)
            .with_segment(Segment::new(Coordinate(9, 0), Size(1, 4)))
            .with_segment(Segment::new(Coordinate(8, 0), Size(1, 4)))
            .build();

        let board = BoardBuilder::new(Size(10, 10), snake).build();

        assert_eq!(true, board.snake_touches_edge());
    }
}
