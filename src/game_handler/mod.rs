use bevy::prelude::*;

use crate::GameState;
use crate::anton::spawn_anton::SpawnRandomAnton;

pub struct GameHandlerPlugin;
impl Plugin for GameHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup);
        app.add_systems(Update, (
            update
        ).run_if(in_state(GameState::Playing)));
    }
}

const TEST_ANTONS: u32 = 10;

fn setup(mut commands: Commands) {
    for _ in 0..TEST_ANTONS {
        commands.trigger(SpawnRandomAnton);
    }
    
}

fn update(mut commands: Commands, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Space) {
        commands.trigger(SpawnRandomAnton);
    }
}