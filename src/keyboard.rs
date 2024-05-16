use bevy::prelude::*;

use crate::{board::{Board, Inventory, LineCount}, game::Game, tile::{Color, Face, Selected}, SIZE_X, SIZE_Y};

pub fn keyboard_system(
    mut game: Query<&mut Game>,
    mut board: Query<&mut Board>,
    mut selected: Query<&mut Selected>,
    mut inventory: Query<(&mut Inventory, &Color)>, 
    mut line_counts: ResMut<LineCount>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyF) {
        let mut g = game.single_mut();
        g.first_move();
        let (_, b, (red, black)) = g.get_mut_move();
        *board.single_mut() = b.clone();
        *inventory.iter_mut().find(|e| *e.1 == Color::Red).unwrap().0 = red.clone();
        *inventory.iter_mut().find(|e| *e.1 == Color::Black).unwrap().0 = black.clone();
        selected.single_mut().face = None;
        line_counts.0 = b.get_lines();
    }
    if keyboard_input.just_pressed(KeyCode::KeyP) {
        let mut g = game.single_mut();
        g.prev_move();
        let (_, b, (red, black)) = g.get_mut_move();
        *board.single_mut() = b.clone();
        *inventory.iter_mut().find(|e| *e.1 == Color::Red).unwrap().0 = red.clone();
        *inventory.iter_mut().find(|e| *e.1 == Color::Black).unwrap().0 = black.clone();
        selected.single_mut().face = None;
        line_counts.0 = b.get_lines();
    }
    if keyboard_input.just_pressed(KeyCode::KeyN) {
        let mut g = game.single_mut();
        g.next_move();
        let (_, b, (red, black)) = g.get_mut_move();
        *board.single_mut() = b.clone();
        *inventory.iter_mut().find(|e| *e.1 == Color::Red).unwrap().0 = red.clone();
        *inventory.iter_mut().find(|e| *e.1 == Color::Black).unwrap().0 = black.clone();
        selected.single_mut().face = None;
        line_counts.0 = b.get_lines();
    }
    if keyboard_input.just_pressed(KeyCode::KeyL) {
        let mut g = game.single_mut();
        g.last_move();
        let (_, b, (red, black)) = g.get_mut_move();
        *board.single_mut() = b.clone();
        *inventory.iter_mut().find(|e| *e.1 == Color::Red).unwrap().0 = red.clone();
        *inventory.iter_mut().find(|e| *e.1 == Color::Black).unwrap().0 = black.clone();
        selected.single_mut().face = None;
        line_counts.0 = b.get_lines();
    }
}