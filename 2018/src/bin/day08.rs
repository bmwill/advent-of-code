use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let root = Node::build_root(&mut input.trim().split(" ").map(|c| c.parse().unwrap()));

    part1(&root);
    part2(&root);

    Ok(())
}

fn part1(root: &Node) {
    println!("Sum of metadata: {}", root.sum_metadata());
}

fn part2(root: &Node) {
    println!("Root node value: {}", root.get_node_value());
}

struct Node {
    children: Vec<Node>,
    metadata: Vec<u32>,
}

impl Node {
    fn build_root(input_stream: &mut impl Iterator<Item = u32>) -> Self {
        let mut children = vec![];
        let mut metadata = vec![];
        let num_children = input_stream.next().unwrap();
        let num_metadata = input_stream.next().unwrap();

        // Process the children
        for _ in 0..num_children {
            let child = Self::build_root(input_stream);
            children.push(child);
        }

        // Process the metadata
        for _ in 0..num_metadata {
            metadata.push(input_stream.next().unwrap());
        }

        Self { children, metadata }
    }

    fn sum_metadata(&self) -> u32 {
        let own_sum = self.metadata.iter().sum::<u32>();

        let children_sum = self
            .children
            .iter()
            .map(|child| child.sum_metadata())
            .sum::<u32>();

        own_sum + children_sum
    }

    fn get_node_value(&self) -> u32 {
        if self.children.is_empty() {
            self.metadata.iter().sum::<u32>()
        } else {
            let mut value = 0;
            for &index in &self.metadata {
                // zero doesn't refer to a child
                if index == 0 {
                    continue;
                }

                if let Some(child) = self.children.get((index - 1) as usize) {
                    value += child.get_node_value();
                }
            }

            value
        }
    }
}
