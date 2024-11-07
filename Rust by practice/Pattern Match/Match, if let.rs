//1🌟
enum Direction {
    East,
    West,
    North,
    South,
}

fn main() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::South | Direction::North => { 
            println!("South or North");
        },
        _ => println!("Other direction"), 
    };
}

//2🌟
fn main() {
    let boolean = true;

    
    let binary = match boolean {
        true => 1,
        false => 0,
    };

    assert_eq!(binary, 1);

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
    let msgs = [
        Message::Quit,
        Message::Move { x: 1, y: 3 },
        Message::ChangeColor(255, 255, 0),
    ];

    for msg in msgs {
        show_message(msg);
    }

    println!("Success!");
}

fn show_message(msg: Message) {
    match msg {
        Message::Move { x, y } => { 
            assert_eq!(x, 1);
            assert_eq!(y, 3);
        },
        Message::ChangeColor(_, g, b) => {
            assert_eq!(g, 255);
            assert_eq!(b, 0);
        },
        _ => println!("no data in these variants"), 
    }
}

//4🌟
fn main() {
    let alphabets = ['a', 'E', 'Z', '0', 'x', '9' , 'Y'];

    
    for ab in alphabets {
        assert!(matches!(ab, 'A'..='Z'));
    }

    println!("Success!");
}

//5🌟
#[derive(PartialEq)] 
enum MyEnum {
    Foo,
    Bar
}

fn main() {
    let mut count = 0;

    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    for e in v {
        if e == MyEnum::Foo { 
            count += 1;
        }
    }

    assert_eq!(count, 2);

    println!("Success!");
}

//6🌟
fn main() {
    let o = Some(7);

    
    if let Some(i) = o {
        println!("This is a really long string and `{:?}`", i);
        println!("Success!");
    }
}

//7🌟
enum Foo {
    Bar(u8)
}

fn main() {
    let a = Foo::Bar(1);

    match a {
        Foo::Bar(i) => {
            println!("foobar holds the value: {}", i);

            println!("Success!");
        }
    }
}

//8🌟
enum Foo {
    Bar,
    Baz,
    Qux(u32),
}

fn main() {
    let a = Foo::Qux(10);

    match a {
        Foo::Bar => println!("match foo::bar"),
        Foo::Baz => println!("match foo::baz"),
        Foo::Qux(val) => println!("match others: Qux with value {}", val),
    }
}

//9🌟
fn main() {
    let age = Some(30);
    
    if let Some(age) = age { 
       assert_eq!(age, 30); 
    }
    
    let age = Some(30); 

    match age {
        
        Some(age) =>  println!("age is a new variable, it's value is {}", age),
        _ => ()
    }
}
