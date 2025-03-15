use std::collections::HashMap;

pub struct Dag {
    graph: Option<HashMap<usize, Vec<usize>>>,
}

impl Dag {
    pub fn create(edges: Vec<(usize, usize)>) -> Vec<usize> {
        let mut adjacency_map: HashMap<usize, Vec<usize>> = HashMap::new();

        for (k, v) in edges {
            let node = &mut adjacency_map.entry(k).or_default();
            node.push(v);
        }

        let graph = Dag {
            graph: Some(adjacency_map),
        };

        graph.topological_sort()
    }

    pub fn topological_sort(&self) -> Vec<usize> {
        let nodes = self.graph.as_ref().unwrap().keys();
        let mut stack = vec![];
        let mut visited = vec![];

        for node in nodes {
            self.get_order(node, &mut stack, &mut visited);
        }

        stack.reverse();

        stack
    }

    fn get_order(&self, node: &usize, stack: &mut Vec<usize>, visited: &mut Vec<usize>) {
        let nodes = self.graph.as_ref().unwrap().get(node);

        visited.push(*node);
        
        if nodes.is_some() {
            for value in nodes.unwrap() {
                if !(visited.contains(value)){
                    self.get_order(value, stack, visited);
                }
            }
        }
    
        
        if !stack.contains(node) {
            stack.push(*node);
            
        }
    }
}
