use std::fmt::Debug;

#[derive(Debug)]
pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T: Debug> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn push_front(&mut self, value: T) {
        self.head = Some(Box::new(Node {
            data: value,
            next: self.head.take(),
        }));
    }

    pub fn push_back(&mut self, value: T) {
        if let Some(ref mut head) = self.head {
            // Walk to the tail
            let mut tail = head;
            while let Some(ref mut next) = tail.next {
                tail = next;
            }
            tail.next = Some(Box::new(Node { data: value, next: None }));
        } else {
            // If the list is empty, just push_front
            self.push_front(value);
        }
    }

    /// Removes the front element and returns it.
    pub fn pop_front(&mut self) -> Option<T> {
        if let Some(mut old_head) = self.head.take() {
            // Move the head pointer to the next node
            self.head = old_head.next.take();
            // Return the data that was in the old head
            return Some(old_head.data);
        }
        None
    }
    
    /// Iterates over the list and prints out each element’s data.
    pub fn print_items(&self) {
        let mut current = &self.head;
        while let Some(node) = current {
            println!("{:?}", node.data);
            current = &node.next;
        }
    }
}

fn main() {
    let mut ll = LinkedList::new();
    ll.push_back(10);
    ll.push_back(20);
    ll.push_back(30);

    ll.print_items();
    println!("Popped front: {:?}", ll.pop_front());
    ll.print_items();
    println!("LinkedList = {:?}", ll);
}
