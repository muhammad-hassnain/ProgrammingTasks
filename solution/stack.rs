use LinkedList::LinkedList;

pub struct Stack {
    list: LinkedList,
}

impl Stack {
    // Constructor
    pub fn new() -> Self {
        Stack { list: LinkedList::new() }
    }

    // Push an item onto the stack
    pub fn push(&mut self, item: i32) {
        self.list.insert_head(item);
    }

    // Peek at the top item of the stack without removing it
    // Returns an Option type, None if the stack is empty
    pub fn top(&self) -> Option<i32> {
        self.list.head.as_ref().map(|node| node.data)
    }

    // Pop the top item from the stack
    // Returns an Option type, None if the stack is empty
    pub fn pop(&mut self) -> Option<i32> {
        if self.list.head.is_some() {
            let head_data = self.list.head.as_ref().map(|node| node.data);
            self.list.remove_head();
            head_data
        } else {
            None
        }
    }

    // Get the current length of the stack
    pub fn length(&self) -> i32 {
        self.list.get_length() 
    }

    // Check if the stack is empty
    pub fn is_empty(&self) -> bool {
        if self.list.get_length() == 0{
            return true
        }
        return false;
    }
}
