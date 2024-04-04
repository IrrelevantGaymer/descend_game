use bevy::{asset::{Assets, Handle}, ecs::{component::Component, query::Without, system::{Query, ResMut}}, math::primitives::{Circle, Rectangle}, render::mesh::Mesh, sprite::{ColorMaterial, Mesh2dHandle}, text::Text};
use bevy::render::color::Color as BevyColor;

use crate::{tile::{Color, Face, Selected, Tile}, SIZE_X, UI};

#[derive(Component, Clone)]
pub struct Board {
    size: (u32, u32),
    board: Vec<Tile>,
    pub color: Color
}

#[derive(Component, Clone)]
pub struct Inventory {
    aces: u32,
    kings: u32,
    queens: u32,
    jacks: u32
}

#[derive(Component)]
pub struct Index(pub usize);

#[derive(Clone, Copy)]
pub enum Dir {
    Left, Up, Right, Down
}

impl Board {
    pub fn new(x: u32, y: u32) -> Board {
        let mut board = vec![Tile::None; (x * y) as usize];
        board[(x * y / 2) as usize] = Tile::Card(Face::Wild, Color::Both);
        Board {
            size: (x, y),
            board: board,
            color: Color::Red
        }
    }
    
    pub fn place_tile(&mut self, x: u32, y: u32, face: Face) -> Result<(), TileError> {
        self.can_place_tile(x, y, face).or(self.can_place_free(x, y, face))?;
        *self.get_mut_tile(x, y)? = Tile::Card(face, self.color);
        
        self.clear_blocked_tiles();
        for dir in [Dir::Left, Dir::Up, Dir::Right, Dir::Down] {
            if let Some((x, y)) = self.get_blocked_tile(x, y, dir) {
                self.set_tile(x, y, Tile::Blocked);
            }
        }

        self.color.next();
        return Ok(());
    }

    pub fn get_blocked_tile(&self, x: u32, y: u32, dir: Dir) -> Option<(u32, u32)> {
        let mut pos = (x as i32, y as i32);
        let offset = dir.offset();
        let mut will_block = false;

        loop {
            pos.0 += offset.0;
            pos.1 += offset.1;
            if pos.0 < 0 && pos.1 < 0 {
                return None;
            }
            match self.get_tile(pos.0 as u32, pos.1 as u32).ok()? {
                Tile::None => if will_block {
                    return Some((pos.0 as u32, pos.1 as u32));
                } else {
                    return None;
                },
                Tile::Blocked => return None,
                Tile::Card(face, _) => if face != Face::Wild {
                    will_block = true;
                }
            }
        }
    }

    pub fn clear_blocked_tiles(&mut self) {
        for tile in self.board.iter_mut() {
            if *tile == Tile::Blocked {
                *tile = Tile::None;
            }
        }
    }

    pub fn can_place_tile(&self, x: u32, y: u32, face: Face) -> Result<(), TileError> {
        if self.get_tile(x, y)? != Tile::None {
            return Err(TileError::BlockedTile);
        }

        let surrounding_tiles: Vec<Option<Tile>> = [
            if x > 0 {self.get_tile(x - 1, y)} else {Err(TileError::XOutOfBounds)}, 
            if y > 0 {self.get_tile(x, y - 1)} else {Err(TileError::YOutOfBounds)},
            self.get_tile(x + 1, y),
            self.get_tile(x, y + 1)
        ].into_iter().map(|e| e.ok()).collect();

        surrounding_tiles.iter()
            .find(|f| f.is_some_and(|s| s.is_ascending_from_face(face)))
            .ok_or(TileError::NonDescending)?;

        return Ok(());
    }

