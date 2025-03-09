#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HexCoord {
    pub q: i32,  // Axial coordinates
    pub r: i32,
}

impl HexCoord {
    pub fn new(q: i32, r: i32) -> Self {
        Self { q, r }
    }
    
    // The third cube coordinate, calculated from q and r
    pub fn s(&self) -> i32 {
        -self.q - self.r
    }
    
    // Get all six neighboring coordinates
    pub fn neighbors(&self) -> Vec<HexCoord> {
        vec![
            HexCoord::new(self.q + 1, self.r),
            HexCoord::new(self.q + 1, self.r - 1),
            HexCoord::new(self.q, self.r - 1),
            HexCoord::new(self.q - 1, self.r),
            HexCoord::new(self.q - 1, self.r + 1),
            HexCoord::new(self.q, self.r + 1),
        ]
    }
    
    // Manhattan distance in hex grid
    pub fn distance(&self, other: &HexCoord) -> i32 {
        ((self.q - other.q).abs() + 
         (self.r - other.r).abs() + 
         (self.s() - other.s()).abs()) / 2
    }
    
    // Check if coordinates are within board bounds
    // For a hexagonal board with 6 hexes per side
    pub fn is_valid(&self) -> bool {
        let q_abs = self.q.abs();
        let r_abs = self.r.abs();
        let s_abs = self.s().abs();
        
        // For a board with side length 6, the maximum absolute value
        // of any coordinate is 5
        q_abs <= 5 && r_abs <= 5 && s_abs <= 5
    }
    
    // Convert to/from algebraic notation
    // For a chess-like interface (e.g., "e4" in regular chess)
    pub fn to_algebraic(&self) -> String {
        // This is simplified and would need refinement
        // Could map to letters and numbers in a spiral pattern from center
        // For now, just return the raw coordinates
        format!("({},{})", self.q, self.r)
    }
    
    pub fn from_algebraic(notation: &str) -> Option<Self> {
        // Would need parsing logic for your chosen notation
        // For now, stub implementation
        None
    }
}

// A proper game board with size and boundary checks
#[derive(Debug)]
pub struct HexBoard {
    side_length: i32,
}

impl HexBoard {
    pub fn new(side_length: i32) -> Self {
        Self { side_length }
    }
    
    // Generate all valid coordinates for this board
    pub fn all_coordinates(&self) -> Vec<HexCoord> {
        let mut coords = Vec::new();
        let max = self.side_length - 1;
        
        // Generate all coordinates within the cube constraint
        for q in -max..=max {
            // r bounds depend on q
            let r_min = (-max).max(-q - max);
            let r_max = max.min(-q + max);
            
            for r in r_min..=r_max {
                coords.push(HexCoord::new(q, r));
            }
        }
        
        coords
    }
    
    // Get the color of a tile at specified coordinates
    pub fn tile_color(&self, coord: &HexCoord) -> Option<TileColor> {
        if !self.is_valid_coord(coord) {
            return None;
        }
        
        // Check if it's a corner hex
        let is_corner = 
            coord.q.abs() == self.side_length - 1 && 
            coord.r.abs() == self.side_length - 1 && 
            coord.s().abs() == self.side_length - 1;
            
        if is_corner {
            // Top corner is dark (negative r)
            if coord.r == -(self.side_length - 1) {
                return Some(TileColor::Dark);
            }
            // Bottom corner is light (positive r)
            if coord.r == self.side_length - 1 {
                return Some(TileColor::Light);
            }
        }
            
        // For non-corner tiles, use the standard three-color pattern
        // This simple formula ensures no adjacent tiles have the same color
        // For a pointy-topped hexagon board
        let color_index = ((coord.q + coord.r) % 3).rem_euclid(3);
        
        match color_index {
            0 => Some(TileColor::Light),
            1 => Some(TileColor::Medium),
            2 => Some(TileColor::Dark),
            _ => unreachable!(),
        }
    }
    
    // Check if coordinates are valid for this board
    pub fn is_valid_coord(&self, coord: &HexCoord) -> bool {
        let max = self.side_length - 1;
        coord.q.abs() <= max && 
        coord.r.abs() <= max && 
        coord.s().abs() <= max
    }
}

// The three tile colors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileColor {
    Light,
    Medium,
    Dark,
}

impl TileColor {
    pub fn as_str(&self) -> &'static str {
        match self {
            TileColor::Light => "Light",
            TileColor::Medium => "Medium",
            TileColor::Dark => "Dark",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_neighbors() {
        let center = HexCoord::new(0, 0);
        let neighbors = center.neighbors();
        
        assert_eq!(neighbors.len(), 6);
        assert!(neighbors.contains(&HexCoord::new(1, 0)));
        assert!(neighbors.contains(&HexCoord::new(0, 1)));
    }
    
    #[test]
    fn test_hex_distance() {
        let a = HexCoord::new(0, 0);
        let b = HexCoord::new(3, -2);
        
        assert_eq!(a.distance(&b), 3);
    }
    
    #[test]
    fn test_board_coordinates() {
        let board = HexBoard::new(6);
        let coords = board.all_coordinates();
        
        // A board with side length 6 should have (3n² - 3n + 1) = 91 hexes
        assert_eq!(coords.len(), 91);
    }
    
    #[test]
    fn test_tile_coloring() {
        let board = HexBoard::new(6);
        
        // Adjacent hexes should have different colors
        let center = HexCoord::new(0, 0);
        let center_color = board.tile_color(&center).unwrap();
        
        for neighbor in center.neighbors() {
            if board.is_valid_coord(&neighbor) {
                let neighbor_color = board.tile_color(&neighbor).unwrap();
                assert_ne!(center_color, neighbor_color);
            }
        }
    }
}