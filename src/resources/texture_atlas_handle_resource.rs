use bevy::prelude::*;

#[derive(Resource)]
pub struct TextureAtlasHandle {
    pub farm: Handle<TextureAtlas>,
    pub plants: Handle<TextureAtlas>,
}
