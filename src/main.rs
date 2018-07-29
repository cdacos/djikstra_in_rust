use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec::Vec;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        println!("Use Djikstra's algorithm to return the shortest path in a graph.\n\
                  Usage: {} <file_path> <start_id> <end_id>", &args[0]);
        return;
    }

    let file_path = &args[1];
    let start_id = &args[2].parse::<usize>().expect("");
    let end_id = &args[3].parse::<usize>().expect("");

    let f = File::open(file_path).expect("Unable to open file");
    let f = BufReader::new(f);

    let mut adjacencies: HashMap<usize, Vec<Edge>> = HashMap::new();
    let mut vertices = 0;

    for (i, line) in f.lines().enumerate() {
        let line = line.expect("Unable to read line");

        if i == 0 {
            vertices = line.parse::<usize>().expect("Failed to parse number of vertices");
        }
        else if i < vertices + 1 {
            adjacencies.insert(line.parse::<usize>().expect("Failed to parse vertex ID (expect usize)"), vec!());
        }
        else if i == vertices + 1 {
            line.parse::<usize>().expect("Failed to parse number of edges");
        }
        else {
            let edge: Vec<usize> = line.split_whitespace().map(|x| x.parse::<usize>().expect("Failed to parse edge")).collect();
            {
                let mut neighbours = adjacencies.get_mut(&edge[0]).expect("?");
                neighbours.push(Edge { id: edge[1], cost: edge[2] });
            }
            // Bi-directional edge, so add the opposite direction too
            {
                let mut neighbours = adjacencies.get_mut(&edge[1]).expect("?");
                neighbours.push(Edge { id: edge[0], cost: edge[2] });
            }
        }
    }

    let shortest_path = get_shortest_path(&adjacencies, *start_id, *end_id);

    if shortest_path == MAX_DISTANCE {
        println!("no path");
    }
    else {
        println!("{}", shortest_path);
    }
}

const MAX_DISTANCE: usize = usize::max_value();

#[derive(Copy, Clone, Eq, PartialEq)]
struct Edge {
    id: usize,
    cost: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for Edge {
    fn cmp(&self, other: &Edge) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.id.cmp(&other.id))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Edge) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_shortest_path(adjacencies: &HashMap<usize, Vec<Edge>>, start_id: usize, end_id: usize) -> usize {
    if start_id == end_id {
        return 0;
    }

    let mut min_queue: BinaryHeap<Edge> = BinaryHeap::new();
    let mut distances = adjacencies.keys().map(|x| (x, MAX_DISTANCE)).collect::<HashMap<&usize, usize>>();

    distances.entry(&start_id).or_insert(0);
    min_queue.push(Edge { id: start_id, cost: 0 });

    while let Some(Edge { id, cost }) = min_queue.pop() {
        if id == end_id { break; }

        for edge in adjacencies.get(&id).expect("msg") {
            let next = Edge { id: edge.id, cost: cost + edge.cost };

            if next.cost < *distances.get(&next.id).expect("msg") {
                distances.insert(&edge.id, next.cost);
                min_queue.push(next);
            }
        }
    }

    return *distances.get(&end_id).expect("msg");
}