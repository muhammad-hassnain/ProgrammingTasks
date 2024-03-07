mod queue;
mod stack;
use stack::Stack;
use queue::Queue;

pub struct LuckyDraw {
    pub lane_1: Queue,  // This lane has the winners standing
    pub lane_2: Queue,  // This lane is empty
    pub table_1: Stack, // This table has the gifts stacked up
    pub table_2: Stack, // This is the empty table and the gifts are to be shifted (stacked) here
    pub total_count: i32,  // Store the total number of gifts/winners (assuming both are the same)
    pub special_variable: i32, // Use this variable if you need one
}

impl LuckyDraw {
    // Constructor
    pub fn new(count: i32) -> Self {
        Self {
            lane_1: Queue::new(),
            lane_2: Queue::new(),
            table_1: Stack::new(),
            table_2: Stack::new(),
            total_count: count,
            special_variable: 0, // Assuming an initial value if needed
        }
    }

    pub fn populate_queue(&mut self, winners_id: &[i32]) {
        for &id in winners_id.iter() {
            self.lane_1.enqueue(id);
        }
    }

    // Populates table_1 with the given slice of gifts_id
    pub fn populate_table(&mut self, gifts_id: &[i32]) {
        for &id in gifts_id.iter() {
            self.table_1.push(id);
        }
    }

    /* reverse the queue keeping the following conidtions in mind
    ● You cannot hard code this part i.e your code should work for any number of
    people.
    ● Implement the two lanes as queues.
    ● You are not allowed to use any other data structure except for the given two
    queues.
    ● You are strictly not allowed to even use any variables.
    ● You are only allowed to use the member functions of the Queue */
    pub fn reverse_queue(&mut self) {
        for _ in 0..self.total_count {
            for _ in 0..self.total_count - 1 {
                if let Some(value) = self.lane_1.dequeue() {
                    self.lane_1.enqueue(value);
                }
            }
            if let Some(front_value) = self.lane_1.front() {
                self.lane_2.enqueue(front_value); // Clone or copy the value to enqueue it to lane_2
            }
        }

        while self.lane_1.length() != 0 {
            self.lane_1.dequeue(); // Empty lane_1
        }

        while self.lane_2.length() != 0 {
            if let Some(value) = self.lane_2.dequeue() {
                self.lane_1.enqueue(value); // Move elements back to lane_1 from lane_2
            }
        }
    }

    /* transfer the stack keeping the following conditions in mind
    ● You cannot hard code this part i.e your code should work for any number of
    gifts.
    ● Implement the two tables as two stacks and a helper from the management as
    a variable.
    ● You are not allowed to use any other data structures except for the given two
    stacks and you cannot use more than one variable.
    ● You are only allowed to use member functions of Stack*/
    pub fn transfer_stack(&mut self) {
        
        for _ in 0..self.table_1.length() - 1 {
            self.special_variable = self.table_1.pop().unwrap_or_default();
            while self.table_1.length() != 0 {
                if let Some(top_value) = self.table_1.top() {
                    self.table_2.push(top_value); 
                    self.table_1.pop();
                }
            }
            self.table_1.push(self.special_variable);
            while self.table_2.length() != 0 {
                if let Some(value) = self.table_2.top() {
                    self.table_1.push(value); 
                    self.table_2.pop();
                }
            }
        }
        
        while self.table_1.length() != 0 {
            if let Some(value) = self.table_1.pop() {
                self.table_2.push(value);
            }
        }
    }
}
