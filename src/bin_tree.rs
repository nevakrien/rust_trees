

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
			let mut ans = self.children[idx].take()?;
			ans.parent = std::ptr::null_mut();
			return Some(ans);		
	}

	pub fn set_child(&mut self, idx :usize,mut val:Option<Box<Node<T>>>) {
		val.as_mut().map(|node| node.parent=&mut (*self));
		self.children[idx]=val;
	}
}

pub struct Tree<T>{
	root : Option<Box<Node<T>>>,
	handle : *mut Node<T>,//only invalid when root is null
}

impl <T> Tree<T>{
	pub fn pop_root(self) -> Option<Box<Node<T>>>{
		self.root
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