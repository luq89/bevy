use std::borrow::Borrow;

use bevy::prelude::*;

use crate::animate::{animate_sprite, AnimationIndices, AnimationTimer};
use crate::{
    config::Config,
    loading::{Level, LevelHandle, TextureAssets},
    GameState,
};

pub struct TilemapPlugin;

#[derive(Component)]
pub struct Tile;

#[derive(Component)]
pub struct Map {
    width: f32,
    height: f32,
}

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Playing),
            (spawn_foliage, spawn_trees, spawn_background),
        );
    }
}

fn spawn_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<&Config>,
    textures: Res<TextureAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let config = query.single();

    let texture_atlas = TextureAtlas::from_grid(
        textures.ground.clone(),
        config.tilesize_sm,
        8,
        1,
        None,
        None,
    );

    for x in -config.map_x..config.map_x {
        for y in -config.map_y..config.map_y {
            let texture_atlas_handle = texture_atlases.add(texture_atlas.clone());

            commands
                .spawn(
                    (SpriteSheetBundle {
                        texture_atlas: texture_atlas_handle,
                        transform: Transform {
                            translation: Vec3 {
                                x: x as f32 * config.tilesize_sm.x,
                                y: y as f32 * config.tilesize_sm.y,
                                z: config.z_0,
                            },
                            rotation: Quat::default(),
                            scale: Vec3::splat(1.),
                        },
                        ..default()
                    }),
                )
                .insert(Tile);
        }
    }
}

fn spawn_foliage(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    level: Res<LevelHandle>,
    mut levels: ResMut<Assets<Level>>,
    textures: Res<TextureAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    query: Query<(&Config)>,
) {
    let config = query.single();

    if let Some(level) = levels.get(&level.0) {
        let texture_atlas = TextureAtlas::from_grid(
            textures.forest_props.clone(),
            config.tilesize_sm,
            6,
            2,
            None,
            None,
        );

        for position in level.foliage_positions.clone() {
            let texture_atlas_handle = texture_atlases.add(texture_atlas.clone());

            println!("AAAAAAAAA");

            let animation_indices = AnimationIndices { first: 1, last: 1 };

            commands
                .spawn((
                    SpriteSheetBundle {
                        texture_atlas: texture_atlas_handle,
                        sprite: TextureAtlasSprite::new(animation_indices.first),
                        transform: Transform {
                            translation: Vec3 {
                                x: position[0],
                                y: position[1],
                                z: config.z_10,
                            },
                            ..default()
                        },

                        ..default()
                    },
                    animation_indices,
                    AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
                ))
                .insert(Tile);
        }
    }

    // match level.0.clone() {
    //     Some(level) => {
    //         let texture_atlas = TextureAtlas::from_grid(
    //             textures.tree.clone(),
    //             config.tilesize_trees,
    //             6,
    //             2,
    //             None,
    //             None,
    //         );
    //     }
    //     _ => (),
    // }

    // if let Some(level) = levels.remove(level.0.id()) {
    //     let texture_atlas = TextureAtlas::from_grid(
    //         textures.tree.clone(),
    //         config.tilesize_trees,
    //         6,
    //         2,
    //         None,
    //         None,
    //     );

    //     for position in level.positions {
    //         let texture_atlas_handle = texture_atlases.add(texture_atlas.clone());

    //         let animation_indices = AnimationIndices { first: 1, last: 1 };

    //         commands
    //             .spawn((
    //                 SpriteSheetBundle {
    //                     texture_atlas: texture_atlas_handle,
    //                     sprite: TextureAtlasSprite::new(animation_indices.first),
    //                     transform: Transform {
    //                         translation: Vec3 {
    //                             x: position[0],
    //                             y: position[1],
    //                             z: config.z_10,
    //                         },
    //                         ..default()
    //                     },

    //                     ..default()
    //                 },
    //                 animation_indices,
    //                 AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    //             ))
    //             .insert(Tile);
    //     }
    // }
}

fn spawn_trees(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    level: Res<LevelHandle>,
    mut levels: ResMut<Assets<Level>>,
    textures: Res<TextureAssets>,
    mut query: Query<(&Config)>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let config = query.single();

    if let Some(level) = levels.get(&level.0) {
        let texture_atlas = TextureAtlas::from_grid(
            textures.tree.clone(),
            config.tilesize_trees,
            6,
            2,
            None,
            None,
        );

        for position in level.positions.clone() {
            let texture_atlas_handle = texture_atlases.add(texture_atlas.clone());

            let animation_indices = AnimationIndices { first: 1, last: 1 };

            commands
                .spawn((
                    SpriteSheetBundle {
                        texture_atlas: texture_atlas_handle,
                        sprite: TextureAtlasSprite::new(animation_indices.first),
                        transform: Transform {
                            translation: Vec3 {
                                x: position[0],
                                y: position[1],
                                z: config.z_10,
                            },
                            ..default()
                        },

                        ..default()
                    },
                    animation_indices,
                    AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
                ))
                .insert(Tile);
        }
    }
}
