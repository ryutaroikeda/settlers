#[macro_use]
extern crate log;
extern crate log4rs;

// Represent the game board as a grid of triangles.
// In hindsight, we make it complicated to work with hexagons. It might've been better to go from
// hexagons down to edges and vertices instead of vertices up to edges and hexagons.
// The grid of triangles is shaped as a rhombus.
// The rhombus is orientated so that we get a horizontal (pointy topped) hexagonal grid.
// The vertices are stored in an array.
// This is how the vertices are indexed.
//
// Indices for vertices
//
//        0
//        |\ 
//        | \ 1
//        | /|\
//      4 |/ | \ 2
//        |\ | /|\
//        |5\|/ | \ 3
//        | /|\ | /|
//      8 |/ |6\|/ |
//         \ | /|\ |
//          \|/ | \| 7
//         9  \ | /|
//             \|/ |
//           10  \ |
//                \|
//              11
//
//
// Edges are also stored in an array.
// There are three types of edges: vertical, right diagonal, and left diagonal,
// Vertical means the edges going north to south.
// Right diagonal means the edges going down from left to right.
// Left diagonal means the edges going down from right to left.
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
//               |/  |   \|
//                \  |   /  Southeast
//       Southwest \ |  /
//                  \| /
//                   \/
//                  South
//
//
// Indices for vertical edges
//
//         
//        |\ 
//      0 | \  
//        | /|\
//        |/1| \  
//        |\ | /|\
//      4 | \|/2| \  
//        | /|\ | /|
//        |/5| \|/3|
//         \ | /|\ |
//          \|/6| \|  
//            \ | /|
//             \|/7|
//               \ |
//                \|
//                
//
// Indices for right diagonal edges
//
// Add 8 (the number of vertical edges) to each index.
//         
//        |\0
//        | \  
//        | /|\1
//        |/ | \  
//        |\3| /|\2
//        | \|/ | \  
//        | /|\4| /|
//        |/ | \|/ |
//         \6| /|\5|
//          \|/ | \|  
//            \7| /|
//             \|/ |
//               \8|
//                \|
//
// Indices for left diagonal edges
//
// Add 17 (the number of vertical edges and right diagonal edges) to each index.
//         
//        |\ 
//        | \  
//        |0/|\ 
//        |/ | \  
//        |\ |1/|\ 
//        | \|/ | \  
//        |3/|\ |2/|
//        |/ | \|/ |
//         \ |4/|\ |
//          \|/ | \|  
//            \ |5/|
//             \|/ |
//               \ |
//                \|
//                
// Offset coordinates for hexagonal grids
//
//           /\  /\  /\
//          /  \/  \/  \
//         |   ||   |   |
//         |0,0||0,1|0,2|
//          \  /\  /\  /\
//           \/  \/  \/  \
//           |   ||   |   |
//           |1,0||1,1|1,2|
//           /\  /\  /\  /
//          /  \/  \/  \/
//         |   ||   |   |
//         |2,0||2,1|2,2|
//          \  /\  /\  /
//           \/  \/  \/
//
// The coordinates can be translated into a vertex index for the face of the hexagon.
// Some coordinates like (0, 2) are invalid because they don't fit on the rhombus.
//

// The kinds of vertices.
// Invalid means they're not used.
// Intersection means you can build settlements and cities on them.
// Face means it has a resource, and you can put a robber on it.
pub enum VertexKind {
    Invalid,
    Intersection,
    Face,
}

impl Default for VertexKind {
    fn default() -> VertexKind {
        return VertexKind::Invalid;
    }
}

// The kinds of edges.
// Invalid means they're not used.
// Valid means you can build roads on them.
pub enum EdgeKind {
    Invalid,
    Valid,
}

impl Default for EdgeKind {
    fn default() -> EdgeKind {
        return EdgeKind::Invalid;
    }
}

