use log::trace;
use regex::Regex;

#[derive(Debug)]
pub struct Node {
    label: String,
}

impl Node {
    pub fn new(label: String) -> Self {
        Self { label }
    }
}

#[derive(Debug)]
pub struct Edge {
    from: String,
    to: String,
    label: String,
}

impl Edge {
    pub fn new(from: String, to: String, label: String) -> Self {
        Self { from, to, label }
    }
}

pub struct Graph {
    edge_list: Vec<Edge>,
    node_list: Vec<Node>,
    directed: bool,
    layout: String,
    layout_settings: String,
    node_settings: String,
}

impl Graph {
    pub fn new(directed: bool, layout: String, layout_settings: String, node_settings: String) -> Self {
        Self { edge_list: Vec::new(), node_list: Vec::new(), directed, layout, layout_settings, node_settings }
    }
    fn add_edge(&mut self, edge: Edge) {
        self.edge_list.push(edge);
    }
    fn add_node(&mut self, node: Node) {
        self.node_list.push(node);
    }
    pub fn try_parse(&mut self, list: impl Into<String>) -> Result<(), &'static str> {
        self.edge_list.clear();
        self.node_list.clear();

        let list = list.into();
        let lines: Vec<&str> = list.split("\n").collect();
        for line in lines {
            if line.trim().is_empty() {
                continue;
            }

            let x: Vec<&str> = Regex::new(r"\s+").unwrap().splitn(line, 3).collect();
            trace!("{:#?}",x);

            match x.len() {
                1 => {
                    let node = Node::new(x[0].into());
                    self.add_node(node);
                }
                2 => {
                    let edge = Edge::new(x[0].into(), x[1].into(), "".into());
                    self.add_edge(edge);
                }
                3 => {
                    let edge = Edge::new(x[0].into(), x[1].into(), x[2].into());
                    self.add_edge(edge);
                }
                _ => { todo!() }
            };
        }
        Ok(())
    }
    pub fn to_dot(&self) -> String {
        let mut dot: String = if self.directed { "digraph".into() } else { "graph".into() };
        dot.push_str(" {\n");

        dot.push_str(&format!("\tlayout =\"{}\"\n", self.layout));
        dot.push_str(&format!("\tnode [{}]\n", self.node_settings));
        dot.push_str(&self.layout_settings);

        for node in &self.node_list {
            dot.push_str(&format!("{}\n", node.label));
        }

        for edge in &self.edge_list {
            let arrow = if self.directed { "->" } else { "--" };

            dot.push_str(&format!("{} {} {} [label=\"{}\"]\n",
                                  edge.from,
                                  arrow,
                                  edge.to,
                                  edge.label,
            ));
        }

        dot.push('}');
        trace!("{:#?}",dot);
        dot
    }
}