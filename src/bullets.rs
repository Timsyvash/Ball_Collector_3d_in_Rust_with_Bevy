use bevy::prelude::*;
use rand::*;
use crate::states::*;

pub struct BulletsPlugin;

#[derive(Component)]
pub struct Bullets;

#[derive(Resource)]
pub struct MaxCountBullets {
    pub value_max_count_bullets: i16
}

impl Plugin for BulletsPlugin {
    fn build(&self, app: &mut App) {
        let mut rng = thread_rng();
        let rng_count = rng.gen_range(3..10);
        app
            .insert_resource(MaxCountBullets {value_max_count_bullets: rng_count})
            .add_systems(OnEnter(GameStates::Play), init_bullets);
    }
}

pub fn init_bullets(
    mut commands: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    max_bullets: Res<MaxCountBullets>,
    bullets_query: Query<Entity, With<Bullets>>
) {
    if !bullets_query.is_empty() {
        return;
    }

    commands.spawn((
        PbrBundle {
            mesh: mesh.add(Mesh::from(shape::Plane::from_size(200.0))),
            material: materials.add(Color::rgb(0.2, 0.4, 0.2).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        InGameEntity
    ));

    let mut positions: Vec<Vec3> = Vec::new();

    let mut rng = thread_rng();

    for _ in 0..max_bullets.value_max_count_bullets {
        let pos = loop {
            let candidate = Vec3::new(
                rng.gen_range(-20.0..20.0),
                rng.gen_range(2.0..7.0),
                rng.gen_range(-20.0..-5.0),
            );

            let valid = positions
                .iter()
                .all(|p| p.distance(candidate) > 3.0);

            if valid {
                break candidate;
            }
        };

        positions.push(pos);

        let rng_color = Color::rgb(
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0),
        );

        commands.spawn((
            PbrBundle {
                mesh: mesh.add(Mesh::from(shape::UVSphere {
                    radius: 1.0,
                    sectors: 16,
                    stacks: 8,
                })),
                material: materials.add(rng_color.into()),
                transform: Transform::from_translation(pos),
                ..default()
            },
            Bullets,
            InGameEntity,
        ));
    }
}