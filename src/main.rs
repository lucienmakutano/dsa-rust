use crate::list::linked_list::{List, Node};
use crate::tree::{
    balanced_tree::print_bn as print_bal,
    binary_tree::print_bn,
};

mod list;
mod tree;

fn main() {
    let mut list = List::new();

    println!("Is empty: {}", list.is_empty());

    (1..10).for_each(|value| {
        let n = Node { value, next: None };
        list.insert(Box::new(n));
    });

    println!("Is empty: {}", list.is_empty());
    list.append(Box::new(Node { value: 100, next: None }));

    println!("Found search key {}: {:?}", 18, list.search(18));
    println!("{:?}", list);

    print_bn();
    print_bal();
}
