//🌟1 A variable can be used only if it has been initialized.
// Fix the error below with least amount of modification to the code
fn main() {
    let x: i32=5; // Uninitialized but used, ERROR !
    let y: i32; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, 5);
    println!("Success!");
}

//Variables🌟2 Use mut to mark a variable as mutable.
// Fill the blanks in the code to make it compile
fn main() {
    let mut x = 1;
    x += 2; 
    
    assert_eq!(x, 3);
    println!("Success!");
}

//A scope is the range within the program for which the item is valid.
🌟3
// Fix the error below with least amount of modification
fn main() {
    let x: i32 = 10;
    {
        let y: i32 = 5;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {}", x); 
}

//4🌟🌟
// Fix the error with the use of define_x
fn main() {
    let x = define_x();
   println!("{}, world", x); 
}

fn define_x() -> &'static str {
   let x = "hello";
   x
}

//Shadowing
You can declare a new variable with the same name as a previous variable, here we can say the first one is shadowed by the second one.
🌟🌟5
fn main() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12); 
    }

    assert_eq!(x, 5); 

    let x = 42;
    println!("{}", x); 
}

//6🌟🌟
fn main() {
    let mut x: i32 = 1;
    x = 7;
    
    x += 3;

    let y = 4;
    // Shadowing переменной y
    let y = "I can also be bound to text!"; 

    println!("Success!");
}

//7 🌟🌟Unused variablesFix the warning below with :
fn main() {
    let x = 1;
    println!("{}", x); 
}

//8🌟🌟 
fn main() {
    let (mut x, y) = (1, 2); // Сделать x изменяемым
    x += 2; // Теперь x можно изменить

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}

//9🌟🌟
fn main() {
    let (x, y);
    [x, _] = [3, 4]; // Инициализация x значением 3, а _ игнорирует 4
    [_, y] = [1, 2]; // Инициализация y значением 2, а _ игнорирует 1

    assert_eq!([x, y], [3, 2]); // Сравниваем с массивом [3, 2]

    println!("Success!");
}
