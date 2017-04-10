use std::cmp;
use std::ops::{Index, IndexMut};
use std::mem;

type Link = Option<Box<Node>>;

pub struct Buffer {
    root: Link,
}

// TODO: Implement iterator interface if indexing is a performance bottleneck
struct Node {
    size: usize,
    left: Link,
    right: Link,
    buf: Option<Vec<u32>>,
}

impl Index<usize> for Buffer {
    type Output = u32;

    // Call the mutable version, it doesn't modify the structure.
    fn index(&self, mut offset: usize) -> &u32 {
        let mut cur = &self.root;

        loop {
            match cur {
                &None => panic!("Index out of bounds"),
                &Some(ref node) => {
                    if node.size < offset {
                        offset -= node.size;
                        cur = &node.right;
                    } else if node.buf.is_none() {
                        cur = &node.left;
                    } else {
                        return &node.buf.as_ref().unwrap()[offset];
                    }
                }
            }
        }
    }
}

// TODO: How to avoid duplicating code?
impl IndexMut<usize> for Buffer {
    fn index_mut(&mut self, mut offset: usize) -> &mut u32 {
        let mut cur = &mut self.root;

        loop {
            match {
                      cur
                  } {
                &mut None => panic!("Index out of bounds"),
                &mut Some(ref mut node) => {
                    if node.size < offset {
                        offset -= node.size;
                        cur = &mut node.right;
                    } else if node.buf.is_none() {
                        cur = &mut node.left;
                    } else {
                        return &mut node.buf.as_mut().unwrap()[offset];
                    }
                }
            }
        }
    }
}

fn length(node: &Link) -> usize {
    match node {
        &None => 0,
        &Some(ref node) => node.size + length(&node.right),
    }
}

fn concat(n1: Link, n2: Link) -> Link {
    Some(Box::new(Node {
                      size: length(&n1),
                      left: n1,
                      right: n2,
                      buf: None,
                  }))
}

fn split(node_option: Link, index: usize) -> (Link, Link, usize) {
    if let None = node_option {
        panic!("Index out of bounds");
    }

    let mut node = node_option.unwrap();

    if node.size < index {
        let (new_right, right_split, right_split_size) = split(mem::replace(&mut node.right, None),
                                                               index - node.size);
        node.right = new_right;
        return (Some(node), right_split, right_split_size);
    }

    if node.buf.is_some() {
        if index == 0 {
            let node_size = node.size;
            return (None, Some(node), node_size);
        }

        let new_right_size = node.size - index;
        let mut new_right = Box::new(Node {
                                         size: new_right_size,
                                         left: None,
                                         right: None,
                                         buf: Some(Vec::with_capacity(new_right_size)),
                                     });

        for i in index..node.size {
            new_right.buf.as_mut().unwrap()[i - index] = node.buf.as_ref().unwrap()[i];
        }

        node.buf.as_mut().unwrap().resize(index, 0);

        return (Some(node), Some(new_right), new_right_size);
    };

    let (new_left, mut right_split, mut right_split_size) =
        split(mem::replace(&mut node.left, None), index);
    node.size -= right_split_size;
    node.left = new_left;

    if node.size == index && node.right.is_some() {
        right_split_size += length(&node.right);
        right_split = concat(right_split, mem::replace(&mut node.right, None));
    }

    (Some(node), right_split, right_split_size)
}

fn rotate_right(mut node: Link) -> Link {
    let mut x = mem::replace(&mut node.as_mut().unwrap().left, None);
    node.as_mut().unwrap().left = mem::replace(&mut x.as_mut().unwrap().right, node.take());
    node.as_mut().unwrap().size -= x.as_ref().unwrap().size;
    x
}

fn rotate_left(mut node: Link) -> Link {
    let mut x = mem::replace(&mut node.as_mut().unwrap().right, None);
    node.as_mut().unwrap().right = mem::replace(&mut x.as_mut().unwrap().left, node.take());
    x.as_mut().unwrap().size += node.as_ref().unwrap().size;
    x
}

