use std::f32::consts::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::camera::init_camera;
use crate::states::*;

#[derive(Component)]
pub struct Player {
    speed: f32,
    rotation_speed: f32,
    grounded: bool,
    vel_y: f32
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameStates::Play), init_player.before(init_camera))
            .add_systems(Update, player_physics.run_if(in_state(GameStates::Play)));
    }
}

pub fn init_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_query: Query<Entity, With<Player>>,
) {
    if !player_query.is_empty() {
        return;
    }

    commands.spawn((
        Player {
            speed: 10.0,
            rotation_speed: 5.0,
            grounded: true,
            vel_y: 0.0
        },
        InGameEntity,
        Name::new("Player"),
        RigidBody::KinematicPositionBased,
        Sleeping::default(),
        KinematicCharacterController {
            autostep: Some(CharacterAutostep {
                max_height: CharacterLength::Absolute(0.5),
                min_width: CharacterLength::Absolute(0.2),
                include_dynamic_bodies: true,
            }),
            snap_to_ground: Some(CharacterLength::Absolute(0.2)),
            ..default()
        },
        SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.0, 0.0)),
    )).with_children(|parent| {
        parent.spawn(SceneBundle {
            scene: asset_server.load("Player.gltf#Scene0"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        });
    });
}

fn player_physics(
    time: Res<Time>,
    key_code: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Player, &mut Transform),
        With<Player>>,
) {
    let Ok((mut player_control, mut player_transform)) =
        player_query.get_single_mut() else {return;};

    let mut dir = Vec3::ZERO;

    if key_code.pressed(KeyCode::W) || key_code.pressed(KeyCode::Up) {
        dir.z -= 1.0;
    }

    if key_code.pressed(KeyCode::S) || key_code.pressed(KeyCode::Down) {
        dir.z += 1.0;
    }

    if key_code.pressed(KeyCode::A) || key_code.pressed(KeyCode::Left) {
        dir.x -= 1.0;
    }

    if key_code.pressed(KeyCode::D) || key_code.pressed(KeyCode::Right) {
        dir.x += 1.0;
    }

    let gravity = 9.8;
    let del = time.delta_seconds();

    if key_code.just_pressed(KeyCode::Space) && player_control.grounded {
        player_control.vel_y += 10.0;
        player_control.grounded = false;
    }

    player_control.vel_y -= gravity * del;

    player_transform.translation.y += player_control.vel_y * del;

    if player_transform.translation.y <= 0.5 {
        player_transform.translation.y = 0.5;
        player_control.vel_y = 0.0;
        player_control.grounded = true;
    }

    if dir != Vec3::ZERO {
        let direction = dir.normalize_or_zero();

        player_transform.translation += direction * player_control.speed * time.delta_seconds();

        let target_angle = direction.x.atan2(direction.z) + PI;

        let target_rotation = Quat::from_rotation_y(target_angle);

        player_transform.rotation = player_transform.rotation.slerp(
            target_rotation,
            player_control.rotation_speed * time.delta_seconds()
        );
    }
}