use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

pub struct Direction {
    x: i32,
    y: i32,
}

pub const DIRECTION_UP: Direction = Direction { x: 0, y: -1 };
pub const DIRECTION_DOWN: Direction = Direction { x: 0, y: 1 };
pub const DIRECTION_LEFT: Direction = Direction { x: -1, y: 0 };
pub const DIRECTION_RIGHT: Direction = Direction { x: 1, y: 0 };

pub const ALL_DIRECTIONS: [Direction; 4] = [DIRECTION_UP, DIRECTION_DOWN, DIRECTION_LEFT, DIRECTION_RIGHT];

pub struct GridGraph<'a> {
    pub width: u32,
    pub height: u32,
    pub tiles: &'a [&'a [u8]],
}

impl<'a> GridGraph<'a> {
    pub fn new(width: u32, height: u32, tiles: &'a[&'a[u8]]) -> Self {
        Self {
            width,
            height,
            tiles,
        }
    }

    pub fn get_neighbouring_vertex(&self, vertex: &VisitedGraphVertex, direction: Direction) -> Option<GraphVertex> {
        let neighbour_x = vertex.x as i32 + direction.x;
        let neighbour_y = vertex.y as i32 + direction.y;

        if neighbour_x < 0 || neighbour_y < 0 || neighbour_x >= self.width as i32 || neighbour_y >= self.height as i32 {
            return None;
        }

        Some(GraphVertex {
            x: neighbour_x as _,
            y: neighbour_y as _,
        })
    }

    pub fn get_cost(&self, vertex: &GraphVertex) -> u8 {
        self.tiles[vertex.x as usize][vertex.y as usize]
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct GraphVertex {
    pub x: u32,
    pub y: u32,
}

impl GraphVertex {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            x,
            y,
        }
    }

    pub fn into_visited(self, visit_cost: f32) -> VisitedGraphVertex {
        VisitedGraphVertex::new(self.x, self.y, visit_cost)
    }
}

impl From<VisitedGraphVertex> for GraphVertex {
    fn from(vertex: VisitedGraphVertex) -> Self {
        GraphVertex::new(vertex.x, vertex.y)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct VisitedGraphVertex {
    pub x: u32,
    pub y: u32,
    pub cost: f32,
}

impl VisitedGraphVertex {
    pub fn new(x: u32, y: u32, cost: f32) -> Self {
        Self {
            x,
            y,
            cost,
        }
    }
}

impl PartialEq for VisitedGraphVertex {
    fn eq(&self, other: &Self) -> bool {
        (self.x, self.y, self.cost) == (other.x, other.y, other.cost)
    }
}

impl PartialEq<GraphVertex> for VisitedGraphVertex {
    fn eq(&self, other: &GraphVertex) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for VisitedGraphVertex {}

impl PartialOrd for VisitedGraphVertex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cost.partial_cmp(&other.cost).map(|cmp| cmp.reverse())
    }
}

impl Ord for VisitedGraphVertex {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

impl Hash for VisitedGraphVertex {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}