// @cleanup use enums later for some fields.
#[derive(Default)]
pub struct Vertex {
    resource: i32,
    resource_number: i32,
    building_color: i32,
    building_type: i32,
    robber: i32,
    kind: VertexKind,
}

#[derive(Default)]
pub struct Harbor {
    input_resource: i32,
    input_resource_count: i32,
    output_resource: i32,
    output_resouce_count: i32,
}

#[derive(Default)]
pub struct Edge {
    road_color: i32,
    harbor: Harbor,
    kind: EdgeKind,
}

pub struct Board {
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

pub fn get_row(vertex_index: usize, board_width: usize) -> usize {
    return vertex_index / board_width;
}

pub fn get_column(vertex_index: usize, board_width: usize) -> usize {
    return vertex_index % board_width;
}

pub fn get_north_vertex(vertex_index: usize, board: &Board) -> usize {
    return vertex_index - board.width;
}

pub fn get_northeast_vertex(vertex_index: usize, board: &Board) -> usize {
    return vertex_index - board.width + 1;
}

pub fn get_southeast_vertex(vertex_index: usize, _: &Board) -> usize {
    return vertex_index + 1;
}

pub fn get_south_vertex(vertex_index: usize, board: &Board) -> usize {
    return vertex_index + board.width;
}

pub fn get_southwest_vertex(vertex_index: usize, board: &Board) -> usize {
    return vertex_index + board.width - 1;
}

pub fn get_northwest_vertex(vertex_index: usize, _: &Board) -> usize {
    return vertex_index - 1;
}

pub fn get_neighbor_vertices(vertex_index: usize, board: &Board) -> [usize; 6] {
    return [
        get_north_vertex(vertex_index, &board),
        get_northeast_vertex(vertex_index, &board),
        get_southeast_vertex(vertex_index, &board),
        get_south_vertex(vertex_index, &board),
        get_southwest_vertex(vertex_index, &board),
        get_northwest_vertex(vertex_index, &board),
    ];
}

pub fn get_north_edge(vertex_index: usize, board: &Board) -> usize {
    return (get_row(vertex_index, board.width) -1) * board.width +
        get_column(vertex_index, board.width);
}

pub fn get_northeast_edge(vertex_index: usize, board: &Board) -> usize {
    let offset = (get_row(vertex_index, board.width) - 1) * (board.width - 1) +
        get_column(vertex_index, board.width);
    return board.vertical_edges_size + board.right_diagonal_edges_size + offset;
}

pub fn get_southeast_edge(vertex_index: usize, board: &Board) -> usize {
    let offset = get_row(vertex_index, board.width) * (board.width - 1) +
        get_column(vertex_index, board.width);
    return board.vertical_edges_size  + offset;
}

pub fn get_south_edge(vertex_index: usize, board: &Board) -> usize {
    // This can be simplified to `return vertex_index`
    return get_row(vertex_index, board.width) * board.width +
        get_column(vertex_index, board.width);
}

pub fn get_southwest_edge(vertex_index: usize, board: &Board) -> usize {
    let offset = get_row(vertex_index, board.width) * (board.width - 1) +
        get_column(vertex_index, board.width) - 1;
    return board.vertical_edges_size + board.right_diagonal_edges_size + offset;
}

pub fn get_northwest_edge(vertex_index: usize, board: &Board) -> usize {
    let offset = get_row(vertex_index, board.width) * (board.width - 1) +
        get_column(vertex_index, board.width) - 1;
    return board.vertical_edges_size + offset;
}

pub fn get_neighbor_edges(vertex_index: usize, board: &Board) -> [usize; 6] {
    return [
        get_north_edge(vertex_index, &board),
        get_northeast_edge(vertex_index, &board),
        get_southeast_edge(vertex_index, &board),
        get_south_edge(vertex_index, &board),
        get_southwest_edge(vertex_index, &board),
        get_northwest_edge(vertex_index, &board),
    ];
}

pub fn get_northeast_perimeter(vertex_index: usize, board: &Board) -> usize {
    let north = get_north_vertex(vertex_index, &board);
    return get_southeast_edge(north, &board);
}

pub fn get_east_perimeter(vertex_index: usize, board: &Board) -> usize {
    let northeast = get_northeast_vertex(vertex_index, &board);
    return get_south_edge(northeast, &board);
}

pub fn get_southeast_perimeter(vertex_index: usize, board: &Board) -> usize {
    let southeast = get_southeast_vertex(vertex_index, &board);
    return get_southwest_edge(southeast, &board);
}

pub fn get_southwest_perimeter(vertex_index: usize, board: &Board) -> usize {
    let south = get_south_vertex(vertex_index, &board);
    return get_northwest_edge(south, &board);
}

pub fn get_west_perimeter(vertex_index: usize, board: &Board) -> usize {
    let southwest = get_southwest_vertex(vertex_index, &board);
    return get_north_edge(southwest, &board);
}

pub fn get_northwest_perimeter(vertex_index: usize, board: &Board) -> usize {
    let northwest = get_northwest_vertex(vertex_index, &board);
    return get_northeast_edge(northwest, &board);
}

pub fn get_perimeter_edges(vertex_index: usize, board: &Board) -> [usize; 6] {
    return [
        get_northeast_perimeter(vertex_index, &board),
        get_east_perimeter(vertex_index, &board),
        get_southeast_perimeter(vertex_index, &board),
        get_southwest_perimeter(vertex_index, &board),
        get_west_perimeter(vertex_index, &board),
        get_northwest_perimeter(vertex_index, &board),
    ];
}

// If vertex_index is the middle of a hexagon, then get_*_face returns the index of the vertex in 
// the middle of the neighboring hexagon.
pub fn get_northeast_face(vertex_index: usize, board: &Board) -> usize {
    let northeast_index = get_northeast_vertex(vertex_index, &board);
    return get_north_vertex(northeast_index, &board);
}

pub fn get_east_face(vertex_index: usize, board: &Board) -> usize {
    let northeast_index = get_northeast_vertex(vertex_index, &board);
    return get_southeast_vertex(northeast_index, &board);
}

// Get the vertex index for the middle of the hexagon at (row, column) in offset-coordinates.
pub fn get_vertex_from_hex(hex_row: usize, hex_column: usize, board: &Board) -> usize {
    let row_offset = (hex_row % 2) + 1;
    let row = 3 * (hex_row / 2) + row_offset;
    // Trying to avoid underflow.
    let row = row - hex_column;
    let column_offset = (hex_row % 2) + 1;
    let column = hex_column * 2 + column_offset;
    return row * board.width + column;
}

// Make a hexagon centered on the given vertex.
pub fn make_hexagon(vertex_index: usize, board: &mut Board) {
    board.vertices[vertex_index].kind = VertexKind::Face;
    // Make each neighbor vertex a valid intersection.
    let neighbor_vertex_indices = get_neighbor_vertices(vertex_index, &board);
    for neighbor_vertex_index in &neighbor_vertex_indices {
        board.vertices[*neighbor_vertex_index].kind = VertexKind::Intersection;
    }
    let neighbor_edge_indices = get_neighbor_edges(vertex_index, &board);
    for neighbor_edge_index in &neighbor_edge_indices {
        board.edges[*neighbor_edge_index].kind = EdgeKind::Invalid;
    }
    let perimeter_edge_indices = get_perimeter_edges(vertex_index, &board);
    for perimeter_edge_index in &perimeter_edge_indices {
        board.edges[*perimeter_edge_index].kind = EdgeKind::Valid;
    }
}

// Make a triangular board.
pub fn make_board(height: usize, width: usize) -> Board {
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

fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    info!("settlers in rust");

    info!("creating board");

    let mut board: Board = make_board(10, 10);

    make_hexagon(22, &mut board);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_north_vertex() {
        let board: Board = make_board(10, 10);
        assert_eq!(1, get_north_vertex(11, &board));
    }

    #[test]
    fn test_get_northeast_vertex() {
        let board: Board = make_board(4, 4);
        assert_eq!(2, get_northeast_vertex(5, &board));
    }

    #[test]
    fn test_get_southeast_vertex() {
        let board: Board = make_board(4, 4);
        assert_eq!(6, get_southeast_vertex(5, &board));
    }

    #[test]
    fn test_get_south_vertex() {
        let board: Board = make_board(4, 4);
        assert_eq!(9, get_south_vertex(5, &board));
    }

    #[test]
    fn test_get_southwest_vertex() {
        let board: Board = make_board(4, 4);
        assert_eq!(8, get_southwest_vertex(5, &board));
    }

    #[test]
    fn test_get_northwest_vertex() {
        let board: Board = make_board(4, 4);
        assert_eq!(4, get_northwest_vertex(5, &board));
    }

    #[test]
    fn test_get_north_edge() {
        let board = make_board(4, 4);
        assert_eq!(1, get_north_edge(5, &board));
    }

    #[test]
    fn test_get_northeast_edge() {
        let board = make_board(4, 4);
        assert_eq!(25, get_northeast_edge(5, &board));
    }

    #[test]
    fn test_get_southeast_edge() {
        let board = make_board(4, 4);
        assert_eq!(16, get_southeast_edge(5, &board));
    }

    #[test]
    fn test_get_south_edge() {
        let board = make_board(4, 4);
        assert_eq!(5, get_south_edge(5, &board));
    }

    #[test]
    fn test_get_southwest_edge() {
        let board = make_board(4, 4);
        assert_eq!(27, get_southwest_edge(5, &board));
    }

    #[test]
    fn test_get_northwest_edge() {
        let board = make_board(4, 4);
        assert_eq!(15, get_northwest_edge(5, &board));
    }

    #[test]
    fn test_get_northeast_perimeter() {
        let board = make_board(4, 4);
        assert_eq!(13, get_northeast_perimeter(5, &board));
    }

    #[test]
    fn test_get_east_perimeter() {
        let board = make_board(4, 4);
        assert_eq!(2, get_east_perimeter(5, &board));
    }

    #[test]
    fn test_get_southeast_perimeter() {
        let board = make_board(4, 4);
        assert_eq!(28, get_southeast_perimeter(5, &board));
    }

    #[test]
    fn test_get_southwest_perimeter() {
        let board = make_board(4, 4);
        assert_eq!(18, get_southwest_perimeter(5, &board));
    }

    #[test]
    fn test_get_west_perimeter() {
        let board = make_board(4, 4);
        assert_eq!(4, get_west_perimeter(5, &board));
    }

    #[test]
    fn test_get_northwest_perimeter() {
        let board = make_board(4, 4);
        assert_eq!(24, get_northwest_perimeter(5, &board));
    }

    #[test]
    fn test_get_northeast_face() {
        let board = make_board(4, 4);
        assert_eq!(2, get_northeast_face(9, &board));
    }

    #[test]
    fn test_get_east_face() {
        let board = make_board(4, 4);
        assert_eq!(3, get_east_face(5, &board));
    }

    #[test]
    fn test_get_vertex_from_hex() {
        let board = make_board(10, 10);
        assert_eq!(11, get_vertex_from_hex(0, 0, &board));
        assert_eq!(3, get_vertex_from_hex(0, 1, &board));
        assert_eq!(22, get_vertex_from_hex(1, 0, &board));
        assert_eq!(14, get_vertex_from_hex(1, 1, &board));
        assert_eq!(41, get_vertex_from_hex(2, 0, &board));
        assert_eq!(33, get_vertex_from_hex(2, 1, &board));
        assert_eq!(25, get_vertex_from_hex(2, 2, &board));
    }

}

