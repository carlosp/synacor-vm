use std::collections::VecDeque;

/*
We have the following rooms organized in a 4 x 4 grid, each of which contains either a number or an operator:

     *  8  -  1
     4  * 11  *
     +  4  - 18
    22  -  9  *

Starting in the room with value 22, the goal is to reach the room with value 1 in the least number of steps
such that the value of the traversed path considering left-associative, same precedence operators, equals 30.

We can solve this using a BFS on the graph where each room with a number is a node, and each edge represents
a connection through one of the adjacent operators to a room adjacent to it, including the starting one.
As the solution should be unique we can also eliminate ambiguous edges connecting the same start and end nodes
via different paths (i.e. from room with number 9 to room with number 4 through W-N or N-W)
*/

const N: &str = "north";
const E: &str = "east";
const S: &str = "south";
const W: &str = "west";

enum Op {
    Add,
    Mul,
    Sub
}

struct Node {
    id: usize,
    value: i32,
    connections: Vec<([&'static str; 2], Op, usize)>
}

fn find_shortest_path(graph: &[Node], (start_node_id, start_value): (usize, i32), (end_node_id, end_value): (usize, i32)) -> Vec<&str> {
    let mut pending = VecDeque::from([(&graph[start_node_id], start_value, vec![])]);

    while !pending.is_empty() {
        let (node, value, path) = pending.pop_front().unwrap();

        if node.id == end_node_id && value == end_value {
            return path;
        }

        for (directions, op, next_node_id) in &node.connections {
            let next_node = &graph[*next_node_id];
            let next_value = match &op {
                Op::Add => value + next_node.value,
                Op::Mul => value * next_node.value,
                Op::Sub => value - next_node.value
            };
            let mut next_path: Vec<&str> = path.clone();

            next_path.extend(directions);
            pending.push_back((next_node, next_value, next_path));
        }
    }

    panic!("No path found")
}

fn main() {
    let graph = [
        Node { id: 0, value: 22, connections: vec![
            ([E, E], Op::Sub, 1), ([N, E], Op::Add, 2), ([E, N], Op::Sub, 2), ([N, N], Op::Add, 4), ]
        },
        Node { id: 1, value: 9, connections: vec![
            ([N, E], Op::Sub, 3), ([E, N], Op::Mul, 3), ([N, N], Op::Sub, 5)]
        },
        Node { id: 2, value: 4, connections: vec![
            ([E, E], Op::Sub, 3), ([W, N], Op::Add, 4), ([N, W], Op::Mul, 4), ([N, E], Op::Mul, 5), ([E, N], Op::Sub, 5), ([N, N], Op::Mul, 6)]
        },
        Node { id: 3, value: 18, connections: vec![
            ([W, S], Op::Sub, 1), ([S, W], Op::Mul, 1), ([W, N], Op::Sub, 5), ([N, W], Op::Mul, 5), ([N, N], Op::Mul, 7)]
        },
        Node { id: 4, value: 4, connections: vec![
            ([S, E], Op::Add, 4), ([E, S], Op::Mul, 4)]
        },
        Node { id: 5, value: 11, connections: vec![
            ([S, S], Op::Sub, 1), ([W, S], Op::Mul, 2), ([S, W], Op::Sub, 2), ([S, E], Op::Sub, 3), ([E, S], Op::Mul, 3),([N, E], Op::Sub, 7), ([E, N], Op::Mul, 7)]
        },
        Node { id: 6, value: 8, connections: vec![
            ([S, S], Op::Mul, 2), ([S, E], Op::Mul, 5), ([E, S], Op::Sub, 5), ([E, E], Op::Sub, 1)]
        },
        Node { id: 7, value: 1, connections: vec![] }
    ];

    println!("{}", find_shortest_path(&graph, (0, 22), (7, 30)).join("\n"));
}
