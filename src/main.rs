//! Example of a dynamic graph data structure without pointer references.
//!
//! This type of implementations is useful for representing the kind of
//! graphs that occur in compilers, such as a Graph IR, CFG of basic
//! blocks and so on.
use std::collections::VecDeque;

/// The root node is always at 0th index.
const ROOT: NodeRef = NodeRef(0);

/// Strongly typed reference to a node in the graph.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct NodeRef(pub usize);

/// Node in the graph, could hold anything, for example it could
/// hold code and a list of edges to children.
#[derive(Debug)]
pub struct Node {
    // Identifier of the node.
    id: NodeRef,
    // Node label.
    pub label: String,
    // Edges for the node, you can have more complex edges using an enum
    // such as `Vec<(NodeRef, Edge)>` to represent edges that are unconditional
    // for example if the basic block branches to another edge regardless
    // of its internal state, or a conditional edge that has some input.
    edges: Vec<NodeRef>,
}

/// A Graph represented as a `Vec<Node>`, the struct can be more complex
/// and hold more metadata but for our purposes it will hold just the
/// nodes.
pub struct Graph {
    /// Nodes in the graph.
    nodes: Vec<Node>,
}

impl Graph {
    /// Create a new empty graph.
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    /// Add a new unlinked node to the graph.
    pub fn add_node(&mut self) -> NodeRef {
        let node_id = self.nodes.len();
        let node = Node {
            id: NodeRef(node_id),
            label: format!(".L{node_id}"),
            edges: Vec::new(),
        };

        self.nodes.push(node);

        NodeRef(node_id)
    }

    /// Add a new linked node to the graph.
    pub fn add_node_linked(&mut self, edges: Vec<NodeRef>) -> NodeRef {
        let node_id = self.nodes.len();
        let node = Node {
            id: NodeRef(node_id),
            label: format!(".L{node_id}"),
            edges: edges.clone(),
        };
        self.nodes.push(node);

        for &edge in &edges {
            self.link(NodeRef(node_id), edge)
        }

        NodeRef(node_id)
    }

    /// Return an immutable reference to a node.
    pub fn node_as_ref(&self, id: NodeRef) -> Option<&Node> {
        self.nodes.get(id.0)
    }

    /// Return a mutable reference to a node.
    pub fn node_as_mut_ref(&mut self, id: NodeRef) -> Option<&mut Node> {
        self.nodes.get_mut(id.0)
    }

    /// Link two nodes in the graph.
    pub fn link(&mut self, from: NodeRef, to: NodeRef) {
        let src_node = self.nodes.get_mut(from.0).unwrap();

        src_node.edges.push(to);
    }

    /// Walk the graph in BFS order.
    pub fn walk<F>(&self, mut visitor: F)
    where
        F: FnMut(&Node, Option<&Node>),
    {
        // BFS visit state.
        let mut visited = vec![false; self.nodes.len()];

        // BFS queue.
        let mut queue = VecDeque::new();
        // Since we start at the root push it first.
        queue.push_back(ROOT);

        while let Some(node) = queue.pop_front() {
            // Check if we visited this node and we should skip it.
            if visited[node.0] {
                continue;
            }

            // Mark it as visited.
            visited[node.0] = true;

            let node = self.node_as_ref(node).unwrap();

            if node.edges.len() > 0 {
                // Queue up edges to visit next.
                for &edge in &node.edges {
                    // Call visitor function.
                    visitor(node, self.node_as_ref(edge));
                    queue.push_back(edge);
                }
            } else {
                // Node has no edges.
                visitor(node, None);
            }
        }
    }
}

fn main() {
    // Let's build the following graph
    //                   __C
    //                  /
    // root -> A -> B -
    //         |________\_D
    //
    let mut graph = Graph::new();
    let root = graph.add_node();
    let a = graph.add_node();
    let b = graph.add_node();
    let c = graph.add_node();
    let d = graph.add_node();

    graph.link(root, a);
    graph.link(a, b);
    graph.link(b, c);
    graph.link(b, d);
    graph.link(d, a);

    graph.walk(|from, to| {
        let to_label = match to {
            Some(to) => to.label.as_str(),
            None => "None",
        };
        println!("Node {:?} is connected to {:?}", from.label, to_label);
    });
}
