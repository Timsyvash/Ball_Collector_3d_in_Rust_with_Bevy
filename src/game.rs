use bevy::prelude::*;
use crate::bullets::*;
use crate::player::*;
use crate::states::GameStates;

pub struct GamePlugin;

#[derive(Resource)]
pub struct Count {
    pub value_count: i16
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Count {value_count: 0})
            .add_systems(Update, collision_player_with_bullets.run_if(in_state(GameStates::Play)));
    }
}

fn collision_player_with_bullets(
    mut commands: Commands,
    mut player_query: Query<&mut Transform, (With<Player>, Without<Bullets>)>,
    bullets_query: Query<(Entity, &Transform), (With<Bullets>, Without<Player>)>,
    mut count: ResMut<Count>
) {
    let Ok(player) = player_query.get_single_mut() else {return;};

    for (bullets_entity, bullets_transform) in bullets_query.iter() {
        let distance = player.translation.distance(bullets_transform.translation);

        if distance < 1.75 {
            count.value_count += 1;
            commands.entity(bullets_entity).despawn_recursive();
        }
    }
}
