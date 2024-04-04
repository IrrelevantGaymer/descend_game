use bevy::{prelude::*, window::PrimaryWindow};

use crate::{board::{Board, Inventory}, game::Game, tile::{Color, Face, Selected}, SIZE_X, SIZE_Y};

pub fn mouse_click_system(
    window: Query<&Window, With<PrimaryWindow>>,
    mut game: Query<&mut Game>,
    mut board: Query<&mut Board>,
    mut selected: Query<&mut Selected>,
    mut inventory: Query<(&mut Inventory, &Color)>, 
    mouse_button_input: Res<ButtonInput<MouseButton>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        if let Some(pos) = window.single().cursor_position() {
            let w = window.single().width() / 2.;
            let h = window.single().height() / 2.;

            if pos.x - w >= -537.5 && pos.x - w <= -462.5 {
                let mut value = selected.single().face;
                for i in 0..4 {
                    let level = -240. + (i * 160) as f32; 
                    if pos.y - h >= level - 37.5 && pos.y - h <= level + 37.5 {
                        let face = Face::from_num(3 - i);
                        let inv = inventory.iter().find(|e| *e.1 == board.single().color).unwrap().0;
                        if inv.can_place_face(face) {
                            value = Some(Face::from_num(3 - i));
                        }
                        break;
                    }
                }
                selected.single_mut().face = value;
                return;
            }

            if selected.single().face.is_none() {
                return;
            }

            if pos.x - w >= -325. && pos.x - w <= 325.
                && pos.y - h >= -325. && pos.y - h <= 325. 
            {
                let x = (pos.x - w + 325.) as u32 / (650 / SIZE_X);
                let y = 12 - ((pos.y - h + 325.) as u32 / (650 / SIZE_Y));
                if let Some(face) = selected.single().face {
                    let mut b = board.single_mut();
                    let red_inv = inventory.iter().find(|e| *e.1 == Color::Red).unwrap().0.clone();
                    let black_inv = inventory.iter().find(|e| *e.1 == Color::Black).unwrap().0.clone();
                    let mut inv = inventory.iter_mut().find(|e| *e.1 == b.color).unwrap().0;
                    if inv.can_place_face(face) 
                        && b.can_place_tile(x, y, face).or(b.can_place_free(x, y, face)).is_ok() 
                    {
                        inv.place_face(face);
                        selected.single_mut().face = None;
                        b.place_tile(x, y, face).ok().unwrap();
                        game.single_mut().add_move(x, y, face, b.clone(), red_inv.clone(), black_inv.clone());
                    }
                }
            }
        }
    }
}