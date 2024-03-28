#[derive(Copy, Clone)]
pub enum Position {
    Index(usize),
    Cartesian(u32, u32),
}

impl Into<Position> for usize {
    fn into(self) -> Position {
        Position::Index(self)
    }
}

impl Into<Position> for (u32, u32) {
    fn into(self) -> Position {
        Position::Cartesian(self.0, self.1)
    }
}

#[derive(Copy, Clone)]
pub enum Tile {
    Air,
    Wall,
    Player,
}

pub struct Map {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            tiles: vec![Tile::Air; width * height],
            width,
            height,
        }
    }

    fn get_index_from_pos(&self, xy: impl Into<Position>) -> usize {
        match xy.into() {
            Position::Index(i) => i,
            Position::Cartesian(x, y) => y as usize * self.width + x as usize,
        }
    }

    pub fn get_tile(&self, xy: impl Into<Position>) -> Option<&Tile> {
        self.tiles.get(self.get_index_from_pos(xy))
    }

    pub fn get_tile_mut(&mut self, xy: impl Into<Position>) -> Option<&mut Tile> {
        let index = self.get_index_from_pos(xy);
        self.tiles.get_mut(index)
    }
}
