/// Display
pub const DISPLAY_WIDTH: usize = 480;
pub const DISPLAY_HEIGHT: usize = 480;
pub const BYTES_PER_PIXEL: usize = 4;

/// Tile map
pub const TILE_MAP_DIMENSION: i32 = 120;
pub const TILE_MAP_MARGIN: i32 = 10;
pub static TILE_NODE_DIMENSION: u32 = TILE_MAP_DIMENSION as u32 - TILE_MAP_MARGIN as u32 *2 ;

/// Node style
pub const NODE_PORT_TEXT_SHIFTING: i32 = 5;
pub const NODE_PORT_DIMENSION: u32 = 5;
