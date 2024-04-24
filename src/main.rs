mod board;
mod game;
mod keyboard;
mod tile;
mod mouse;

use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};
use board::{update_screen, Board, Index, Inventory};
use game::Game;
use keyboard::keyboard_system;
use mouse::mouse_click_system;
use tile::{Color, Face, Selected, Tile};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (mouse_click_system, keyboard_system, update_screen).chain())
        .run();
}

const GAP: f32 = 1.15;
const SIZE_X: u32 = 7;
const SIZE_Y: u32 = 7;
const MIN_SIZE: u32 = [SIZE_X, SIZE_Y][(SIZE_X < SIZE_Y) as usize];
const MIN_SPACING: f32 = 600. / MIN_SIZE as f32;
const BOARD_WIDTH: f32 = MIN_SPACING * SIZE_X as f32;
const BOARD_HEIGHT: f32 = MIN_SPACING * SIZE_Y as f32;
const CARD_SIZE: f32 = 600. / MIN_SIZE as f32 - GAP;
const BLOCK_SIZE: f32 = (600. / MIN_SIZE as f32 - GAP) * 0.27;
const FONT_SIZE: f32 = CARD_SIZE as f32 * 8./9.;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    _asset_server: Res<AssetServer>
) {
    let board = Board::new(SIZE_X, SIZE_Y);
    let red_text_style = TextStyle {
        font: default(),
        font_size: 60.,
        color: bevy::render::color::Color::rgba(1., 0., 0., 1.)
    };
    let black_text_style = TextStyle {
        font: default(),
        font_size: 60.,
        color: bevy::render::color::Color::rgba(0.5, 0.5, 0.5, 1.)
    };
    let card_text_style = TextStyle {
        font: default(),
        font_size: 60.,
        color: bevy::render::color::Color::rgba(0., 0., 0., 1.)
    };
    let tile_text_style = TextStyle {
        font: default(),
        font_size: FONT_SIZE,
        color: bevy::render::color::Color::rgba(0., 0., 0., 1.)
    };
    
    commands.spawn(Camera2dBundle::default());

    for i in 0..(SIZE_X * SIZE_Y) as usize {
        let x = i % SIZE_X as usize;
        let y = i / SIZE_X as usize;

        let color = match board.get_tile(x as u32, y as u32).ok().unwrap() {
            Tile::None => bevy::render::color::Color::rgba(0., 0., 0., 1.),
            Tile::Blocked => bevy::render::color::Color::rgba(1., 1., 0., 1.0),
            Tile::Card(_, Color::Black) => bevy::render::color::Color::rgba(0.5, 0.5, 0.5, 1.0),
            Tile::Card(_, Color::Red) => bevy::render::color::Color::rgba(1., 0., 0., 1.),
            Tile::Card(_, Color::Both) => bevy::render::color::Color::rgba(1., 0., 1., 1.)
        };
        
        let shape = match board.get_tile(x as u32, y as u32).ok().unwrap() {
            Tile::None => Mesh2dHandle(meshes.add(Rectangle::new(CARD_SIZE, CARD_SIZE))),
            Tile::Blocked => Mesh2dHandle(meshes.add(Circle::new(BLOCK_SIZE))),
            Tile::Card(..) => Mesh2dHandle(meshes.add(Rectangle::new(CARD_SIZE, CARD_SIZE)))
        };

        commands.spawn((MaterialMesh2dBundle {
            mesh: shape,
            material: materials.add(color),
            transform: Transform::from_xyz(
                // Distribute shapes from -X_EXTENT to +X_EXTENT.
                -(MIN_SPACING * SIZE_X as f32) / 2. + x as f32 / (SIZE_X - 1) as f32 * (MIN_SPACING * SIZE_X as f32),
                -(MIN_SPACING * SIZE_Y as f32) / 2. + y as f32 / (SIZE_Y - 1) as f32 * (MIN_SPACING * SIZE_Y as f32),
                0.0,
            ),
            ..default()
        }, Index(i as usize)));
        commands.spawn((Text2dBundle {
            text: Text::from_section(match board.get_tile(x as u32, y as u32).ok().unwrap() {
                Tile::Card(Face::Wild, ..) => "W",
                Tile::Card(Face::Ace, ..) => "A",
                Tile::Card(Face::King, ..) => "K",
                Tile::Card(Face::Queen, ..) => "Q",
                Tile::Card(Face::Jack, ..) => "J",
                _ => " "
            }, tile_text_style.clone()),
            transform: Transform::from_xyz(
                // Distribute shapes from -X_EXTENT to +X_EXTENT.
                -(MIN_SPACING * SIZE_X as f32) / 2. + x as f32 / (SIZE_X - 1) as f32 * (MIN_SPACING * SIZE_X as f32),
                -(MIN_SPACING * SIZE_Y as f32) / 2. + y as f32 / (SIZE_Y - 1) as f32 * (MIN_SPACING * SIZE_Y as f32),
                1.,
            ),
            ..default()
        }, Index(i as usize)));
    }
    
    for i in 0..4 {
        commands.spawn(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(75., 75.))),
            material: materials.add(bevy::render::color::Color::rgba(1., 0., 0., 1.)),
            transform: Transform::from_xyz(-500., 240. - (i * 160) as f32, 0.),
            ..default()
        });
        commands.spawn((Text2dBundle {
            text: Text::from_section(match i {
                0 => "A",
                1 => "K",
                2 => "Q",
                3 => "J",
                _ => unreachable!()
            }, card_text_style.clone()),
            transform: Transform::from_xyz(-500., 240. - (i * 160) as f32, 1.),
            ..default()
        }, UI));
    }
    commands.spawn(
        (Text2dBundle {
            text: Text::from_section("A * * *", red_text_style.clone()),
            text_anchor: bevy::sprite::Anchor::Center,
            transform: Transform::from_xyz(500., 300., 0.),
            ..default()
        },
        Color::Red, 
        Face::Ace
    ));
    commands.spawn(
        (Text2dBundle {
            text: Text::from_section("K * * *", red_text_style.clone()),
            text_anchor: bevy::sprite::Anchor::Center,
            transform: Transform::from_xyz(500., 252., 0.),
            ..default()
        },
        Color::Red, 
        Face::King
    ));
    commands.spawn(
        (Text2dBundle {
            text: Text::from_section("Q * * *", red_text_style.clone()),
            text_anchor: bevy::sprite::Anchor::Center,
            transform: Transform::from_xyz(500., 204., 0.),
            ..default()
        },
        Color::Red, 
        Face::Queen
    ));
    commands.spawn(
        (Text2dBundle {
            text: Text::from_section("J * * *", red_text_style.clone()),
            text_anchor: bevy::sprite::Anchor::Center,
            transform: Transform::from_xyz(500., 156., 0.),
            ..default()
        },
        Color::Red, 
        Face::Jack
    ));
    commands.spawn(
        (Text2dBundle {
            text: Text::from_section("A * * *", black_text_style.clone()),
            text_anchor: bevy::sprite::Anchor::Center,
            transform: Transform::from_xyz(500., -156., 0.),
            ..default()
        },
        Color::Black, 
        Face::Ace
    ));
    commands.spawn(
        (Text2dBundle {
            text: Text::from_section("K * * *", black_text_style.clone()),
            text_anchor: bevy::sprite::Anchor::Center,
            transform: Transform::from_xyz(500., -204., 0.),
            ..default()
        },
        Color::Black, 
        Face::King
    ));
    commands.spawn(
        (Text2dBundle {
            text: Text::from_section("Q * * *", black_text_style.clone()),
            text_anchor: bevy::sprite::Anchor::Center,
            transform: Transform::from_xyz(500., -252., 0.),
            ..default()
        },
        Color::Black, 
        Face::Queen
    ));
    commands.spawn(
        (Text2dBundle {
            text: Text::from_section("J * * *", black_text_style.clone()),
            text_anchor: bevy::sprite::Anchor::Center,
            transform: Transform::from_xyz(500., -300., 0.),
            ..default()
        },
        Color::Black, 
        Face::Jack
    ));
    commands.spawn(board);
    commands.spawn(Game::new(SIZE_X, SIZE_Y));
    commands.spawn((Inventory::new(3), Color::Black));
    commands.spawn((Inventory::new(3), Color::Red));
    commands.spawn(Selected {face: None});
}

#[derive(Component)]
pub struct UI;