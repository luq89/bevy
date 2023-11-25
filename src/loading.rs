use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use serde::Deserialize;

use bevy::reflect::{TypePath, TypeUuid};

use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html>
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::Menu),
        )
        .add_systems(OnEnter(GameState::Loading), setup)
        .add_collection_to_loading_state::<_, AudioAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, TextureAssets>(GameState::Loading);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let levelHandle = LevelHandle(asset_server.load("map/trees.level.ron"));
    commands.insert_resource(levelHandle);
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see <https://github.com/NiklasEi/bevy_asset_loader>)

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/flying.ogg")]
    pub flying: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub bevy: Handle<Image>,
    #[asset(path = "textures/github.png")]
    pub github: Handle<Image>,
    #[asset(path = "textures/gabe-idle-run.png")]
    pub gabe: Handle<Image>,
    #[asset(path = "textures/gabe-idle-run.png")]
    pub enemy: Handle<Image>,
    #[asset(path = "textures/Trees.png")]
    pub tree: Handle<Image>,
    #[asset(path = "textures/Ground.png")]
    pub ground: Handle<Image>,
    #[asset(path = "textures/Forest_Props.png")]
    pub forest_props: Handle<Image>,
}

#[derive(Deserialize, TypeUuid, TypePath, Debug)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2bbb"]
pub struct Level {
    pub positions: Vec<[f32; 3]>,
    pub foliage_positions: Vec<[f32; 3]>,
}
#[derive(Resource, Debug)]
pub struct LevelHandle(pub Handle<Level>);
