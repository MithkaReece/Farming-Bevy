#[derive(Clone, Copy)]
pub enum TilemapLayer {
    Ground = 0,
    Object = 1,
    Collision = 2,
    EndOfLayers = 3,
}

impl From<u32> for TilemapLayer {
    fn from(value: u32) -> Self {
        match value {
            0 => TilemapLayer::Ground,
            1 => TilemapLayer::Object,
            2 => TilemapLayer::Collision,
            3 => TilemapLayer::EndOfLayers,
            _ => panic!("Invalid u32 value for TilemapLayer"),
        }
    }
}
