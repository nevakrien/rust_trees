

#[derive(Debug)]
pub struct Node<T> {
    pub value: T,
    children: [Option<Box<Node<T>>>; 2],
    parent: *mut Node<T>, 
}

impl <T> Node<T>{
	pub fn new(value: T) -> Self {
        Node {
            value,
            children: [None, None],
            parent: std::ptr::null_mut(),
        }
    }
    pub fn view_child(& self, idx :usize) -> Option<& Node<T>> {
		self.children[idx].as_deref()
	}

	pub fn get_child(&mut self, idx :usize) -> Option<&mut Node<T>> {
		self.children[idx].as_deref_mut()
	}

	pub fn pop_child(&mut self, idx :usize) -> Option<Box<Node<T>>>{
			let ans = self.children[idx].take()?;
			//ans.parent = std::ptr::null_mut();
			//no need because Tree does not assume parent makes sense when its given externally
			return Some(ans);		
	}

	pub fn set_child(&mut self, idx :usize,mut val:Option<Box<Node<T>>>) {
		val.as_mut().map(|node| node.parent=&mut (*self));
		self.children[idx]=val;
	}
}

#[cfg(test)]
mod node_tests {
    use super::*;

    #[test]
    fn test_node_structure_initial_setup() {
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

        let left = root.get_child(0).unwrap();
        assert_eq!(left.value, 2);
        assert_eq!(left.view_child(0).unwrap().value, 4);
        assert_eq!(left.get_child(1).unwrap().value, 5);

        let right = root.view_child(1).unwrap();
        assert_eq!(right.value, 3);
        assert_eq!(right.view_child(0).unwrap().value, 6);
        assert_eq!(right.view_child(1).unwrap().value, 7);
    }

    #[test]
    fn test_node_move_child() {
        let mut root = Node::new(1);
        root.set_child(0, Some(Box::new(Node::new(2))));
        root.set_child(1, Some(Box::new(Node::new(3))));

        if let Some(left) = root.get_child(0) {
            left.set_child(0, Some(Box::new(Node::new(4))));
            left.set_child(1, Some(Box::new(Node::new(5))));
        }

        let r = root.pop_child(0).expect("we set a child there...").pop_child(1);
        root.set_child(1, r);

        let right = root.view_child(1).unwrap();
        assert_eq!(right.value, 5);
    }
}

pub struct Tree<T>{
	root : Option<Box<Node<T>>>, 
	//root being None means an empty tree
	//importatnly we are not assuming that roots parent pointer is null
	//it can be garbage.
	//any parent pointer not equal to root is a non null valid pointer
	handle : *mut Node<T>,//only invalid when root is null
}

impl <T> Tree<T>{
	pub fn reset_handle(&mut self) -> &mut Tree<T>{
		self.handle = match &self.root {
            Some(node) => &**node as *const Node<T> as *mut Node<T>,
            None => std::ptr::null_mut(),
        };
        return self;
	} 
	pub fn new(root: Option<Box<Node<T>>>) -> Self{
		let mut ans = Tree{
			root ,
			handle: std::ptr::null_mut(),
		};
		ans.reset_handle();
		return ans;
	}

	pub fn pop_root(self) -> Option<Box<Node<T>>>{
		self.root
	}

	pub fn empty(&self) -> bool {
		match self.root.as_deref(){
			Some(_root) => false,
			None => true
		}
	}

	pub fn parent(&mut self) -> Option<&mut Tree<T>> {
		let root = self.root.as_deref_mut()?;
		if (root as *mut Node<T>) == self.handle {
			return None;
		}
		//we now know that handle is not null
		//and we know it has a parent
		unsafe{
			let node = &*self.handle;
			self.handle=node.parent;
		}

		return Some(self);
	}

	pub fn child(&mut self,idx :usize) -> Option<&mut Tree<T>> {
		let _root = self.root.as_deref_mut()?;
		
		//we now know that handle is not null
		
		let node = unsafe{&mut *self.handle};
		self.handle = node.get_child(idx)?;
		return Some(self);
	}

