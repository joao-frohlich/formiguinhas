use crate::heterogeneous_board::HeterogeneousBoard;
use crate::heterogeneous_cell::HeterogeneousCell;
use crate::data_item::DataItem;
use crate::params::Params;
use bevy::prelude::*;
use rand::distributions::WeightedIndex;
use rand::distributions::{Distribution, Uniform};
use rand::Rng;
// use std::{thread, time};

#[derive(Default, Component)]
pub struct Agent {
    x: usize,
    y: usize,
    radius: usize,
    item: Option<DataItem>,
    iter: usize,
    active: bool,
}

pub fn setup_agents(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    board: Res<HeterogeneousBoard>,
    windows: Res<Windows>,
    mut query: Query<&mut HeterogeneousCell>,
    params: Res<Params>,
) {
    asset_server.watch_for_changes().unwrap();

    let window = windows.primary();
    let border_width = 2.0;
    let cell_width =
        (window.width() - border_width * (board.width - 1) as f32) / (board.width as f32);
    let cell_height =
        (window.height() - border_width * (board.height - 1) as f32) / (board.height as f32);

    let mut cont = 0;
    let between_width = Uniform::from(0..board.width);
    let between_height = Uniform::from(0..board.height);
    let mut rng = rand::thread_rng();

    while cont < params.agents {
        let x = between_width.sample(&mut rng);
        let y = between_height.sample(&mut rng);
        let mut cell = query.get_mut(board.cells[x][y]).unwrap();
        if !cell.has_alive {
            cell.has_alive = true;
            let xx = x as f32;
            let yy = y as f32;
            let cx = -window.width() / 2. + cell_width * xx + border_width * xx + cell_width / 2.;
            let cy =
                -window.height() / 2. + cell_height * yy + border_width * yy + cell_height / 2.;
            commands
                .spawn_bundle(SpriteBundle {
                    texture: asset_server.load("empty_ant.png"),
                    transform: Transform::from_xyz(cx, cy, 2.0),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(cell_width, cell_height)),
                        ..default()
                    },
                    ..default()
                })
                .insert(Agent {
                    x: x,
                    y: y,
                    radius: params.radius,
                    active: true,
                    ..default()
                });
            cont += 1;
        }
    }
}

pub fn draw_agents(asset_server: Res<AssetServer>, mut query: Query<(&Agent, &mut Handle<Image>)>) {
    for (agent, mut image_handle) in query.iter_mut() {
        match agent.item {
            Some(_) => {*image_handle = asset_server.load("carry_ant.png");},
            None => { *image_handle = asset_server.load("empty_ant.png");}
        }
    }
}

fn euclidean_distance_2d(item1: DataItem, item2: DataItem) -> f32 {
    f32::powf(f32::powf(item1.size - item2.size, 2.) + f32::powf(item1.weight - item2.weight,2.),1./2.)
}

fn check_radius(
    board: &Res<HeterogeneousBoard>,
    ax: i32,
    ay: i32,
    r: i32,
    carried_item: DataItem,
    query_cell: &Query<&mut HeterogeneousCell>,
) -> f32 {
    let width = board.width as i32;
    let height = board.height as i32;
    let mut tot = 0;
    let mut occ = 0;
    let mut sum_d: f32 = 0.;
    for x in ax - r..=ax + r {
        for y in ay - r..=ay + r {
            if x >= 0 && x < width && (x != ax || y != ay) && y >= 0 && y < height
            {
                tot += 1;
                let cell = query_cell.get(board.cells[x as usize][y as usize]).unwrap();
                match cell.item {
                    Some(looked_item) => {
                        occ += 1;
                        sum_d += euclidean_distance_2d(carried_item, looked_item);
                    },
                    None => {}
                }
            }
        }
    }
    if occ == 0 { return 0.; }
    else {
        // println!("{} {}", tot, sum_d);
        tot as f32/sum_d
    }
}

