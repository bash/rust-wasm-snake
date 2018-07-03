use std::collections::VecDeque;

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

    pub(crate) fn position(&self) -> Coordinate {
        self.position
    }

    pub(crate) fn size(&self) -> Size {
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
