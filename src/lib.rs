use std::cmp::max;
use log::trace;
use regex::Regex;

/// Represents a single node within a graph
#[derive(Debug)]
struct Node {
    label: String,
}
impl Node {
    pub fn new(label: String) -> Self {
        Self { label }
    }
}

/// Represents a connection (edge) between two nodes in a graph
#[derive(Debug)]
struct Edge {
    from: String,
    to: String,
    label: String,
}
impl Edge {
    pub fn new(from: String, to: String, label: String) -> Self {
        Self { from, to, label }
    }
}

/// Represents a complete graph structure
pub struct Graph {
    edge_list: Vec<Edge>,
    node_list: Vec<Node>,
    directed: bool,
    layout: String,
    layout_settings: String,
    node_settings: String,
}

///  Error types that can occur during graph parsing
pub enum GraphSyntaxError {
    ListTooLargeError,
    LabelTooLargeError,
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

    /// Tries to parse a textual representation of a graph
    ///
    /// # Arguments
    /// * `list`:  The textual representation of the graph
    ///
    /// # Returns
    /// * `Result<(), GraphSyntaxError>`:
    ///    Returns Ok(()) on success, or a GraphSyntaxError if the parsing fails.
    pub fn try_parse(&mut self, list: impl Into<String>) -> Result<(), GraphSyntaxError> {
        self.edge_list.clear();
        self.node_list.clear();

        let list = list.into();
        let lines: Vec<&str> = list.split("\n").collect();

        if lines.iter().count() > 50 {
            return Err(GraphSyntaxError::ListTooLargeError);
        }
        for line in lines {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let parts: Vec<&str> = Regex::new(r"\s+").unwrap().splitn(line, 3).collect();
            match parts[..] {
                [node] => {
                    if node.len() > 10 {
                        return Err(GraphSyntaxError::LabelTooLargeError);
                    }
                    self.add_node(Node::new(node.to_string()))
                }
                [from, to] => {
                    if max(from.len(), to.len()) > 10 {
                        return Err(GraphSyntaxError::LabelTooLargeError);
                    }
                    self.add_edge(Edge::new(from.to_string(), to.to_string(), "".to_string()))
                }
                [from, to, label] => {
                    if max(max(from.len(), to.len()), label.len()) > 10 {
                        return Err(GraphSyntaxError::LabelTooLargeError);
                    }
                    self.add_edge(Edge::new(from.to_string(), to.to_string(), label.to_string()))
                }
                _ => {}
            };
        }
        Ok(())
    }

    /// Generates a DOT representation of the graph (for use with Graphviz)
    ///
    /// # Returns
    /// A String containing the DOT representation
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

            dot.push_str(&format!("\"{}\" {} \"{}\" [label=\"{}\"]\n",
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

/// Helper function to count the number of lines in a string
pub fn number_of_lines(str: &str) -> usize {
    str.split('\n').count()
}