    pub fn can_place_free(&self, x: u32, y: u32, face: Face) -> Result<(), TileError> {
        if self.any_legal_move(face) {
            return Err(TileError::NotFree);
        } else if self.get_tile(x, y)? != Tile::None {
            return Err(TileError::BlockedTile);
        }

        let surrounding_tiles: Vec<Option<Tile>> = [
            if x > 0 {self.get_tile(x - 1, y)} else {Err(TileError::XOutOfBounds)}, 
            if y > 0 {self.get_tile(x, y - 1)} else {Err(TileError::YOutOfBounds)},
            self.get_tile(x + 1, y),
            self.get_tile(x, y + 1)
        ].into_iter().map(|e| e.ok()).collect();

        surrounding_tiles.iter()
            .find(|f| f.is_some_and(|s| s.is_card()))
            .ok_or(TileError::NonDescending)?;

        return Ok(());

    }

    pub fn any_legal_move(&self, face: Face) -> bool {
        for i in 0..self.board.len() as u32 {
            let x = i % self.size.0;
            let y = i / self.size.0;
            if self.can_place_tile(x, y, face).is_ok() {
                return true;
            }
        }
        return false;
    }

    pub fn get_tile(&self, x: u32, y: u32) -> Result<Tile, TileError> {
        if x >= self.size.0 {
            return Err(TileError::XOutOfBounds);
        } else if y >= self.size.1 {
            return Err(TileError::YOutOfBounds);
        }
        
        let index = self.size.0 * y + x;
        return Ok(self.board[index as usize]);
    }

    pub fn get_mut_tile(&mut self, x: u32, y: u32) -> Result<&mut Tile, TileError> {
        if x >= self.size.0 {
            return Err(TileError::XOutOfBounds);
        } else if y >= self.size.1 {
            return Err(TileError::YOutOfBounds);
        }
        
        let index = self.size.0 * y + x;
        return Ok(&mut self.board[index as usize]);
    }

    pub fn set_tile(&mut self, x: u32, y: u32, tile: Tile) {
        if x >= self.size.0 {
            panic!();
        } else if y >= self.size.1 {
            panic!();
        }
        
        let index = self.size.0 * y + x;
        self.board[index as usize] = tile;
    }
}

impl Inventory {
    pub fn new(num_cards: u32) -> Inventory {
        Inventory {
            aces: num_cards,
            kings: num_cards,
            queens: num_cards,
            jacks: num_cards
        }
    }

    pub fn can_place_face(&self, face: Face) -> bool {
        match face {
            Face::Ace => self.can_place_ace(),
            Face::King => self.can_place_king(),
            Face::Queen => self.can_place_queen(),
            Face::Jack => self.can_place_jack(),
            _ => false
        }
    }

    pub fn place_face(&mut self, face: Face) {
        match face {
            Face::Ace => self.place_ace(),
            Face::King => self.place_king(),
            Face::Queen => self.place_queen(),
            Face::Jack => self.place_jack(),
            _ => ()
        }
    }
    
    pub fn can_place_ace(&self) -> bool {
        self.aces > 0
    }

    pub fn place_ace(&mut self) {
        self.aces -= 1;
    }

    pub fn can_place_king(&self) -> bool {
        self.kings > 0
    }

    pub fn place_king(&mut self) {
        self.kings -= 1;
    }

    pub fn can_place_queen(&self) -> bool {
        self.queens > 0
    }

    pub fn place_queen(&mut self) {
        self.queens -= 1;
    }

    pub fn can_place_jack(&self) -> bool {
        self.jacks > 0
    }

    pub fn place_jack(&mut self) {
        self.jacks -= 1;
    }

    pub fn get_num_face(&self, face: Face) -> u32 {
        match face {
            Face::Ace => self.aces,
            Face::King => self.kings,
            Face::Queen => self.queens,
            Face::Jack => self.jacks,
            _ => panic!()
        }
    }
}

impl Dir {
    pub fn offset(&self) -> (i32, i32) {
        match self {
            Dir::Left => (-1, 0),
            Dir::Up => (0, -1),
            Dir::Right => (1, 0),
            Dir::Down => (0, 1)
        }
    }
}

#[derive(Debug)]
pub enum TileError {
    XOutOfBounds,
    YOutOfBounds,
    BlockedTile,
    NonDescending,
    NotFree
}

