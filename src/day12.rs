use crate::prelude::*;

const MAX_NODES: usize = 32;

#[derive(Debug, Clone)]
struct Graph {
    start: usize,
    end: usize,
    small: Vec<bool>,
    edges: HashMap<usize, Vec<usize>>,
}

fn parse(input: &str) -> Graph {
    let iter = input.lines().map(|line| {
        let mut split = line.split('-');
        let from = split.next().unwrap();
        let to = split.next().unwrap();
        (from, to)
    });
    let mut ids = HashMap::default();
    let mut small = Vec::new();
    let mut create_node = |k: &str| {
        *ids.entry(k.to_owned()).or_insert_with(|| {
            small.push(k.chars().all(|c| c.is_lowercase()));
            small.len() - 1
        })
    };
    let start = create_node("start");
    let end = create_node("end");
    let mut edges = HashMap::default();
    for (k, v) in iter {
        let kid = create_node(k);
        let vid = create_node(v);
        edges.entry(kid).or_insert_with(Vec::new).push(vid);
        if kid != start && vid != end {
            edges.entry(vid).or_insert_with(Vec::new).push(kid);
        }
    }
    if small.len() > MAX_NODES {
        panic!("too many nodes in input: {} > {}", small.len(), MAX_NODES);
    }
    Graph {
        start,
        end,
        small,
        edges,
    }
}

fn part_one(graph: Graph) -> usize {
    println!("{:?}", graph);
    let visited = [false; MAX_NODES];
    search(&graph, graph.start, visited, false)
}

fn part_two(graph: Graph) -> usize {
    let visited = [false; MAX_NODES];
    search(&graph, graph.start, visited, true)
}

fn search(
    graph: &Graph,
    node: usize,
    mut visited: [bool; MAX_NODES],
    can_visit_twice: bool,
) -> usize {
    if node == graph.end {
        return 1;
    }
    visited[node] = graph.small[node];
    let edges = graph.edges.get(&node);
    let neighbors = edges
        .iter()
        .flat_map(|v| v.iter().cloned())
        .filter(|&n| !visited[n] || can_visit_twice)
        .collect::<Vec<_>>();
    let mut total = 0;
    for n in neighbors {
        total += search(graph, n, visited, !visited[n] && can_visit_twice);
    }
    total
}

pub fn run(runner: &Runner) {
    runner.run(parse, part_one, part_two);
}
