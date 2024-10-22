pub struct Digraph {
    pub name: String,
    pub nodes: Vec<Node>,
    pub edges: Vec<(String, String)>,
}

impl Digraph {
    pub fn new(name: String, nodes: Vec<Node>, edges: Vec<(String, String)>) -> Self {
        Self { name, nodes, edges }
    }

    pub fn generate(self) -> String {
        let mut output = String::new();

        output.push_str(format!("digraph {} {{\n", self.name).as_str());

        for node in self.nodes {
            output.push_str(format!("    {}", node.name).as_str());

            if node.attributes.len() != 0 {
                output.push_str(" [");
            }

            for attr in &node.attributes {
                match attr {
                    NodeAttribute::Label(name) => {
                        output.push_str(format!("label=\"{}\" ", name).as_str())
                    }
                    NodeAttribute::Shape(name) => {
                        output.push_str(format!("shape=\"{}\" ", name).as_str())
                    }
                    NodeAttribute::FillColor(name) => {
                        output.push_str(format!("style=\"filled\" ").as_str());
                        output.push_str(format!("fillcolor=\"{}\" ", name).as_str())
                    }
                    NodeAttribute::BackgroundColor(name) => {
                        output.push_str(format!("bgcolor=\"{}\"", name).as_str())
                    }
                }
            }

            if node.attributes.len() != 0 {
                output.push(']');
            }
            output.push('\n');
        }

        for edge in self.edges {
            output.push_str(format!("    {} -> {}\n", edge.0, edge.1).as_str());
        }

        output.push('}');

        output
    }
}

pub enum NodeAttribute {
    Label(String),
    Shape(String),
    FillColor(String),
    BackgroundColor(String),
}

pub struct Node {
    pub name: String,
    pub attributes: Vec<NodeAttribute>,
}

impl Node {
    pub fn new(name: String, attributes: Vec<NodeAttribute>) -> Self {
        Self { name, attributes }
    }
}
