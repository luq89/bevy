use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;

use bevy::reflect::{TypePath, TypeUuid};
use bevy_asset_loader::prelude::*;
use serde::Deserialize;

use crate::GameState;

pub struct ConfigPlugin;

#[derive(Component, Debug)]

pub struct Config {
    pub plat: f32,
    pub tilesize: Vec2,
    pub tilesize_trees: Vec2,
    pub tilesize_sm: Vec2,
    pub map_x: i16,
    pub map_y: i16,
    pub z_0: f32,
    pub z_10: f32,
}

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Config {
        plat: 2.0,
        tilesize: Vec2::new(24.0, 24.0),
        tilesize_sm: Vec2::new(48.0, 48.0),
        tilesize_trees: Vec2::new(64.0, 88.0),
        map_x: 256,
        map_y: 256,
        z_0: 1.0,
        z_10: 1.1,
    });
    println!("FOOO")
}
