use std::alloc;
use std::alloc::alloc;
use std::fmt::{Display, write};
use std::ptr;

#[derive(Debug)]
pub struct Node {
    pub data: isize,
    pub next: *mut Node,
}

impl Node {
    fn new(data: isize, pointer: *mut Node) -> Self {
        Self {
            data,
            next: pointer,
        }
    }

    pub fn get(&self) -> Node {
        Self {
            data: self.data,
            next: self.next,
        }
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.next.is_null() {
            write!(f, "{}", self.data)
        } else {
            write!(f, "{}, ", self.data)
        }
    }
}

#[derive(Debug)]
pub struct SimpleLinkedList {
    pub head: *mut Node,
}

impl SimpleLinkedList {
    pub fn new() -> SimpleLinkedList {
        return SimpleLinkedList {
            head: ptr::null_mut(),
        };
    }

    pub fn get_first(&self) -> Option<Node> {
        if self.head.is_null() {
            return None;
        }
        unsafe {
            return Some(self.head.read());
        }
    }

    pub fn push_front(&mut self, data: isize) {
        unsafe {
            let layout = alloc::Layout::new::<Node>();

            let ptr = alloc::alloc(layout) as *mut Node;
            if ptr.is_null() {
                alloc::handle_alloc_error(layout);
            }

            let old_head = self.head;
            ptr.write(Node::new(data, old_head));
            self.head = ptr;
        }
    }

    pub fn push_back(&mut self, data: isize) {
        unsafe {
            let layout = alloc::Layout::new::<Node>();
            let ptr = alloc::alloc(layout) as *mut Node;
            if ptr.is_null() {
                alloc::handle_alloc_error(layout)
            }
            ptr.write(Node::new(data, ptr::null_mut()));

            if self.head.is_null() {
                self.head = ptr;
                return;
            }

            let mut node = self.head;
            while !(*node).next.is_null() {
                node = (*node).next;
            }
            (*node).next = ptr;
        }
    }

    pub fn pop_front(&mut self) -> Option<Node> {
        if self.head.is_null() {
            return None;
        }

        unsafe {
            let node = self.head.read();

            alloc::dealloc(self.head as *mut u8, alloc::Layout::new::<Node>());
            self.head = node.next;
            return Some(node);
        }
    }

    pub fn pop_back(&mut self) -> Option<Node> {
        if self.head.is_null() {
            return None;
        }

        unsafe {
            let mut tmp = self.get_first().unwrap();
            let mut node = self.head;
            let mut length: usize = 1;
            while !(*node).next.is_null() {
                node = (*node).next;
                tmp = (*node).get();
                length += 1;
            }
            
            let mut node = self.head;
            let mut index: usize = 1;
            while index < length - 1 {
                node = (*node).next;
                index += 1;
            }
            let last = (*node).next;
            (*node).next = ptr::null_mut();
            alloc::dealloc(last as *mut u8, alloc::Layout::new::<Node>());
            
            return Some(tmp);
        }
    }
}

impl Drop for SimpleLinkedList {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

impl std::fmt::Display for SimpleLinkedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            let mut node = self.get_first().unwrap();
            write!(f, "[{}", node)?;
            while !node.next.is_null() {
                node = node.next.read();
                write!(f, "{}", node)?;
            }
        }
        write!(f, "]")
    }
}