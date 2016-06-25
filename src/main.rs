#[macro_use]
extern crate log;
extern crate log4rs;

// Represent the hexagonal game board as an array of vertices and an array of edges.
// There are three types of edges: vertical, right diagonal, and left diagonal,
// Right diagonal means the line going down from left to right.
//               
//                  North
//                   /\
//    Left diagonal /| \  Right diagonal
//                 / |  \
//     Northwest  /  |   \  Northeast
//               |\  |   /|
//               | \ |  / |  
//               |  \* /  |   Vertical edges
//               |  /| \  |
//               | / |  \ |
//                \  |   /  Southeast
//       Southwest \ |  /
//                  \| /
//                   \/
//                  South
//
// This will be used to change indices for vertices into indices for neighbouring edges.
// The edges inside a hexagon aren't edges in the game but having them makes it easier to index the
// edges.
//

// @cleanup use enums later for some fields.
#[derive(Default)]
struct Vertex {
    resource: i32,
    resource_number: i32,
    building_color: i32,
    building_type: i32,
    robber: i32,
    // Some vertices are in the middle of a hexagon.
    // You can have a resource and robbers on them, but not buildings.
    
}

#[derive(Default)]
struct Harbor {
    input_resource: i32,
    input_resource_count: i32,
    output_resource: i32,
    output_resouce_count: i32
}

#[derive(Default)]
struct Edge {
    road_color: i32,
    harbor: Harbor,
    // Some edges are inside a hexagon and aren't real edges in the game.
    is_valid: bool,
}

struct Board {
    width: usize,
    height: usize,
    vertices_size: usize,
    vertical_edges_size: usize,
    right_diagonal_edges_size: usize,
    left_diagonal_edges_size: usize,
    edges_size: usize,
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
}

fn get_row(vertex_index: usize, width: usize) -> usize {
    return vertex_index / width;
}

fn get_column(vertex_index: usize, width: usize) -> usize {
    return vertex_index % width;
}

fn get_north_vertex(vertex_index: usize, board: Board) -> usize {
    return vertex_index - board.width;
}

fn get_northeast_vertex(vertex_index: usize, board: Board) -> usize {
    return vertex_index - board.width + 1;
}

fn get_southeast_vertex(vertex_index: usize, board: Board) -> usize {
    return vertex_index + 1;
}

fn get_north_edge(vertex_index: usize, board: Board) -> usize {
    return (get_row(vertex_index, board.width) -1) * board.width +
        get_column(vertex_index, board.width);
}

fn get_northeast_edge(vertex_index: usize, board: Board) -> usize {
    let offset = (get_row(vertex_index, board.width) - 1) * (board.width - 1) +
        get_column(vertex_index, board.width);
    return board.vertical_edges_size + board.right_diagonal_edges_size + offset;
}

fn get_southeast_edge(vertex_index: usize, board: Board) -> usize {
    let offset = get_row(vertex_index, board.width) * (board.width - 1) +
        get_column(vertex_index, board.width);
    return board.vertical_edges_size  + offset;
}

fn get_south_edge(vertex_index: usize, board: Board) -> usize {
    // This can be simplified to `return vertex_index`
    return get_row(vertex_index, board.width) * board.width +
        get_column(vertex_index, board.width);
}

fn get_southwest_edge(vertex_index: usize, board: Board) -> usize {
    let offset = get_row(vertex_index, board.width) * (board.width - 1) +
        get_column(vertex_index, board.width) - 1;
    return board.vertical_edges_size + board.right_diagonal_edges_size + offset;
}

fn get_northwest_edge(vertex_index: usize, board: Board) -> usize {
    let offset = get_row(vertex_index, board.width) * (board.width - 1) +
        get_column(vertex_index, board.width) - 1;
    return board.vertical_edges_size + offset;
}

fn make_board(height: usize, width: usize) -> Board {
    assert!(0 < height);
    assert!(0 < width);
    let vertices_size = height * width;
    let vertical_edges_size = (height - 1) * width;
    let left_diagonal_edges_size = (height - 1) * (width - 1); 
    let right_diagonal_edges_size = height * (width - 1);
    let edges_size = vertical_edges_size + left_diagonal_edges_size + right_diagonal_edges_size;

    let mut vertices: Vec<Vertex> = vec![];
    let mut edges: Vec<Edge> = vec![];
    for _ in 0..vertices_size {
        let vertex: Vertex = Default::default();
        vertices.push(vertex);
    }
    for _ in 0..edges_size {
        let edge: Edge = Default::default();
        edges.push(edge);
    }
    let board = Board {
        width: width,
        height: height,
        vertices_size: vertices_size,
        vertical_edges_size: vertical_edges_size,
        right_diagonal_edges_size: right_diagonal_edges_size,
        left_diagonal_edges_size: left_diagonal_edges_size,
        edges_size: edges_size,
        vertices: vertices, edges:edges
    };
    return board;
}

impl Board {
    fn setup(&self) -> &Board {
        return self;
    }
}

fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    info!("settlers in rust");

    info!("creating board");

    let board: Board = make_board(10, 10);
    assert_eq!(1, get_north_vertex(11, board));
}

#[test]
fn test_get_north_edge() {
    let board = make_board(4, 4);
    assert_eq!(1, get_north_edge(5, board));
}

#[test]
fn test_get_northeast_edge() {
    let board = make_board(4, 4);
    assert_eq!(25, get_northeast_edge(5, board));
}

#[test]
fn test_get_southeast_edge() {
    let board = make_board(4, 4);
    assert_eq!(16, get_southeast_edge(5, board));
}

#[test]
fn test_get_south_edge() {
    let board = make_board(4, 4);
    assert_eq!(5, get_south_edge(5, board));
}

#[test]
fn test_get_southwest_edge() {
    let board = make_board(4, 4);
    assert_eq!(27, get_southwest_edge(5, board));
}

#[test]
fn test_get_northwest_edge() {
    let board = make_board(4, 4);
    assert_eq!(15, get_northwest_edge(5, board));
}

