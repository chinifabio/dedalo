use bevy::prelude::*;

#[derive(Resource)]
struct BoardCellColors {
    light: Color,
    dark: Color,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Chess".to_string(),
                resolution: (600., 600.).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(BoardCellColors {
            light: Color::rgb(0.75, 0.75, 0.75),
            dark: Color::rgb(0.25, 0.25, 0.25),
        })
        .add_systems(Startup, (setup, draw_board, draw_king).chain())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn draw_board(
    mut commands: Commands,
    board_cell_colors: Res<BoardCellColors>,
    windows: Query<&mut Window>,
) {
    let win = windows.single();
    let dx = win.resolution.width() / 8.0;
    let dy = win.resolution.height() / 8.0;
    for i in 0..8 {
        for j in 0..8 {
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: if (i + j) % 2 == 0 {
                        board_cell_colors.light
                    } else {
                        board_cell_colors.dark
                    },
                    custom_size: Some(Vec2::new(dx, dy)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(
                    (i as f32 + 0.5) * dx - win.resolution.width() / 2.0,
                    (j as f32 + 0.5) * dy - win.resolution.height() / 2.0,
                    0.0,
                )),
                ..default()
            });
        }
    }
}

fn draw_king(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("sprites/K.png"),
        ..default()
    });
}
