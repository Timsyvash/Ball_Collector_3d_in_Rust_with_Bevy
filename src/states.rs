use bevy::prelude::*;
use rand::{thread_rng, Rng};
use crate::bullets::*;
use crate::game::*;

#[derive(States, Default, Debug, Eq, Hash, Copy, Clone, PartialEq)]
pub enum GameStates {
    Play,
    Win,
    Pause,
    #[default]
    NotStarted,
}

#[derive(Component)]
struct MainMenu;

#[derive(Component)]
struct NotStartedStruct;

pub struct GameStatesPlugin;

#[derive(Component)]
pub struct InGameEntity;

#[derive(Component)]
pub struct Win;

#[derive(Component)]
pub struct Pause;

impl Plugin for GameStatesPlugin {
    fn build(&self, app: &mut App) {
    app.add_state::<GameStates>()
        .add_systems(OnEnter(GameStates::NotStarted), setup_menu)
        .add_systems(OnEnter(GameStates::NotStarted), start_game)
        .add_systems(Update, wait_for_start.run_if(in_state(GameStates::NotStarted)))

        .add_systems(
            Update,
            pause.run_if(
                in_state(GameStates::Play)
                    .or_else(in_state(GameStates::Pause))
            )
        )

        .add_systems(Update, win.run_if(in_state(GameStates::Play)))

        .add_systems(
            OnEnter(GameStates::Win),
            (
                cleanup_game_objects,
                show_win_screen,
            )
        )
        .add_systems(Update, restart_game.run_if(in_state(GameStates::Win)))

        .add_systems(OnExit(GameStates::Win), (
            cleanup_game_objects,
            cleanup_win,
            reset_game,
        ))
        .add_systems(OnExit(GameStates::NotStarted), cleanup_menu);
    }
}

fn wait_for_start(
    keys: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<GameStates>>,
) {
    if keys.just_pressed(KeyCode::N) {
        next_state.set(GameStates::Play);
    }
}

fn start_game(
    keys: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<GameStates>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(50.0),
                height: Val::Percent(50.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                left: Val::Percent(25.0),
                top: Val::Percent(25.0),
                ..default()
            },
            ..default()
        },
        NotStartedStruct
    )).with_children(|p| {
        p.spawn(TextBundle {
            text: Text::from_section(
                "Натисніть на N, щоб почати гру".to_string(),
                TextStyle {
                    font: asset_server.load("Mariupol-Regular.otf"),
                    font_size: 40.0,
                    color: Color::WHITE
                },
            ),
            ..default()
        });
    });
    if keys.just_pressed(KeyCode::N) {
        next_state.set(GameStates::Play);
    }
}

fn cleanup_game_objects(
    mut commands: Commands,
    query: Query<Entity, With<InGameEntity>>
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn cleanup_menu(
    mut commands: Commands,
    menu_query: Query<Entity, With<MainMenu>>,
    text_query: Query<Entity, With<NotStartedStruct>>,
) {
    for e in menu_query.iter() {
        commands.entity(e).despawn_recursive();
    }

    for e in text_query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

fn setup_menu(
    mut commands: Commands,
) {
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ..default()
        },
        MainMenu,
    ));
}

fn pause(
    key_code: Res<Input<KeyCode>>,
    current_state: Res<State<GameStates>>,
    mut next_state: ResMut<NextState<GameStates>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    pause_query: Query<Entity, With<Pause>>
) {
    if key_code.just_pressed(KeyCode::P) {
        if *current_state == GameStates::Play {
            next_state.set(GameStates::Pause);
            commands.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        position_type: PositionType::Absolute,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                },
                Pause
            )).with_children(|p| {
                p.spawn(
                    TextBundle {
                        text: Text::from_section(
                            "Пауза! Натисніть на P, щоб продовжити гру".to_string(),
                            TextStyle {
                                font: asset_server.load("Mariupol-Regular.otf"),
                                font_size: 20.0,
                                color: Color::WHITE
                            },
                        ),
                        ..default()
                    }
                );
            });
        } else if *current_state == GameStates::Pause {
            next_state.set(GameStates::Play);
            if let Ok(pause_entity) = pause_query.get_single() {
                commands.entity(pause_entity).despawn_recursive();
            }
        }
    }
}

fn win(
    count: Res<Count>,
    max_count_bullets: Res<MaxCountBullets>,
    mut next_state: ResMut<NextState<GameStates>>,
) {
    if count.value_count >= max_count_bullets.value_max_count_bullets {
        next_state.set(GameStates::Win);
    }
}

fn restart_game(
    key_code: Res<Input<KeyCode>>,
    current_state: Res<State<GameStates>>,
    mut next_state: ResMut<NextState<GameStates>>,
    mut commands: Commands,
    query: Query<Entity, With<Win>>
) {
    if key_code.just_pressed(KeyCode::R) && *current_state.get() == GameStates::Win {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        next_state.set(GameStates::Play);
    }
}

fn reset_game(
    mut count: ResMut<Count>,
    mut max_count: ResMut<MaxCountBullets>
) {
    count.value_count = 0;
    
    let mut rng = thread_rng();
    max_count.value_max_count_bullets = rng.gen_range(3..10);
}

fn cleanup_win(
    mut commands: Commands,
    query: Query<Entity, With<Win>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn show_win_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        },
        Win,
    ))
        .with_children(|p| {
            p.spawn(TextBundle {
                text: Text::from_section(
                    "Перемога! Натисніть R для рестарту",
                    TextStyle {
                        font: asset_server.load("Mariupol-Regular.otf"),
                        font_size: 40.0,
                        color: Color::WHITE,
                    },
                ),
                ..default()
            });
        });
}