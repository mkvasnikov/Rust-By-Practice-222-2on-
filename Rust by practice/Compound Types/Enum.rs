//1🌟
// Fixed enums
enum Number {
    Zero,
    One,
    Two,
}

enum Number1 {
    Zero = 0,
    One,
    Two,
}

// C-like enum with floating-point values
enum Number2 {
    Zero = 0.0,
    One = 1.0,
    Two = 2.0,
}

fn main() {
    // Convert enum variants to integers for comparison
    assert_eq!(Number::One as i32, Number1::One as i32); // Convert to integers for comparison
    assert_eq!(Number1::One as i32, Number2::One as i32); // Convert to integers for comparison

    println!("Success!");
}

//2🌟
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg1 = Message::Move { x: 1, y: 2 }; // Instantiating with x = 1, y = 2
    let msg2 = Message::Write(String::from("hello, world!")); // Instantiating with "hello, world!"

    println!("Success!");
}

//3🌟
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::Move { x: 1, y: 2 };

    if let Message::Move { x: a, y: b } = msg {
        assert_eq!(a, 1);
        assert_eq!(b, 2);
    } else {
        panic!("NEVER LET THIS RUN!");
    }

    println!("Success!");
}

//4🌟
use std::fmt;

#[derive(Debug)] // Derive Debug trait to enable printing
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msgs: [Message; 3] = [
        Message::Quit,
        Message::Move { x: 1, y: 3 },
        Message::ChangeColor(255, 255, 0),
    ];

    for msg in msgs {
        show_message(msg);
    }
}

fn show_message(msg: Message) {
    println!("{:?}", msg); // Use {:?} to print debug representation
}

//5🌟
fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    if let Some(n) = six { // Check if `six` is Some(n)
        println!("{}", n);

        println!("Success!");
    } 
        
    // Panic is not reached if `six` is Some(n) because we've already handled it
    panic!("NEVER LET THIS RUN！"); // This will not run if six is Some
} 

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,               // If None, return None
        Some(i) => Some(i + 1),     // If Some(i), return Some(i + 1)
    }
}

//6🌟
use crate::List::*; // Import the enum variants

enum List {
    // Cons: Tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<List>),
    // Nil: A node that signifies the end of the linked list
    Nil,
}

// Methods can be attached to an enum
impl List {
    // Create an empty list
    fn new() -> List {
        // `Nil` has type `List`
        Nil
    }

    // Consume a list, and return the same list with a new element at its front
    fn prepend(self, elem: u32) -> List {
        // `Cons` also has type List
        Cons(elem, Box::new(self))
    }

    // Return the length of the list
    fn len(&self) -> u32 {
        // `self` has to be matched, because the behavior of this method
        // depends on the variant of `self`
        match *self {
            // If it's a `Cons` node, return 1 + length of the tail
            Cons(_, ref tail) => 1 + tail.len(),
            // If it's `Nil`, the length is 0
            Nil => 0,
        }
    }

    // Return representation of the list as a (heap allocated) string
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // `format!` is similar to `print!`, but returns a heap
                // allocated string instead of printing to the console
                format!("{}, {}", head, tail.stringify()) // Call `stringify` on the tail
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

fn main() {
    // Create an empty linked list
    let mut list = List::new();

    // Prepend some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // Show the final state of the list
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
