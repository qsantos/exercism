pub mod graph {
    pub mod graph_items {
        pub mod node {
            use std::collections::HashMap;

            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct Node {
                pub label: String,
                pub attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(label: &str) -> Self {
                    Node {
                        label: String::from(label),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for (key, value) in attrs.iter().copied() {
                        self.attrs.insert(String::from(key), String::from(value));
                    }
                    self
                }

                pub fn attr(&self, key: &str) -> Option<&str> {
                    Some(self.attrs.get(key)?)
                }
            }
        }
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct Edge {
                from: String,
                to: String,
                pub attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(from: &str, to: &str) -> Self {
                    Edge {
                        from: String::from(from),
                        to: String::from(to),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for (key, value) in attrs.iter().copied() {
                        self.attrs.insert(String::from(key), String::from(value));
                    }
                    self
                }

                pub fn attr(&self, key: &str) -> Option<&str> {
                    Some(self.attrs.get(key)?)
                }
            }
        }
    }
    use std::collections::HashMap;

    use graph_items::edge::Edge;
    use graph_items::node::Node;
    #[derive(Default)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph::default()
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes.extend(nodes.iter().cloned());
            self
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges.extend(edges.iter().cloned());
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            for (key, value) in attrs.iter().copied() {
                self.attrs.insert(String::from(key), String::from(value));
            }
            self
        }

        pub fn node(&self, label: &str) -> Option<Node> {
            self.nodes.iter().find(|&node| node.label == label).cloned()
        }

        pub fn attr(&self, key: &str) -> Option<&str> {
            Some(self.attrs.get(key)?)
        }
    }
}