pub fn move_agent(
    windows: Res<Windows>,
    board: Res<HeterogeneousBoard>,
    mut query: Query<(&mut Agent, &mut Transform)>,
    mut query_cell: Query<&mut HeterogeneousCell>,
    mut params: ResMut<Params>,
) {
    // let time = time::Duration::from_secs_f32(0.1);
    // thread::sleep(time);

    let moves: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    let mut cur_iter = 0;
    let window = windows.primary();
    let border_width = 2.0;
    let cell_width =
        (window.width() - border_width * (board.width - 1) as f32) / (board.width as f32);
    let cell_height =
        (window.height() - border_width * (board.height - 1) as f32) / (board.height as f32);
    let max_iter = params.max_iter;

    for _ in 0..params.iter_per_render {
        for (mut agent, _) in query.iter_mut() {
            if !agent.active {
                continue;
            }
            agent.iter += 1;
            cur_iter = agent.iter;
            let mut weights: [i32; 4] = [0, 0, 0, 0];
            let mut has_option = false;
            let cell = query_cell.get_mut(board.cells[agent.x][agent.y]).unwrap();
            match agent.item {
                //Largar
                Some(item) => {
                    match cell.item {
                        Some(_) => {},
                        None => {
                            let score = check_radius(
                                &board,
                                agent.x as i32,
                                agent.y as i32,
                                agent.radius as i32,
                                item,
                                &query_cell,
                            );
                            let mut cell = query_cell.get_mut(board.cells[agent.x][agent.y]).unwrap();
                            // let exp = 2.;
                            let exp = std::f32::consts::E;
                            let score = score * (1. - params.min_prob * 2.) + params.min_prob;
                            let threshold = f32::powf(score, exp);
                            // let threshold = f32::min(2.*score, 1.);
                            // let let_threshold = f32::powf(score, exp);
                            // let get_threshold = 1. - (f32::powf(score, 1. / exp));
                            let dist = Uniform::<f32>::new_inclusive(0., 1.);
                            let choice: f32 = rand::thread_rng().sample(dist);
                            if choice <= threshold {
                                cell.item = agent.item;
                                agent.item = None;
                            }
                        }
                    }
                },
                //Pegar
                None => {
                    if agent.iter > max_iter {
                        agent.active = false;
                    } else {
                        match cell.item {
                            Some(item) => {
                                let score = check_radius(
                                    &board,
                                    agent.x as i32,
                                    agent.y as i32,
                                    agent.radius as i32,
                                    item,
                                    &query_cell,
                                );
                                let mut cell = query_cell.get_mut(board.cells[agent.x][agent.y]).unwrap();
                                // let exp = 2.;
                                let exp = std::f32::consts::E;
                                let score = score * (1. - params.min_prob * 2.) + params.min_prob;
                                // println!("{}", score);
                                // let threshold = 1. - f32::powf(score, 1./ exp);
                                let threshold = f32::powf(params.k1/(params.k1+score), exp);
                                // let let_threshold = f32::powf(score, exp);
                                // let get_threshold = 1. - (f32::powf(score, 1. / exp));
                                let dist = Uniform::<f32>::new_inclusive(0., 1.);
                                let choice: f32 = rand::thread_rng().sample(dist);
                                if choice <= threshold {
                                    agent.item = cell.item;
                                    cell.item = None;
                                }
                            },
                            None => {}
                        }
                    }
                }
            }
            // if agent.state && !cell.has_dead {
            //     if choice <= let_threshold {
            //         agent.state = false;
            //         cell.has_dead = true;
            //     }
            // } else if !agent.state {
            //     if agent.iter > max_iter {
            //         agent.active = false;
            //     } else {
            //         if cell.has_dead && choice <= get_threshold {
            //             agent.state = true;
            //             cell.has_dead = false;
            //         }
            //     }
            // }
            if agent.x < board.height - 1 {
                let x = agent.x + 1;
                let y = agent.y;
                let cell = query_cell.get(board.cells[x][y]).unwrap();
                if !cell.has_alive {
                    weights[0] = 1;
                    has_option = true;
                }
            }
            if agent.y < board.width - 1 {
                let x = agent.x;
                let y = agent.y + 1;
                let cell = query_cell.get(board.cells[x][y]).unwrap();
                if !cell.has_alive {
                    weights[1] = 1;
                    has_option = true;
                }
            }
            if agent.x > 0 {
                let x = agent.x - 1;
                let y = agent.y;
                let cell = query_cell.get(board.cells[x][y]).unwrap();
                if !cell.has_alive {
                    weights[2] = 1;
                    has_option = true;
                }
            }
            if agent.y > 0 {
                let x = agent.x;
                let y = agent.y - 1;
                let cell = query_cell.get(board.cells[x][y]).unwrap();
                if !cell.has_alive {
                    weights[3] = 1;
                    has_option = true;
                }
            }
            if has_option {
                let dist = WeightedIndex::new(&weights).unwrap();
                let mut rng = rand::thread_rng();
                let movement = moves[dist.sample(&mut rng)];
                let mut cell = query_cell.get_mut(board.cells[agent.x][agent.y]).unwrap();
                cell.has_alive = false;
                let new_x: usize = (agent.x as i32 + movement.0) as usize;
                let new_y: usize = (agent.y as i32 + movement.1) as usize;
                let mut cell = query_cell.get_mut(board.cells[new_x][new_y]).unwrap();
                cell.has_alive = true;
                agent.x = new_x;
                agent.y = new_y;
            }
        }
    }
    for (agent, mut transform) in query.iter_mut() {
        let x = agent.x as f32;
        let y = agent.y as f32;
        let cx = -window.width() / 2. + cell_width * x + border_width * x + cell_width / 2.;
        let cy = -window.height() / 2. + cell_height * y + border_width * y + cell_height / 2.;
        let translation = &mut transform.translation;
        translation.x = cx;
        translation.y = cy;
    }
    if cur_iter == 0 {
        if !params.is_done {
            println!("Cabou");
            params.is_done = true;
        }
        return;
    }
    // if cur_iter % 1000 == 0 {
    //     println!("{}", cur_iter);
    // };
}

pub fn set_visibility(mut query: Query<(&mut Visibility, &Agent)>) {
    for (mut visibility, ant) in query.iter_mut() {
        visibility.is_visible = ant.active;
    }
}
