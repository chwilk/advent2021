use std::env;
use std::fs;
use advent2021::find_filename;

fn main() {
    let filename = find_filename(env::args());
    let contents = fs::read_to_string(filename).expect("File error"); 

    let mut graph  = Graph::new();

    for line in contents.lines() {
        let edge: Vec<&str> = line.split('-').collect();
        graph.add_edge((String::from(edge[1]), String::from(edge[0])));
    }

    println!("{:?}", graph);
    
}

#[derive(Debug)]
struct Node {
    name: String,
    visited: bool,
}

#[derive(Debug)]
struct Edge {
    start: String,
    end: String,
}

#[derive(Debug)]
struct Graph {
    edges: Vec<Edge>,
    nodes: Vec<Node>,
}

impl Graph {
    fn new () -> Graph {
        Graph {
            edges: Vec::new(),
            nodes: vec![
                Node {
                    name: "start".to_string(),
                    visited: false,
                },
                Node {
                    name: "end".to_string(),
                    visited: false,
                },
            ],
        }
    }
    fn add_edge (&mut self, edge: (String, String)) -> &mut Graph {
        self.edges.push(Edge {
            start: edge.0.clone(),
            end: edge.1.clone(),
        });
        self.add_node(edge.0);
        self.add_node(edge.1);
        self
    }
    fn add_node (&mut self, node: String) -> &mut Graph {
        match self.find_node(&node) {
            None => self.nodes.push(Node {name: node, visited: false}),
            _ => (),
        }
        self
    }
    // Returns whether cave is visited already
    fn find_node (&mut self, node: &String) -> Option<bool> {
        for n in &self.nodes {
            if n.name == *node { return Some(n.visited) }
        }
        None
    }
    fn find_paths(&mut self, current: String) -> Vec<String> {
        let mut res = Vec::new();
        let mut cur = current;
        for edge in self.edges.iter() {
            if edge.start == cur {
                res.push(edge.end.clone());
                cur = edge.end.clone();
            }
        }
        res
    }


}
