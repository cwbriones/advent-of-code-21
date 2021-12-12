use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
enum Node {
    Small(String),
    Big(String),
}

impl std::str::FromStr for Node {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let node = match s {
            s if s.chars().all(|c| c.is_lowercase()) => Node::Small(s.into()),
            s => Node::Big(s.into()),
        };
        Ok(node)
    }
}

#[derive(Debug, Clone)]
struct Graph {
    edges: HashMap<Node, Vec<Node>>,
}

fn parse(input: &str) -> Graph {
    let mut edges = HashMap::default();
    for line in input.lines() {
        let mut split = line.split('-');
        let from = split.next().unwrap().parse::<Node>().unwrap();
        let to = split.next().unwrap().parse::<Node>().unwrap();
        edges
            .entry(from.clone())
            .or_insert_with(Vec::new)
            .push(to.clone());
        edges.entry(to).or_insert_with(Vec::new).push(from);
    }
    Graph { edges }
}

fn part_one(graph: Graph) -> usize {
    // Try all paths with backtracking
    // let mut seen = HashSet::default();
    let mut path = String::new();
    let mut paths = HashSet::default();
    search_one(&graph, &mut path, &mut paths);
    paths.len()
}

fn part_two(graph: Graph) -> usize {
    let start = Node::Small("start".into());
    let small_iter = graph.edges.keys().filter_map(|n| {
        if let Node::Small(s) = n {
            Some(s)
        } else {
            None
        }
    });

    let mut remaining = small_iter
        .clone()
        .cloned()
        .map(|k| (k, 1))
        .collect::<HashMap<String, usize>>();
    let mut path = String::new();
    let mut paths = HashSet::default();
    for s in small_iter.filter(|s| *s != "start") {
        // mark each small node besides start as "can visit twice"
        *remaining.get_mut(s).unwrap() += 1;
        search(&graph, &start, &mut remaining, &mut path, &mut paths);
        path.clear();
        *remaining.get_mut(s).unwrap() -= 1;
    }
    paths.len()
}

fn search_one(graph: &Graph, path: &mut String, visited: &mut HashSet<String>) {
    let start = Node::Small("start".into());
    let small_iter = graph.edges.keys().filter_map(|n| {
        if let Node::Small(s) = n {
            Some(s)
        } else {
            None
        }
    });
    let mut remaining = small_iter
        .clone()
        .cloned()
        .map(|k| (k, 1))
        .collect::<HashMap<String, usize>>();
    search(graph, &start, &mut remaining, path, visited);
}

fn search(
    graph: &Graph,
    node: &Node,
    remaining: &mut HashMap<String, usize>,
    path: &mut String,
    visited: &mut HashSet<String>,
) {
    let s = match node {
        Node::Small(s) => s,
        Node::Big(s) => s,
    };
    let len = s.len();
    path.push(' ');
    path.push_str(s);
    match node {
        Node::Small(s) if s == "end" => {
            visited.insert(path.clone());
        }
        Node::Small(s) if remaining.get(s).map(|v| *v > 0).unwrap() => {
            *remaining.get_mut(s).unwrap() -= 1;
            let edges = graph.edges.get(node);
            for v2 in edges.iter().flat_map(|v| v.iter()) {
                search(graph, v2, remaining, path, visited);
            }
            *remaining.get_mut(s).unwrap() += 1;
        }
        Node::Big(_) => {
            let edges = graph.edges.get(node);
            for v2 in edges.iter().flat_map(|v| v.iter()) {
                search(graph, v2, remaining, path, visited);
            }
        }
        _ => {}
    }
    path.truncate(path.len() - len - 1);
}

pub fn run(runner: &Runner) {
    runner.run(parse, part_one, part_two);
}
