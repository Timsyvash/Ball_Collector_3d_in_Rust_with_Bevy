use bevy::prelude::*;
use bevy::input::mouse::{MouseMotion, MouseWheel};

use crate::player::*;
use crate::states::*;

#[derive(Resource)]
pub struct CameraController {
    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,
    pub rotate_sensitivity: f32,
    pub zoom_sensitivity: f32,
    pub min_distance: f32,
    pub max_distance: f32,
}

pub struct Camera3dPlugin;

#[derive(Component)]
pub struct Camera;

#[derive(Component)]
pub struct ControlledCamera;

impl Plugin for Camera3dPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(CameraController {
                yaw: 0.0,
                pitch: 0.0,
                distance: 5.0,
                rotate_sensitivity: 0.005,
                zoom_sensitivity: 0.5,
                min_distance: 1.0,
                max_distance: 15.0,
            })
            .add_systems(Startup, (init_camera, setup_ambient_light))
            .add_systems(Update, move_camera.run_if(in_state(GameStates::Play)));
    }
}

pub fn init_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle::default(),
        Camera,
        ControlledCamera
    ));
}

fn move_camera(
    mut motion_events: EventReader<MouseMotion>,
    mut wheel_events: EventReader<MouseWheel>,
    mut ctrl: ResMut<CameraController>,
    target_q: Query<&GlobalTransform, With<Player>>,
    mut cam_q: Query<&mut Transform, (With<Camera>, With<ControlledCamera>, Without<Player>)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let Ok(target_tf) = target_q.get_single() else {
        return;
    };

    let mut delta = Vec2::ZERO;
    for ev in motion_events.read() {
        delta += ev.delta;
    }

    ctrl.yaw -= delta.x * ctrl.rotate_sensitivity;
    ctrl.pitch = (ctrl.pitch - delta.y * ctrl.rotate_sensitivity).clamp(-1.4, 1.4);

    let rotate_speed = 1.5 * time.delta_seconds();
    if keyboard.pressed(KeyCode::Left) {
        ctrl.yaw += rotate_speed;
    }
    if keyboard.pressed(KeyCode::Right) {
        ctrl.yaw -= rotate_speed;
    }
    if keyboard.pressed(KeyCode::Up) {
        ctrl.pitch = (ctrl.pitch + rotate_speed).clamp(-1.4, 1.4);
    }
    if keyboard.pressed(KeyCode::Down) {
        ctrl.pitch = (ctrl.pitch - rotate_speed).clamp(-1.4, 1.4);
    }

    for ev in wheel_events.read() {
        ctrl.distance = (ctrl.distance - ev.y * ctrl.zoom_sensitivity).clamp(ctrl.min_distance, ctrl.max_distance);
    }

    let zoom_speed = 6.0 * time.delta_seconds();
    if keyboard.pressed(KeyCode::PageUp) || keyboard.pressed(KeyCode::Z) {
        ctrl.distance = (ctrl.distance - zoom_speed).clamp(ctrl.min_distance, ctrl.max_distance);
    }
    if keyboard.pressed(KeyCode::PageDown) || keyboard.pressed(KeyCode::X) {
        ctrl.distance = (ctrl.distance + zoom_speed).clamp(ctrl.min_distance, ctrl.max_distance);
    }

    let yaw = ctrl.yaw;
    let pitch = ctrl.pitch;
    let dir = Vec3::new(yaw.sin() * pitch.cos(), pitch.sin(), yaw.cos() * pitch.cos()).normalize_or_zero();
    let offset = dir * ctrl.distance;

    let target_pos = target_tf.translation();

    if let Ok(mut cam_tf) = cam_q.get_single_mut() {
        cam_tf.translation = target_pos + offset;
        cam_tf.look_at(target_pos, Vec3::Y);
    }
}

fn setup_ambient_light(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.5,
    });
}