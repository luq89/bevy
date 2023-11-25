use crate::actions::Actions;
use crate::animate::{animate_sprite, AnimationIndices, AnimationTimer};
use crate::config::Config;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;

pub struct EnemyPlugin;

#[derive(Component)]
pub struct Enemy;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_enemy)
            .add_systems(
                Update,
                (
                    move_enemy.run_if(in_state(GameState::Playing)),
                    animate_sprite,
                ),
            );
    }
}

fn spawn_enemy(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut query: Query<(&Config)>,
) {
    let config = query.single();

    let texture_atlas =
        TextureAtlas::from_grid(textures.gabe.clone(), config.tilesize, 7, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let animation_indices = AnimationIndices { first: 1, last: 6 };

    commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(animation_indices.first),
                transform: Transform {
                    translation: Vec3 {
                        x: 50.,
                        y: 5.,
                        z: config.z_10,
                    },
                    ..default()
                },
                ..default()
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ))
        .insert(Enemy);
}

fn move_enemy(
    time: Res<Time>,
    actions: Res<Actions>,
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
) {
}
