use std::fmt::Display;

pub struct BinarySearchTree<T>
where
    T: Ord + Display,
{
    pub value: Option<T>,
    pub left: Option<Box<BinarySearchTree<T>>>,
    pub right: Option<Box<BinarySearchTree<T>>>,
}

impl<T> BinarySearchTree<T>
where
    T: Ord + Display,
{
    pub fn new() -> BinarySearchTree<T> {
        BinarySearchTree {
            value: None,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, value: T) {
        match &self.value {
            None => self.value = Some(value),
            Some(key) => {
                if value == *key {
                    return;
                }
                let target_node = if value < *key {
                    &mut self.left
                } else {
                    &mut self.right
                };
                match target_node {
                    Some(ref mut node) => {
                        node.insert(value);
                    }
                    None => {
                        let mut node = BinarySearchTree::new();
                        node.insert(value);
                        *target_node = Some(Box::new(node));
                    }
                }
            }
        }
    }

    pub fn dfs(&self) {
        match &self.left {
            Some(left) => {
                left.dfs();
            }
            _ => {}
        }

        match &self.value {
            Some(key) => print!("{}\n", *key),
            _ => {}
        }

        match &self.right {
            Some(right) => {
                right.dfs();
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut tree: BinarySearchTree<u128> = BinarySearchTree::new();
        tree.insert(10);
        tree.insert(3);
        tree.insert(20);
        tree.insert(8);
        tree.insert(1);
        tree.insert(16);
        tree.insert(22);
        tree.dfs();
    }
}
