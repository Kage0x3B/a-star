use clap::{Arg, arg_enum, Parser, PossibleValue, ValueEnum};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
/// Rust implementation of the A*-algorithm
pub struct Args {
    /// Path to the input image, containing a grid graph with traversal costs and start/end vertices
    #[clap(parse(from_os_str))]
    pub input_path: std::path::PathBuf,

    /// Changes the weight of the path cost
    #[clap(short, long, value_parser, default_value_t = 1.)]
    pub cost_weight: f32,

    /// Changes the weight of the heuristics function
    #[clap(short, long, value_parser, default_value_t = 1.)]
    pub heuristics_weight: f32,

    #[clap(value_enum, short = 'f', long)]
    pub cost_function_enum: CostFunction,
}

pub struct PathfindingOptions {
    pub cost_weight: f32,
    pub heuristics_weight: f32,
}

use image::{GenericImageView, Rgba};

use crate::grid_graph::{GraphVertex, GridGraph, VisitedGraphVertex};
use crate::pathfinding::execute_a_star;

mod pathfinding;
mod grid_graph;

const EMPTY_VERTEX_COLOR: Rgba<u8> = Rgba([255, 255, 255, 255]);

const START_VERTEX_COLOR: Rgba<u8> = Rgba([0, 255, 0, 255]);
const GOAL_VERTEX_COLOR: Rgba<u8> = Rgba([255, 0, 0, 255]);

const PATH_VERTEX_COLOR: Rgba<u8> = Rgba([255, 128, 0, 255]);
const VISITED_VERTEX_COLOR: Rgba<u8> = Rgba([64, 64, 64, 255]);


fn zero_cost_function(start_vertex: &GraphVertex, goal_vertex: &GraphVertex, last_visited_vertex: &VisitedGraphVertex, current_vertex: &GraphVertex, current_vertex_cost: u8, options: &PathfindingOptions) -> f32 {
    (last_visited_vertex.cost + current_vertex_cost as f32) * options.cost_weight
}

fn euclidean_distance_cost_function(start_vertex: &GraphVertex, goal_vertex: &GraphVertex, last_visited_vertex: &VisitedGraphVertex, current_vertex: &GraphVertex, current_vertex_cost: u8, options: &PathfindingOptions) -> f32 {
    let g = last_visited_vertex.cost + current_vertex_cost as f32;
    let h = ((goal_vertex.x as f32 - current_vertex.x as f32).abs().powf(2.) + (goal_vertex.y as f32 - current_vertex.y as f32).abs().powf(2.)).sqrt();

    g * options.cost_weight + h * options.heuristics_weight
}

fn manhattan_distance_cost_function(start_vertex: &GraphVertex, goal_vertex: &GraphVertex, last_visited_vertex: &VisitedGraphVertex, current_vertex: &GraphVertex, current_vertex_cost: u8, options: &PathfindingOptions) -> f32 {
    let g = last_visited_vertex.cost + current_vertex_cost as f32;
    let h = (current_vertex.x as f32 - goal_vertex.x as f32).abs() + (current_vertex.y as f32 - goal_vertex.y as f32).abs();

    g * options.cost_weight + h * options.heuristics_weight
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum CostFunction {
    ZeroCost,
    EuclideanDistance,
    ManhattanDistance,
}

impl CostFunction {
    fn get_cost_function(&self) -> &dyn Fn(&GraphVertex, &GraphVertex, &VisitedGraphVertex, &GraphVertex, u8, &PathfindingOptions) -> f32 {
        match self {
            CostFunction::ZeroCost => &zero_cost_function,
            CostFunction::EuclideanDistance => &euclidean_distance_cost_function,
            CostFunction::ManhattanDistance => &manhattan_distance_cost_function
        }
    }
}

fn main() {
    let args: Args = Args::parse();
    let input_image = image::open(args.input_path).expect("File not found!");

    let (grid_width, grid_height) = input_image.dimensions();
    let mut tiles_grid_raw = vec![0_u8; (grid_width * grid_height) as _];
    let mut tiles_grid_base_mut: Vec<_> = tiles_grid_raw.as_mut_slice().chunks_mut(grid_width as _).collect();
    let tile_grid = tiles_grid_base_mut.as_mut_slice();

    let mut start_vertex = None;
    let mut goal_vertex = None;

    for (x, y, pixel) in input_image.pixels() {
        if pixel == START_VERTEX_COLOR {
            start_vertex = Some(GraphVertex::new(x, y));
            tile_grid[x as usize][y as usize] = 1;
            println!("Found start at {}, {}", x, y);
        } else if pixel == GOAL_VERTEX_COLOR {
            goal_vertex = Some(GraphVertex::new(x, y));
            tile_grid[x as usize][y as usize] = 1;
            println!("Found goal at {}, {}", x, y);
        } else {
            tile_grid[x as usize][y as usize] = pixel[0] as u8;
        }
    }

    let cost_function = args.cost_function_enum.get_cost_function();

    let start_vertex = start_vertex.expect("No start vertex with color rgba(0, 255, 0, 255) found!");
    let goal_vertex = goal_vertex.expect("No goal vertex with color rgba(255, 0, 0, 255) found!");

    let tiles_grid_base: Vec<_> = tiles_grid_raw.as_slice().chunks(grid_width as _).collect();
    let grid_map = GridGraph::new(grid_width, grid_height, tiles_grid_base.as_slice());

    let path_result = execute_a_star(&grid_map, start_vertex, goal_vertex, cost_function, &PathfindingOptions {
        cost_weight: args.cost_weight,
        heuristics_weight: args.heuristics_weight,
    }).expect("Couldn't find valid path");

    println!("Found path: {:?}, visited {} vertices", path_result.path, path_result.visited_vertices.len());

    let mut result_img = image::ImageBuffer::new(grid_width, grid_height);

    for pixel in result_img.pixels_mut() {
        *pixel = EMPTY_VERTEX_COLOR;
    }

    let mut max_visited_cost = 0_f32;
    for path_vertex in &path_result.visited_vertices {
        if path_vertex.cost > max_visited_cost {
            max_visited_cost = path_vertex.cost;
        }
    }

    for visited_vertex in &path_result.visited_vertices {
        let cost_color = ((visited_vertex.cost / max_visited_cost) * 200.).round() as u8;
        result_img[(visited_vertex.x, visited_vertex.y)] = Rgba([cost_color, cost_color, cost_color, 255]);
    }

    let mut max_cost = 0_f32;
    for path_vertex in &path_result.path {
        if path_vertex.cost > max_cost {
            max_cost = path_vertex.cost;
        }
    }

    for path_vertex in &path_result.path {
        result_img[(path_vertex.x, path_vertex.y)] = Rgba([((path_vertex.cost / max_cost) * 255.).round() as u8, 128, 0, 255]);
    }

    result_img[(start_vertex.x, start_vertex.y)] = START_VERTEX_COLOR;
    result_img[(goal_vertex.x, goal_vertex.y)] = GOAL_VERTEX_COLOR;

    result_img.save("path_result.png").expect("Couldn't save resulting path image");
}