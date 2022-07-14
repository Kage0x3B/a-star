use std::collections::{BinaryHeap, HashMap};
use crate::{Args, PathfindingOptions};

use crate::grid_graph::{ALL_DIRECTIONS, GraphVertex, GridGraph, VisitedGraphVertex};

#[derive(Debug)]
pub struct PathResult {
    pub path: Vec<VisitedGraphVertex>,
    pub visited_vertices: Vec<VisitedGraphVertex>,
}

// cost_func(start_vertex, goal_vertex, last_visited_vertex, current_vertex_cost)
pub fn execute_a_star(graph: &GridGraph, start_vertex: GraphVertex, goal_vertex: GraphVertex, cost_func: &dyn Fn(&GraphVertex, &GraphVertex, &VisitedGraphVertex, &GraphVertex, u8, &PathfindingOptions) -> f32, options: &PathfindingOptions) -> Option<PathResult> {
    let mut open_list: BinaryHeap<VisitedGraphVertex> = BinaryHeap::new();
    let mut closed_list: HashMap<GraphVertex, f32> = HashMap::new();
    let mut parent_map: HashMap<GraphVertex, VisitedGraphVertex> = HashMap::new();

    let mut visited_vertices = Vec::new();

    open_list.push(start_vertex.into_visited(0.));
    closed_list.insert(start_vertex, 0.);

    let mut visit_amount = 0;
    let mut visit_neighbour_amount = 0;

    while let Some(current_vertex) = open_list.pop() {
        visit_amount += 1;
        visited_vertices.push(current_vertex);

        if current_vertex == goal_vertex {
            println!("Found goal {:?}", current_vertex);

            break;
        }

        let mut curr_visited_neighbours = 0;

        for direction in ALL_DIRECTIONS {
            if let Some(neighbour_vertex) = graph.get_neighbouring_vertex(&current_vertex, direction) {
                if let std::collections::hash_map::Entry::Vacant(new_entry) = closed_list.entry(neighbour_vertex) {
                    visit_neighbour_amount += 1;
                    curr_visited_neighbours += 1;
                    let vertex_cost = graph.get_cost(&neighbour_vertex);
                    let calculated_cost: f32 = cost_func(&start_vertex, &goal_vertex, &current_vertex, &neighbour_vertex, vertex_cost, options);

                    let visited_neighbour_vertex = neighbour_vertex.into_visited(calculated_cost);
                    //println!("{}, {} to {}, {} cost: {} + {} => {}", current_vertex.x, current_vertex.y, neighbour_vertex.x, neighbour_vertex.y, current_vertex.cost, vertex_cost, calculated_cost);

                    if !parent_map.contains_key(&neighbour_vertex) || parent_map.get(&neighbour_vertex).unwrap().cost > current_vertex.cost {
                        parent_map.insert(neighbour_vertex, current_vertex);
                    }
                    open_list.push(visited_neighbour_vertex);
                    new_entry.insert(current_vertex.cost);

                    println!("{}, {} visited {}, {}", current_vertex.x, current_vertex.y, neighbour_vertex.x, neighbour_vertex.y);
                }
            }
        }

        println!("{}, {} visited {} neighbours", current_vertex.x, current_vertex.y, curr_visited_neighbours);
    }

    println!("Visited {} vertices and {} neighbours", visit_amount, visit_neighbour_amount);

    let mut path_vertices: Vec<VisitedGraphVertex> = Vec::new();

    println!("{}", parent_map.contains_key(&goal_vertex));
    let mut vertex = *parent_map.get(&goal_vertex)?;

    path_vertices.push(vertex);

    while parent_map.contains_key(&vertex.into()) {
        vertex = *parent_map.get(&vertex.into()).unwrap();
        path_vertices.push(vertex);
    }

    path_vertices.reverse();

    Some(PathResult {
        visited_vertices,
        path: path_vertices,
    })
}