	pub fn set_child(&mut self,idx :usize, val:Option<Box<Node<T>>>) -> Option<&mut Tree<T>> {
		let _root = self.root.as_deref_mut()?;
		
		//we now know that handle is not null
		
		let node = unsafe{&mut *self.handle};
		node.set_child(idx,val);
		return Some(self);
	}

	pub fn pop_handle(&mut self) -> Option<Box<Node<T>>>{
		let root = self.root.as_deref_mut()?;
		if (root as *mut Node<T>) == self.handle {
			return self.root.take();
		}

		let comp = self.handle;

		//we now know that handle is not null
		//and we know it has a parent
		let mut node : &mut Node<T>;
		unsafe{
			node = &mut *self.handle;
			self.handle=node.parent;
			node = &mut *self.handle;
		}

		for child in &mut node.children {
			if let Some(ref mut boxed_node) = child {
                let node_ptr: *mut Node<T> = &mut **boxed_node;
                if node_ptr == comp {
                    // no need to set the parent_pointer
                    // since it would be set when we move it to a node
                    // and in Tree we are not assuming root has a valid pointer
                    return child.take();
                }
            }
		}

		unreachable!("node not in its parent...");
	} 
}

impl<T: Copy> Tree<T> {
    pub fn view_value(&mut self) -> Option<T> {
        let _root = self.root.as_deref_mut()?;
        let node = unsafe { &mut *self.handle };
        Some(node.value)
    }
}

#[cfg(test)]
mod tree_tests {
    use super::*;

    #[test]
    fn test_tree() {
        let mut tree_data = Tree::new(Some(Box::new(Node::new(1))));
        let mut tree = &mut tree_data;

        tree = tree.set_child(0, Some(Box::new(Node::new(2)))).expect("failed set");
        tree = tree.set_child(1, Some(Box::new(Node::new(3)))).expect("failed set");
        tree = tree.child(0).expect("failed navigating");

        assert_eq!(2, tree.pop_handle().unwrap().value);
        assert_eq!(1, tree.pop_handle().unwrap().value);
        assert!(tree_data.pop_root().is_none()); // this is owned we can't check anything anymore
    }

    #[test]
    fn test_empty() {
        let mut tree: Tree<i32> = Tree::new(None);
        assert!(tree.empty());

        tree = Tree::new(Some(Box::new(Node::new(10))));
        assert!(!tree.empty());
    }

    #[test]
    fn test_pop_root() {
        let tree_data = Tree::new(Some(Box::new(Node::new(5))));
        assert_eq!(tree_data.pop_root().unwrap().value, 5);
    }

    #[test]
    fn test_view_value() {
        let mut tree_data = Tree::new(Some(Box::new(Node::new(1))));
        let mut tree = &mut tree_data;

        tree = tree.set_child(0, Some(Box::new(Node::new(2)))).expect("failed set");
        tree = tree.set_child(1, Some(Box::new(Node::new(3)))).expect("failed set");

        tree = tree.child(0).expect("failed navigating");
        assert_eq!(tree.view_value().unwrap(), 2);

        tree = tree.parent().expect("failed navigating to parent");
        assert_eq!(tree.view_value().unwrap(), 1);

        tree = tree.child(1).expect("failed navigating");
        assert_eq!(tree.view_value().unwrap(), 3);
    }

    #[test]
    fn test_set_and_view_value() {
        let mut tree_data = Tree::new(Some(Box::new(Node::new(1))));
        let mut tree = &mut tree_data;

        tree = tree.set_child(0, Some(Box::new(Node::new(2)))).expect("failed set");
        tree = tree.set_child(1, Some(Box::new(Node::new(3)))).expect("failed set");

        tree = tree.child(0).expect("failed navigating");
        assert_eq!(tree.view_value().unwrap(), 2);

        tree = tree.parent().expect("failed navigating to parent");
        assert_eq!(tree.view_value().unwrap(), 1);

        tree = tree.child(1).expect("failed navigating");
        assert_eq!(tree.view_value().unwrap(), 3);
    }
}