pub fn update_screen(
    selected: Query<&Selected>,
    board: Query<&Board>,
    inventory: Query<(&Inventory, &Color)>, 
    mut board_meshes: Query<(&mut Mesh2dHandle, &mut Handle<ColorMaterial>, &Index)>,
    mut board_text: Query<(&mut Text, &Index), (Without<UI>, Without<Color>, Without<Face>)>,
    mut face_meshes: Query<(&mut Mesh2dHandle, &mut Handle<ColorMaterial>), Without<Index>>,
    mut inventory_text: Query<(&mut Text, &Color, &Face), (Without<UI>, Without<Index>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let b = board.single();
    let s = selected.single().face;
    for ((mesh, color, Index(i)), (mut text, _)) in board_meshes.iter_mut().zip(board_text.iter_mut()) {
        let x = *i as u32 % SIZE_X;
        let y = *i as u32 / SIZE_X;

        meshes.insert(&mesh.0, match b.get_tile(x, y).ok().unwrap() {
            Tile::None => Into::<Mesh>::into(Rectangle::new(45.0, 45.0)),
            Tile::Blocked => Circle::new(12.).into(),
            Tile::Card(..) => Rectangle::new(45.0, 45.0).into()
        });
        materials.insert(color.id(), match b.get_tile(x, y).ok().unwrap() {
            Tile::None => if let Some(face) = s {
                if b.can_place_tile(x, y, face).or(b.can_place_free(x, y, face)).is_ok() {
                    BevyColor::rgba(1., 1., 1., 1.)
                } else {
                    BevyColor::rgba(0., 0., 0., 1.)
                }
            } else {
                BevyColor::rgba(0., 0., 0., 1.)
            },
            Tile::Blocked => BevyColor::rgba(1., 1., 0., 1.),
            Tile::Card(_, Color::Black) => BevyColor::rgba(0.5, 0.5, 0.5, 1.),
            Tile::Card(_, Color::Red) => BevyColor::rgba(1., 0., 0., 1.),
            Tile::Card(_, Color::Both) => BevyColor::rgba(1., 0., 1., 1.)
        }.into());
        text.sections[0].value = match b.get_tile(x, y).ok().unwrap() {
            Tile::Card(Face::Wild, ..) => "W",
            Tile::Card(Face::Ace, ..) => "A",
            Tile::Card(Face::King, ..) => "K",
            Tile::Card(Face::Queen, ..) => "Q",
            Tile::Card(Face::Jack, ..) => "J",
            _ => " "
        }.to_string();
    }

    let bc = b.color;
    let inv = inventory.iter().find(|e| *e.1 == bc).unwrap().0;
    let faces = [Face::Ace, Face::King, Face::Queen, Face::Jack];
    for ((mesh, color), face) in face_meshes.iter_mut().zip(faces.iter()) {
        if inv.can_place_face(*face) {
            if s.is_some_and(|e| e == *face) {
                meshes.insert(&mesh.0, Rectangle::new(90., 90.).into());
            } else {
                meshes.insert(&mesh.0, Rectangle::new(75., 75.).into());
            }
            materials.insert(color.id(), match bc {
                Color::Red => BevyColor::rgba(1., 0., 0., 1.),
                Color::Black => BevyColor::rgba(0.5, 0.5, 0.5, 1.),
                Color::Both => BevyColor::rgba(1., 0., 1., 1.)
            }.into());
            continue;
        } 
        meshes.insert(&mesh.0, Rectangle::new(75., 75.).into());
        materials.insert(color.id(), BevyColor::rgba(0.2, 0.2, 0.2, 1.).into());
    }

    for (mut text, color, face) in inventory_text.iter_mut() {
        let inv = inventory.iter().find(|e| *e.1 == *color).unwrap().0;
        text.sections[0].value = format!("{} {}", match face {
            Face::Ace => "A",
            Face::King => "K",
            Face::Queen => "Q",
            Face::Jack => "J",
            _ => panic!()
        }, match inv.get_num_face(*face) {
            0 => "     ",
            1 => "*    ",
            2 => "* *  ",
            3 => "* * *",
            _ => panic!()
        })
    }
}