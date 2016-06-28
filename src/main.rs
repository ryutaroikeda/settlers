#[macro_use]
extern crate log;
extern crate log4rs;

// Represent hexagons with axial coordinates like this
//
//                / \ / \ / \
//               |0,0|0,1|0,2|
//                \ / \ / \ / \
//                 |1,0|1,1|1,2|
//                  \ / \ / \ / \
//                   |2,0|2,1|2,2|
//                    \ / \ / \ /
//
// Each hex is referenced by the tuple (row, column).
//
// The intersections are indexed like this
//
//             0   1   2   3
//              \4/ \5/ \6/ \7
//               |   |   |   |
//               8\C/9\D/A\E/B\F
//                 |   |   |   |
//                 G\K/H\L/I\M/J\N
//                   |   |   |   |
//                   O\ /P\ /Q\ /R\
//                     S   T   U   V
//
// There are three kinds of edges: vertical, right, left.
// They are indexed like this
//
// Vertical
//             0   1   2   3
//             |   |   |   |
//            / \4/ \5/ \6/ \7/
//               |   |   |   |
//              / \8/ \9/ \A/ \B/
//                 |   |   |   |
//                / \C/ \D/ \E/ \F/
//                   |   |   |   |
//                  / \G/ \H/ \I/ \J/
//                     |   |   |   |
//
// Right
//                          
//             |   |   |   |
//            / \0/ \1/ \2/ \3/
//               |   |   |   |
//              / \4/ \5/ \6/ \7/
//                 |   |   |   |
//                / \8/ \9/ \A/ \B/
//                   |   |   |   |
//                  / \C/ \D/ \E/ \F/
//                     |   |   |   |
//
// Left
//                          
//             |   |   |   |
//           0/ \1/ \2/ \3/ \4/ 
//               |   |   |   |
//             5/ \6/ \7/ \8/ \9/
//                 |   |   |   |
//               A/ \B/ \C/ \D/ \E/
//                   |   |   |   |
//                 F/ \G/ \H/ \I/ \J/
//                     |   |   |   |
//

#[derive(Default)]
pub struct Terrain {
    resource: i32,
}

#[derive(Default)]
pub struct Intersection {
    building: i32,
}

#[derive(Default)]
pub struct Edge {
    kind: i32,
}

#[derive(Default)]
pub struct Board {
    height: usize,
    width: usize,
    mem_height: usize,
    mem_width: usize,
    vertical_edge_size: usize,
    right_edge_size: usize,
    terrains: Vec<Terrain>,
    intersections: Vec<Intersection>,
    edges: Vec<Edge>,
}

impl Board {
    // Initialize a hexagonal grid in the shape of a rectangle.
    fn init(&mut self, height: usize, width: usize) {
        assert!(0 < height);
        assert!(0 < width);

        let mem_height = height;
        let mem_width = 1 + width + 1;

        let terrain_size = mem_height * mem_width;
        for _ in 0..terrain_size {
            let terrain: Terrain = Default::default();
            self.terrains.push(terrain);
        }

        let intersection_size = 2 * (height + 1) * (width + 1);
        for _ in 0..intersection_size {
            let intersection: Intersection = Default::default();
            self.intersections.push(intersection);
        }

        let vertical_edge_size = (height + 2) * (width + 1);
        // The number of edges going down from left to right.
        let right_edge_size = (height + 1) * (width + 1);
        let left_edge_size = (height + 1) * (width + 2);
        let edge_size = 
            vertical_edge_size +
            right_edge_size +
            left_edge_size;
        for _ in 0..edge_size {
            let edge: Edge = Default::default();
            self.edges.push(edge);
        }

        self.height = height;
        self.width = width;
        self.mem_height = mem_height;
        self.mem_width = mem_width;
        self.vertical_edge_size = vertical_edge_size;
        self.right_edge_size = right_edge_size;
    }

    fn get_terrain_index(&self, row: usize, column:usize) -> usize {
        return row * self.mem_width + column;
    }

    fn get_

}

fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    info!("settlers in rust");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_init() {
        let mut board: Board = Default::default();
        board.init(1, 1);
        assert_eq!(2, board.vertical_edge_size);
        assert_eq!(2, board.right_edge_size);
    }
}

