use bevy::ecs::component::Component;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tile {
    None,
    Blocked,
    Card(Face, Color)
}

impl Tile {
    pub fn is_ascending_from_face(&self, face: Face) -> bool {
        if let Tile::Card(s_face, _) = self {
            return* s_face > face;
        }
        return false;
    }

    pub fn is_card(&self) -> bool {
        if let Tile::Card(..) = self {
            return true;
        } else {
            return false;
        }
    }
}

#[derive(Component, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Face {
    Wild = 4, Ace = 3, King = 2, Queen = 1, Jack = 0
}

impl Face {
    #[allow(dead_code)]
    pub fn is_descending_from_tile(&self, tile: Tile) -> bool {
        if let Tile::Card(face, _) = tile {
            return *self < face;
        }
        return false;
    }

    pub fn from_num(num: u32) -> Face {
        match num {
            0 => Face::Jack,
            1 => Face::Queen,
            2 => Face::King,
            3 => Face::Ace,
            _ => panic!()
        }
    }
}

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub enum Color {
    Both, Red, Black
}

impl Color {
    pub fn next(&mut self) {
        *self = match self {
            Color::Red => Color::Black,
            Color::Black => Color::Red,
            _ => *self
        };
    }
}

#[derive(Component)]
pub struct Selected {
    pub face: Option<Face>
}