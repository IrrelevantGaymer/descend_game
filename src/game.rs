use bevy::ecs::component::Component;

use crate::{board::{Board, Inventory}, tile::Face};

#[derive(Component)]
pub struct Game {
    moves: Vec<(Option<Move>, Board, (Inventory, Inventory))>,
    pos: usize
}

impl Game {
    pub fn new(x: u32, y: u32) -> Game {
        Game {
            moves: vec![(None, Board::new(x, y), (Inventory::new(3), Inventory::new(3)))],
            pos: 0
        }
    }
    pub fn first_move(&mut self) -> &Board {
        self.pos = 0;
        return &self.moves[self.pos].1;
    }
    pub fn prev_move(&mut self) -> &Board {
        if self.pos > 0 {
            self.pos -= 1;
        }
        return &self.moves[self.pos].1;
    }
    pub fn next_move(&mut self) -> &Board {
        if self.pos + 1 < self.moves.len() {
            self.pos += 1;
        }
        return &self.moves[self.pos].1
    }
    pub fn last_move(&mut self) -> &Board {
        self.pos = self.moves.len() - 1;
        return &self.moves[self.pos].1;
    }
    pub fn add_move(
        &mut self, x: u32, y: u32, face: Face, board: Board, 
        red_inv: Inventory, black_inv: Inventory
    ) {
        self.moves.truncate(self.pos + 1);
        self.moves.push((Some(Move {x, y, face}), board, (red_inv, black_inv)));
        self.pos += 1;
    }

    pub fn get_move(&self) -> &(Option<Move>, Board, (Inventory, Inventory)) {
        return &self.moves[self.pos];
    }

    pub fn get_mut_move(&mut self) -> &mut (Option<Move>, Board, (Inventory, Inventory)) {
        return &mut self.moves[self.pos];
    }

    pub fn get_pos(&self) -> usize {
        return self.pos;
    }
}

pub struct Move {
    x: u32,
    y: u32,
    face: Face
}