// TODO: Remove single child nodes.
// TODO: This is hideous
fn splay(mut node: Link, idx: usize) -> Link {
    if node.is_none() {
        panic!("Not reached");
    }
    if node.as_ref().unwrap().buf.is_some() {
        return None;
    }

    if node.as_ref().unwrap().size >= idx {
        if node.as_mut().unwrap().left.as_mut().unwrap().size >= idx {
            node.as_mut().unwrap().left.as_mut().unwrap().left =
                splay(mem::replace(&mut node.as_mut().unwrap().left.as_mut().unwrap().left,
                                   None),
                      idx);
            node = rotate_right(node);
        } else {
            node.as_mut().unwrap().left.as_mut().unwrap().right =
                splay(mem::replace(&mut node.as_mut().unwrap().left.as_mut().unwrap().right,
                                   None),
                      idx - node.as_mut().unwrap().left.as_mut().unwrap().size);
            if node.as_mut()
                   .unwrap()
                   .left
                   .as_mut()
                   .unwrap()
                   .right
                   .is_some() {
                node.as_mut().unwrap().left =
                    rotate_left(mem::replace(&mut node.as_mut().unwrap().left, None));
            }
        }

        if node.as_ref().unwrap().left.is_none() {
            return node;
        } else {
            return rotate_right(node);
        }
    } else {
        let new_idx = idx - node.as_ref().unwrap().size;
        if node.as_mut().unwrap().right.as_mut().unwrap().size >= new_idx {
            node.as_mut().unwrap().right.as_mut().unwrap().left =
                splay(mem::replace(&mut node.as_mut().unwrap().right.as_mut().unwrap().left,
                                   None),
                      new_idx);
            if node.as_mut()
                   .unwrap()
                   .right
                   .as_mut()
                   .unwrap()
                   .left
                   .is_some() {
                node.as_mut().unwrap().right =
                    rotate_right(mem::replace(&mut node.as_mut().unwrap().right, None));
            }
        } else {
            node.as_mut().unwrap().right.as_mut().unwrap().right =
                splay(mem::replace(&mut node.as_mut().unwrap().right.as_mut().unwrap().right,
                                   None),
                      new_idx - node.as_mut().unwrap().right.as_mut().unwrap().size);
            node = rotate_left(node);
        }

        if node.as_ref().unwrap().right.is_none() {
            return node;
        } else {
            return rotate_left(node);
        }
    }
}

fn report(node_option: &Link, idx: usize, len: usize, mut out: &mut Vec<u32>) -> usize {
    if len == 0 {
        return 0;
    }

    if node_option.is_none() {
        panic!("Out of bounds");
    }

    let node = node_option.as_ref().unwrap();

    if node.size < idx {
        return report(&node.right, idx - node.size, len, &mut out);
    }

    if let Some(ref buf) = node.buf {
        let num_chars = cmp::min(len, buf.len());
        for i in 0..num_chars {
            out.push(buf[i]);
        }
        return num_chars;
    }

    let num_reported = report(&node.left, idx, len, &mut out);
    return num_reported + report(&node.right, idx, len - num_reported, &mut out);
}

impl Buffer {
    pub fn new() -> Buffer {
        Buffer { root: None }
    }

    pub fn insert(&mut self, idx: usize, data: Vec<u32>) {
        let (left, right, _) = split(mem::replace(&mut self.root, None), idx);
        self.root = concat(left, right);
    }

    pub fn delete(&mut self, idx: usize, len: usize) {
        let (left, right, _) = split(mem::replace(&mut self.root, None), idx);
        let (_, remain_right, _) = split(right, len);
        self.root = concat(left, remain_right);
    }

    pub fn report(&self, idx: usize, len: usize) -> Vec<u32> {
        let mut out = Vec::new();
        report(&self.root, idx, len, &mut out);
        out
    }

    pub fn len(&self) -> usize {
        length(&self.root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
