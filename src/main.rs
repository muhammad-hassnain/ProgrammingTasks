use LinkedList::LinkedList;
mod stack;
mod queue;
// mod luckydraw;

use stack::Stack;
use queue::Queue;
// use luckydraw::LuckyDraw;

use std::env;




fn main(){
    let mut q = Queue::new();
    q.enqueue(2);
    q.enqueue(5);
    println!("{:?}" , q.front());

    q.dequeue();
    println!("{:?}" , q.front());

    println!("The length of queue {}" , q.length());

    q.dequeue();
    println!("The length of queue {}" , q.length());

    let mut s = Stack::new();
    s.push(1);
    s.push(2);
    println!("{:?}" , s.top());

    s.pop();
    println!("{:?}" , s.top());

    println!("The length of stack {}" , s.length());

    s.pop();
    println!("The length of stack {}" , s.length());

    /*let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Please specify the number of winners/gifts!");
        std::process::exit(1);
    }

    let count = args[1].parse::<i32>().expect("Please provide a valid number");

    let arr: Vec<i32> = (0..count as i32).collect();
    let mut flag = false;
    let mut score = 0;

    let mut test = LuckyDraw::new(count);
    test.populate_queue(&arr);
    test.populate_table(&arr);
    test.reverse_queue();
    test.transfer_stack();

    if test.lane_1.length() != count || test.lane_2.length() != 0 {
        flag = true;
        println!("Queue rearrangement failed! ...  0/15");
    } else {
        println!("1");
    }

    if !flag {
        for i in (0..count as i32).rev() {
            if test.lane_1.dequeue() != Some(i) {
                println!("Queue rearrangement failed! ...  0/15");
                flag = true;
                break;
            }
        }
        println!("3");
    }

    if !flag {
        println!("Queue rearrangement passed! ...  15/15");
        score += 15;
    }

    flag = false;

    if test.table_2.length() != count || test.table_1.length() != 0 {
        println!("Stack transfer failed! ...   0/15");
        flag = true;
    }

    if !flag {
        println!("188");
        for i in (0..count as i32).rev() {
            println!("{:?} {:?}" , test.table_2.top() , Some(i));
            if test.table_2.pop() != Some(i) {
                println!("Stack transfer failed! ...   0/15");
                flag = true;
                break;
            }
        }
    }

    if !flag {
        println!("Stack transfer passed! ...   15/15");
        score += 15;
    }

    println!("Total Score: {}/30", score);*/


}