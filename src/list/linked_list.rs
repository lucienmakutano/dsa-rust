use std::option::Option::{self, None, Some};

#[derive(Debug)]
pub struct Node {
    pub value: i8,
    pub next: Option<Box<Node>>,
}

#[derive(Debug)]
pub struct List {
    pub head: Option<Box<Node>>,
}

impl List {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn is_empty(&self) -> bool {
        return match self.head {
            None => true,
            Some(_) => false
        };
    }

    pub fn insert(&mut self, n: Box<Node>) {
        match self.head {
            None => {
                self.head = Some(n);
                return;
            }
            Some(_) => {}
        }

        let mut current_node = &mut self.head;

        while let Some(x) = current_node {
            match x.next {
                None => {
                    x.next = Some(n);
                    break;
                }
                Some(_) => current_node = &mut x.next
            }
        }
    }

    pub fn search(&self, key: i8) -> bool {
        let mut current_node = &self.head;

        while let Some(x) = current_node {
            if x.value == key {
                return true;
            } else {
                current_node = &x.next;
            }
        }

        return false;
    }

    pub fn append(&mut self, node: Box<Node>) {
        let mut current_node = &mut self.head;
        while let Some(x) = current_node {
            match x.next {
                None => {
                    x.next = Some(node);
                    break;
                }
                Some(_) => {
                    current_node = &mut x.next;
                }
            }
        }
    }
}
