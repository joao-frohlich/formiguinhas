use crate::cell::Cell;
use crate::params::Params;
use bevy::prelude::*;
use rand::distributions::{Distribution, Uniform};

pub struct Board {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<Entity>>,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        let cells = vec![vec![Entity::from_raw(0); width]; height];
        Self {
            width,
            height,
            cells,
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new(50, 50)
    }
}

pub fn setup_board(mut commands: Commands, windows: Res<Windows>, mut board: ResMut<Board>) {
    let window = windows.primary();
    let border_width = 2.0;
    let cell_width =
        (window.width() - border_width * (board.width - 1) as f32) / (board.width as f32);
    let cell_height =
        (window.height() - border_width * (board.height - 1) as f32) / (board.height as f32);
    for xx in 0..board.width {
        for yy in 0..board.height {
            let x = xx as f32;
            let y = yy as f32;
            let cx = -window.width() / 2. + cell_width * x + border_width * x + cell_width / 2.;
            let cy = -window.height() / 2. + cell_height * y + border_width * y + cell_height / 2.;
            let entity = commands
                .spawn_bundle(SpriteBundle {
                    transform: Transform::from_xyz(cx, cy, 1.0),
                    sprite: Sprite {
                        color: Color::rgb(1., 1., 1.),
                        custom_size: Some(Vec2::new(cell_width, cell_height)),
                        ..default()
                    },
                    ..default()
                })
                .insert(Cell::default())
                .id();
            board.cells[xx][yy] = entity;
        }
    }
}

pub fn setup_dead_ants(board: Res<Board>, mut query: Query<&mut Cell>, params: Res<Params>) {
    let mut cont = 0;
    let between_width = Uniform::from(0..board.width);
    let between_height = Uniform::from(0..board.height);
    let mut rng = rand::thread_rng();
    //println!("{}", query.iter().len());
    while cont < params.dead_ants {
        let x = between_width.sample(&mut rng);
        let y = between_height.sample(&mut rng);
        let mut cell = query.get_mut(board.cells[x][y]).unwrap();
        if !cell.has_dead {
            cell.has_dead = true;
            cont += 1;
        }
    }
}

pub fn color_cells(mut query_cell: Query<(&Cell, &mut Sprite), Changed<Cell>>) {
    for (cell, mut sprite) in query_cell.iter_mut() {
        let green = if cell.has_dead { 0.25 } else { 1. };
        let blue = if cell.has_dead { 0.25 } else { 1. };
        sprite.color = Color::rgb(1., green, blue);
    }
}
