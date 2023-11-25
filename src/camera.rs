use bevy::prelude::*;
use crate::GameState;
pub struct CameraPlugin;

#[derive(Component)]
pub struct Camera;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), setup_cameras);
    }
}


#[derive(Resource, Default)]
struct Game {
    // board: Vec<Vec<Cell>>,
    // player: Player,
    // bonus: Bonus,
    // score: i32,
    // cake_eaten: u32,
    // camera_should_focus: Vec3,
    // camera_is_focus: Vec3,
}

const BOARD_SIZE_I: usize = 14;
const BOARD_SIZE_J: usize = 21;

const RESET_FOCUS: [f32; 3] = [(BOARD_SIZE_I as f32) / 2.0, 0.0, (BOARD_SIZE_J as f32) / 2.0 - 0.5];

fn setup_cameras(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(
            -((BOARD_SIZE_I as f32) / 2.0),
            (2.0 * (BOARD_SIZE_J as f32)) / 3.0,
            (BOARD_SIZE_J as f32) / 2.0 - 0.5
        ),
        ..default()
    });
    // commands.spawn(Camera3dBundle {
    //     transform: Transform::from_xyz(
    //         -((BOARD_SIZE_I as f32) / 2.0),
    //         (2.0 * (BOARD_SIZE_J as f32)) / 3.0,
    //         (BOARD_SIZE_J as f32) / 2.0 - 0.5
    //     ).looking_at(game.camera_is_focus, Vec3::Y),
    //     ..default()
    // });
}
