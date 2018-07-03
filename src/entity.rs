use std::collections::VecDeque;

pub(crate) trait Object {
    fn position(&self) -> Coordinate;

    fn size(&self) -> Size;

    fn edge(&self, direction: Direction) -> u64 {
        let position = self.position();
        let size = self.size();

        match direction {
            Direction::North => position.1,
            Direction::East => position.0 + size.0,
            Direction::South => position.1 + size.1,
            Direction::West => position.0,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct Coordinate(pub u64, pub u64);

#[derive(Debug, Copy, Clone)]
pub(crate) struct Size(pub u64, pub u64);

#[derive(Debug)]
pub(crate) struct Snake {
    direction: Direction,
    segments: VecDeque<Segment>,
    consumed_score: u64,
}

#[derive(Debug)]
pub(crate) struct Segment {
    position: Coordinate,
    size: Size,
}

#[derive(Debug)]
pub(crate) struct Food {
    position: Coordinate,
    score: u64,
}

#[derive(Debug)]
pub(crate) struct Obstacle {
    position: Coordinate,
    size: Size,
}

pub(crate) struct SnakeBuilder {
    direction: Direction,
    segments: VecDeque<Segment>,
}

impl Snake {
    pub(crate) fn head(&self) -> Option<&Segment> {
        self.segments.get(0)
    }

    pub(crate) fn direction(&self) -> Direction {
        self.direction
    }
}

impl Segment {
    pub(crate) fn new(position: Coordinate, size: Size) -> Self {
        Segment { position, size }
    }
}

impl Object for Segment {
    fn position(&self) -> Coordinate {
        self.position
    }

    fn size(&self) -> Size {
        self.size
    }
}

impl SnakeBuilder {
    pub(crate) fn new(direction: Direction) -> Self {
        SnakeBuilder {
            direction,
            segments: VecDeque::new(),
        }
    }

    pub(crate) fn with_segment(mut self, segment: Segment) -> Self {
        self.segments.push_back(segment);
        self
    }

    pub(crate) fn build(self) -> Snake {
        let Self {
            direction,
            segments,
        } = self;

        Snake {
            direction,
            segments,
            consumed_score: 0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct ObjectMock {
        size: Size,
        position: Coordinate,
    }

    impl Object for ObjectMock {
        fn size(&self) -> Size {
            self.size
        }

        fn position(&self) -> Coordinate {
            self.position
        }
    }

    #[test]
    fn test_object_edge_north() {
        let object = ObjectMock {
            size: Size(2, 2),
            position: Coordinate(16, 40),
        };

        assert_eq!(40, object.edge(Direction::North));
    }

    #[test]
    fn test_object_edge_east() {
        let object = ObjectMock {
            size: Size(2, 2),
            position: Coordinate(16, 40),
        };

        assert_eq!(18, object.edge(Direction::East));
    }

    #[test]
    fn test_object_edge_south() {
        let object = ObjectMock {
            size: Size(2, 2),
            position: Coordinate(16, 40),
        };

        assert_eq!(42, object.edge(Direction::South));
    }

    #[test]
    fn test_object_edge_west() {
        let object = ObjectMock {
            size: Size(2, 2),
            position: Coordinate(16, 40),
        };

        assert_eq!(16, object.edge(Direction::West));
    }
}
