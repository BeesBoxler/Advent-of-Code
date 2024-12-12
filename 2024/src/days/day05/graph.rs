use std::collections::HashMap;

pub struct Dag {
    graph: Option<HashMap<usize, Vec<usize>>>,
}

impl Dag {
    pub fn create(edges: Vec<(usize, usize)>) -> Vec<usize> {
        let mut adjacency_map: HashMap<usize, Vec<usize>> = HashMap::new();

        for (k, v) in edges {
            let node = &mut adjacency_map.entry(k).or_insert(vec![]);
            node.push(v);
        }

        let graph = Dag {
            graph: Some(adjacency_map),
        };

        graph.topological_sort()
    }

    pub fn topological_sort(&self) -> Vec<usize> {
        let nodes = self.graph.as_ref().unwrap().keys();
        let mut stack: Vec<usize> = vec![];

        for node in nodes {
            self.get_order(node, &mut stack);
        }

        stack.reverse();

        stack
    }

    fn get_order(&self, node: &usize, stack: &mut Vec<usize>) {
        let nodes = self.graph.as_ref().unwrap().get(node);

        if nodes.is_some() {
            for value in nodes.unwrap() {
                self.get_order(value, stack);
            }
        }

        if !stack.contains(node) {
            stack.push(*node);
        }
    }
}
