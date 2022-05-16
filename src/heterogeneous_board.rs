use std::fs;
use crate::heterogeneous_cell::HeterogeneousCell;
use crate::params::Params;
use crate::data_item::DataItem;
use bevy::prelude::*;
use rand::distributions::{Distribution, Uniform};

pub struct HeterogeneousBoard {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<Entity>>,
}

impl HeterogeneousBoard {
    pub fn new(width: usize, height: usize) -> Self {
        let cells = vec![vec![Entity::from_raw(0); width]; height];
        Self {
            width,
            height,
            cells,
        }
    }
}

impl Default for HeterogeneousBoard {
    fn default() -> Self {
        Self::new(50, 50)
    }
}

fn read_item_data() -> Vec<(f32, f32, usize)> {
    let mut data: Vec<(f32, f32, usize)> = Vec::<(f32, f32, usize)>::default();
    let base_path = "bases/base4.txt".to_string();
    let contents = fs::read_to_string(base_path).expect("Something went wrong");
    for line in contents.split('\n') {
        let line = line.trim();
        if line.is_empty() || line.chars().nth(0) == Some('#') {
            continue;
        }
        let line = line.replace(",", ".");
        let values: Vec<&str> = line.split_whitespace().collect();
        let (weight, size, label) = (values[0].parse::<f32>().unwrap(), values[1].parse::<f32>().unwrap(), values[2].parse::<usize>().unwrap());
        data.push((weight, size, label));
    }
    data
}

pub fn setup_board(mut commands: Commands, windows: Res<Windows>, mut board: ResMut<HeterogeneousBoard>) {
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
                .insert(HeterogeneousCell::default())
                .id();
            board.cells[xx][yy] = entity;
        }
    }
}

pub fn setup_items(board: Res<HeterogeneousBoard>, mut query: Query<&mut HeterogeneousCell>) {
    let mut cont = 0;
    let items = read_item_data();

    let between_width = Uniform::from(0..board.width);
    let between_height = Uniform::from(0..board.height);
    let mut rng = rand::thread_rng();

    //println!("{}", query.iter().len());
    while cont < items.len() {
        let x = between_width.sample(&mut rng);
        let y = between_height.sample(&mut rng);
        let mut cell = query.get_mut(board.cells[x][y]).unwrap();
        match cell.item {
            Some(_) => {},
            None => {
                cell.item = Some(DataItem::new(items[cont]));
                cont+=1;
            }
        }
    }
}

pub fn color_cells(mut query_cell: Query<(&HeterogeneousCell, &mut Sprite), Changed<HeterogeneousCell>>, params: Res<Params>) {
    for (cell, mut sprite) in query_cell.iter_mut() {
        let colors: (f32, f32, f32) = match cell.item {
            Some(data_item) => {
                params.colors[data_item.label-1]
            },
            None => (1., 1., 1.),
        };
        let (red, green, blue) = colors;
        sprite.color = Color::rgb(red, green, blue);
    }
}
