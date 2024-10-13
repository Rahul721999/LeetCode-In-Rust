
fn main() {
    let mut cache = LRUCache::new(2); // new LRUCache(2);
    cache.put(1, 1); // cache is {1=1}
    cache.put(2, 2); // cache is {1=1, 2=2}
    assert_eq!(cache.get(1), 1); // return 1
    cache.put(3, 3); // LRU key was 2, evicts key 2, cache is {1=1, 3=3}``
    assert_eq!(cache.get(2), -1); // returns -1 (not found)
    cache.put(4, 4); // LRU key was 1, evicts key 1, cache is {4=4, 3=3}
    assert_eq!(cache.get(1), -1); // return -1 (not found)
    assert_eq!(cache.get(3), 3); // return 3
    assert_eq!(cache.get(4), 4); // return 4
}

use std::{cell::RefCell, collections::HashMap, rc::Rc};
type T = Option<Rc<RefCell<ListNode>>>;
#[derive(Debug, PartialEq, Eq, Clone)]
struct ListNode {
    key: i32,
    value: i32,
    prev: T,
    next: T,
}
impl ListNode {
    fn new(key: i32, value: i32) -> Self {
        Self {
            key,
            value,
            prev: None,
            next: None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct DoublyLinkedList {
    head: T, // Most Recent Use
    tail: T, // Least Recent Use
}

impl DoublyLinkedList {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    // fn to move a node to the head
    fn move_to_head(&mut self, node: Rc<RefCell<ListNode>>) {
        // If node is already at the head, do nothing
        if let Some(ref curr_head) = self.head {
            if Rc::ptr_eq(curr_head, &node) {
                return;
            }
        }

        // Step1: Disconnect the node from it's prev node
        let mut node_borrow = node.borrow_mut();
        if let Some(ref prev_node) = node_borrow.prev {
            prev_node.borrow_mut().next = node_borrow.next.clone();
        }

        if let Some(ref next_node) = node_borrow.next {
            next_node.borrow_mut().prev = node_borrow.prev.clone();
        }

        // if the node was the tail node update the tail

        if let Some(ref curr_tail) = self.tail {
            if Rc::ptr_eq(&node, curr_tail) {
                self.tail = node_borrow.prev.clone();
            }
        }

        // Step2: Insert the node at the head
        node_borrow.prev = None;
        node_borrow.next = self.head.clone();
        drop(node_borrow); // drop the mutable ref

        // Update head and prev of existing head (if any)
        if let Some(ref curr_head) = self.head {
            curr_head.borrow_mut().prev = Some(node.clone());
        }
        self.head = Some(node);
    }

    // fn to insert a new node to the given list
    fn insert_to_head(&mut self, node: Rc<RefCell<ListNode>>) {
        // if there's any head node in the list
        if let Some(ref curr_head) = self.head {
            curr_head.borrow_mut().prev = Some(node.clone());
            node.borrow_mut().next = Some(curr_head.clone());
        } else {
            // if the list is empty
            self.tail = Some(node.clone());
        }
        self.head = Some(node);
    }

    // fn to remove the tail node
    fn remove_tail(&mut self) -> Option<Rc<RefCell<ListNode>>> {
        if let Some(tail_node) = self.tail.take() {
            // check if there's any prev node
            if let Some(prev_node) = tail_node.borrow_mut().prev.take() {
                prev_node.borrow_mut().next = None;
                self.tail = Some(prev_node);
            } else {
                // if there's no prev node means the list is now empty
                self.head = None;
            }
            return Some(tail_node);
        }
        None
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct LRUCache {
    capacity: usize,
    map: HashMap<i32, Rc<RefCell<ListNode>>>,
    cache: DoublyLinkedList,
}
impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            capacity: capacity as usize,
            map: HashMap::new(),
            cache: DoublyLinkedList::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.map.get(&key) {
            self.cache.move_to_head(node.clone()); // mark as Most Recent Use
            return node.borrow().value;
        }
        return -1; // Return -1 if the key is not found
    }

    fn put(&mut self, key: i32, value: i32) {
        // if the key already exists
        if let Some(node) = self.map.get(&key) {
            node.borrow_mut().value = value; // update the value
            self.cache.move_to_head(node.clone()); // move the key to Most Recent Use item,
        } else {
            // if the key doesn't exists
            let new_node = Rc::new(RefCell::new(ListNode::new(key, value)));
            self.cache.insert_to_head(new_node.clone());
            self.map.insert(key, new_node.clone()); // add it to the map

            // if the cache exceeds it's capacity, remove the Least Recently Used Item
            if self.map.len() > self.capacity {
                if let Some(lru_node) = self.cache.remove_tail() {
                    self.map.remove(&lru_node.borrow().key);
                }
            }
        }
    }
}
