use std::cmp::Ordering;
use std::fmt::Debug;
use std::mem;

struct BinarySearchTree<T> {
    root: Option<Box<Node<T>>>,
}

struct Node<T> {
    value: T,
    left: BinarySearchTree<T>,
    right: BinarySearchTree<T>,
}

impl<T: Ord> BinarySearchTree<T> {
    fn new() -> Self {
        BinarySearchTree::<T> { root: None }
    }

    fn take(&mut self) -> BinarySearchTree<T> {
        BinarySearchTree {
            root: self.root.take(),
        }
    }

    fn insert(&mut self, value: T) {
        if let Some(ref mut root) = self.root {
            if root.value < value {
                root.right.insert(value);
            } else {
                root.left.insert(value);
            }
        } else {
            self.root = Some(Box::new(Node::<T> {
                value,
                left: BinarySearchTree::new(),
                right: BinarySearchTree::new(),
            }));
        }
    }

    fn search(&self, value: T) -> Option<&Node<T>> {
        if let Some(root) = &self.root {
            match root.value.cmp(&value) {
                Ordering::Greater => root.left.search(value),
                Ordering::Equal => Some(root),
                Ordering::Less => root.right.search(value),
            }
        } else {
            None
        }
    }

    fn delete(&mut self, value: &T) -> Option<Box<Node<T>>>
    where
        T: Clone,
    {
        if let Some(ref mut root) = self.root {
            match root.value.cmp(value) {
                Ordering::Greater => root.left.delete(value),
                Ordering::Less => root.right.delete(value),
                Ordering::Equal => {
                    if root.left.root.is_none() {
                        if let Some(right) = root.right.root.take() {
                            self.root.replace(right)
                        } else {
                            self.root.take()
                        }
                    } else if root.right.root.is_some() {
                            let min_value = root.right.min_value().unwrap().value.clone();
                            let mut other = root.right.delete(&min_value).unwrap();
                            mem::swap(&mut other.value, &mut root.value);

                            Some(other)
                    } else {
                            let left = root.left.root.take().unwrap();
                            self.root.replace(left)
                    }
                }
            }
        } else {
            None
        }
    }

    fn min_value(&self) -> Option<&Node<T>> {
        if let Some(ref root) = &self.root {
            let mut root = root;
            while let Some(next_root) = &root.left.root {
                root = next_root;
            }

            Some(root)
        } else {
            None
        }
    }

    fn max_value(&self) -> Option<&Node<T>> {
        if let Some(ref root) = &self.root {
            let mut root = root;
            while let Some(next_root) = &root.right.root {
                root = next_root;
            }

            Some(root)
        } else {
            None
        }
    }
}

type SplayTree<T> = BinarySearchTree<T>;

impl<T: Ord> SplayTree<T> {
    fn search_splay(&mut self, value: &T) -> Option<&Node<T>> {
        self.splay(value);

        if let Some(root) = &self.root {
            (root.value == *value).then_some(root)
        } else {
            None
        }
    }

    fn splay(&mut self, value: &T) {
        if let Some(ref mut root) = self.root {
            match root.value.cmp(value) {
                Ordering::Greater => {
                    if let Some(ref mut left) = root.left.root {
                        match left.value.cmp(value) {
                            Ordering::Greater => {
                                left.left.splay(value);

                                self.right_rotate();
                            },
                            Ordering::Less => {
                                left.right.splay(value);

                                if left.right.root.is_some() {
                                    root.left.left_rotate();
                                }
                            }
                            Ordering::Equal => ()
                        }

                        if let Some(ref mut root) = self.root {
                            if root.left.root.is_some() {
                                self.right_rotate();
                            }
                        }
                    }
                }
                Ordering::Less => {
                    if let Some(ref mut right) = root.right.root {
                        match right.value.cmp(value) {
                            Ordering::Greater => {
                                right.left.splay(value);

                                if right.left.root.is_some() {
                                    root.right.right_rotate();
                                }
                            },
                            Ordering::Less => {
                                right.right.splay(value);

                                self.left_rotate();
                            }
                            Ordering::Equal => ()
                        }

                        if let Some(ref mut root) = self.root {
                            if root.right.root.is_some() {
                                self.left_rotate();
                            }
                        }
                    }
                }
                Ordering::Equal => {}
            }
        }
    }

    fn left_rotate(&mut self) {
        if let Some(mut root) = self.root.take() {
            let mut right = root.right.take();
            let right_left = right.root.as_mut().unwrap().left.take();

            root.right = right_left;
            right.root.as_mut().unwrap().left.root = Some(root);

            *self = right;
        }
    }

    fn right_rotate(&mut self) {
        if let Some(mut root) = self.root.take() {
            let mut left = root.left.take();
            let left_right = left.root.as_mut().unwrap().right.take();

            root.left = left_right;
            left.root.as_mut().unwrap().right.root = Some(root);

            *self = left;
        }
    }

    fn join(mut t1: SplayTree<T>, t2: SplayTree<T>) -> SplayTree<T>
    where
        T: Clone,
    {
        if let Some(max_node) = t1.max_value() {
            let max_value = max_node.value.clone();
            t1.splay(&max_value);
            t1.root.as_mut().unwrap().right = t2;

            t1
        } else {
            t2
        }
    }

    fn split(&mut self, value: &T) -> Option<SplayTree<T>> {
        self.splay(value);
        if let Some(ref mut root) = self.root {
            let right = root.right.take();

            Some(right)
        } else {
            None
        }
    }

    fn insert_splay(&mut self, value: T)
    where
        T: Clone,
    {
        self.insert(value.clone());
        self.splay(&value);
    }

    fn delete_splay(&mut self, value: T)
    where
        T: Clone,
    {
        self.splay(&value);
        if let Some(ref mut root) = self.root {
            if root.value == value {
                let left = root.left.take();
                let right = root.right.take();

                *self = SplayTree::join(left, right);
            }
        }
    }
}

fn main() {
    let mut tree1 = SplayTree::new();

    let mut inserted1 = vec![5, 1, 4, 2, 7, 6, 9];
    let mut not_inserted1 = vec![0, 3, 8, 10, 11];
    test_insert(&mut tree1, &inserted1, &not_inserted1);
    test_splay(&mut tree1, &inserted1, &not_inserted1);

    let mut tree2 = SplayTree::new();
    let mut inserted2 = vec![17, 13, 19, 12, 21, 15, 23, 18];
    let mut not_inserted2 = vec![14, 16, 20, 22];
    test_insert(&mut tree2, &inserted2, &not_inserted2);
    test_splay(&mut tree2, &inserted2, &not_inserted2);

    let mut joined_tree = SplayTree::join(tree1, tree2);
    inserted1.append(&mut inserted2);
    not_inserted1.append(&mut not_inserted2);
    test_insert(&mut joined_tree, &inserted1, &not_inserted1);
    test_splay(&mut joined_tree, &inserted1, &not_inserted1);
}

fn test_insert<T: Clone + Ord>(tree: &mut SplayTree<T>, inserted: &[T], not_inserted: &[T]) {
    for val in inserted {
        tree.insert(val.clone());
    }

    for val in inserted {
        assert!(tree.search(val.clone()).is_some())
    }

    for val in not_inserted {
        assert!(tree.search(val.clone()).is_none())
    }
}

fn test_splay<T: Clone + Debug + Ord>(tree: &mut SplayTree<T>, inserted: &[T], not_inserted: &[T]) {
    for val in inserted {
        assert_eq!(tree.search_splay(val).unwrap().value, val.clone());
    }

    for val in not_inserted {
        assert!(tree.search_splay(val).is_none());
    }
}
