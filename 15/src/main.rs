use std::collections::HashMap;

#[derive(Debug)]
struct Node {
    id: usize,
    code: String,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(id: usize, code: &str) -> Self {
        Node {
            id,
            code: code.to_string(),
            left: None,
            right: None,
        }
    }

    fn add_child(&mut self, child: Node) {
        if child.id > self.id {
            if let Some(right) = self.right.as_mut() {
                right.add_child(child);
            } else {
                self.right = Some(Box::new(child));
            }
        } else {
            if let Some(left) = self.left.as_mut() {
                left.add_child(child);
            } else {
                self.left = Some(Box::new(child));
            }
        }
    }

    fn num_layer(&self) -> usize {
        let left_layer = if let Some(left) = &self.left { left.num_layer() } else { 0 };
        let right_layer = if let Some(right) = &self.right { right.num_layer() } else { 0 };
        left_layer.max(right_layer) + 1
    }

    fn ids_by_layer(&self, layer: usize, ids: &mut HashMap<usize, Vec<usize>>) {
        ids.entry(layer).or_default().push(self.id);
        if let Some(left) = &self.left {
            left.ids_by_layer(layer + 1, ids);
        }
        if let Some(right) = &self.right {
            right.ids_by_layer(layer + 1, ids);
        }
    }

    fn code_path(&self, path: &str, id: usize) -> String {
        if id < self.id {
            let path = format!("{}-{}", path, self.code);
            if let Some(left) = &self.left {
                left.code_path(&path, id)
            } else {
                path
            }
        } else {
            let path = format!("{}-{}", path, self.code);
            if let Some(right) = &self.right {
                right.code_path(&path, id)
            } else {
                path
            }
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input1.txt").unwrap();
    let (upper, lower) = input.split_once("\n\n").unwrap();
    let (root_code_str, root_id_str) = upper.trim().lines().next().unwrap().split_once(" | ").unwrap();
    let mut root = Node::new(root_id_str.parse().unwrap(), root_code_str);
    let tree = upper.trim().lines().skip(1).fold(&mut root, |acc, l| {
        let (code, id_str) = l.split_once(" | ").unwrap();
        acc.add_child(Node::new(id_str.parse().unwrap(), code));
        acc
    });

    // p1
    let mut ids_by_layer = HashMap::new();
    tree.ids_by_layer(1, &mut ids_by_layer);
    let p1 = ids_by_layer.iter().map(|(_, ids)| ids.iter().sum::<usize>()).max().unwrap() * tree.num_layer();
    println!("p1: {}", p1);

    let p2 = tree.code_path("", 500000);
    println!("p2: {}", &p2[1..]);

    // p3
    let (a, b) = lower.trim().split_once('\n').unwrap();
    let (a, b) = (
        a.split_once(" | ").unwrap().1.parse().unwrap(),
        b.split_once(" | ").unwrap().1.parse().unwrap(),
    );
    let p3a = tree.code_path("", a);
    let p3b = tree.code_path("", b);
    // get idx - 1 of first unequal string part of p3a and p3b
    let idx = *p3a
        .chars()
        .zip(p3b.chars())
        .enumerate()
        .filter(|(_, (a, b))| a != b)
        .take(1)
        .map(|(idx, (_, _))| idx)
        .peekable()
        .peek()
        .unwrap()
        - 1;
    println!("p3: {}", &p3a[idx - 7..idx]);
}
