use LinkedList::LinkedList;

pub struct Queue {
    pub list: LinkedList,
}

impl Queue {
    // Constructor
    pub fn new() -> Self {
        Queue { list: LinkedList::new() }
    }

    // Enqueue: Adds an element to the end of the queue
    pub fn enqueue(&mut self, data: i32) {
        self.list.insert_tail(data);
    }

    // Dequeue: Removes and returns the front element of the queue
    pub fn dequeue(&mut self) -> Option<i32> {
        if let Some(head) = self.list.head.as_ref() {
            let data = head.data;
            self.list.remove_head();
            Some(data)
        } else {
            None
        }
    }

    // Front: Returns the front element of the queue without removing it
    pub fn front(&self) -> Option<i32> {
        self.list.head.as_ref().map(|node| node.data)
    }

    // Length: Returns the number of elements in the queue
    pub fn length(&self) -> i32 {
        self.list.get_length()
    }

    // IsEmpty: Checks if the queue is empty
    pub fn is_empty(&self) -> bool {
        self.list.head.is_none()
    }
}
