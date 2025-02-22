use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

pub struct GameCameraPlugin;
impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PanOrbitCameraPlugin);
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    //commands.spawn((
    //    Camera2d,
    //    OrthographicProjection {
    //        scaling_mode: ScalingMode::AutoMin { min_width: 1920.0, min_height: 1080.0 },
    //        ..OrthographicProjection::default_2d()
    //    }
    //));

    commands.spawn(PanOrbitCamera::default());
}
