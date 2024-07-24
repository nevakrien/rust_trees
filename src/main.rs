mod bin_tree;

use bin_tree::Node;

fn main() {
    let mut root = Node::new(1);
    root.set_child(0, Some(Box::new(Node::new(2))));
    root.set_child(1, Some(Box::new(Node::new(3))));

    if let Some(left) = root.get_child(0) {
        assert_eq!(left.value, 2);
        left.set_child(0, Some(Box::new(Node::new(4))));
        left.set_child(1, Some(Box::new(Node::new(5))));
    }

    if let Some(right) = root.get_child(1) {
        assert_eq!(right.value, 3);
        right.set_child(0, Some(Box::new(Node::new(6))));
        right.set_child(1, Some(Box::new(Node::new(7))));
    }

    // Check the tree structure after initial setup
    
    let left = root.get_child(0).unwrap();
    assert_eq!(left.value, 2);
    assert_eq!(left.view_child(0).unwrap().value, 4);
    assert_eq!(left.get_child(1).unwrap().value, 5);

    let right = root.view_child(1).unwrap();
    assert_eq!(right.value, 3);
    assert_eq!(right.view_child(0).unwrap().value, 6);
    assert_eq!(right.view_child(1).unwrap().value, 7);
    

    // Move child and check structure again
    let r = root.pop_child(0).expect("we set a child there...").pop_child(1);
    root.set_child(1, r);

    let right = root.view_child(1).unwrap();
    assert_eq!(right.value, 5);


    println!("All assertions passed!");
}