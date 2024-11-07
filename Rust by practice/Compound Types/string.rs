//1🌟
fn main() {
    let s: &str = "hello, world"; 
    println!("Success!");
}
//2🌟
fn main() {
    let s: Box<str> = "hello, world".into();
    greetings(s);
}

fn greetings(s: Box<str>) {
    println!("{}", s);
}

//3🌟
fn main() {
    let mut s = String::new(); 
    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");

    println!("Success!");
}
//4🌟
fn main() {
    let mut s = String::from("hello");
    s.push(',');
    s.push_str(" world");
    s += "!";

    println!("{}", s);
}


//5🌟
fn main() {
    let s = String::from("I like dogs");
    
    let s1 = s.replace("dogs", "cats");

    assert_eq!(s1, "I like cats");

    println!("Success!");
}

//6🌟
fn main() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; 
    assert_eq!(s3, "hello,world!");
    println!("{}", s1); 
}

//7🌟
fn main() {
    let s = "hello, world";
    greetings(s.to_string()); 
}

fn greetings(s: String) {
    println!("{}", s);
}

//8🌟
fn main() {
    let s = "hello, world".to_string();
    let s1: &str = s.as_str(); 

    println!("Success!");
}

//9🌟
fn main() {
    
    let byte_escape = "I'm writing Ru\x73t!";  
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!("Unicode character {} (U+211D) is called {}",
                unicode_codepoint, character_name );

    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here \
                         can be escaped too!";
    println!("{}", long_string);
}

//10🌟
fn main() {
    
    let raw_str = "Escapes don't work here: \x3F \u{211D}";
    assert_eq!(raw_str, "Escapes don't work here: ? ℝ");

    
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    
    let delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", delimiter);

    
    let long_delimiter = r#"Hello, "\"##\"""#; // Escape quotes inside the raw string
    assert_eq!(long_delimiter, "Hello, \"##\"");

    println!("Success!");
}

//11🌟
fn main() {
    let s1 = String::from("hi,中国");
    
    
    let h = s1.chars().next().unwrap(); 
    assert_eq!(h.to_string(), "h"); 

    
    let h1 = &s1[3..6]; 
    assert_eq!(h1, "中"); 

    println!("Success!");
}

//12🌟
fn main() {
    // Fill the blank to print each char in "你好，世界"
    for c in "你好，世界".chars() {
        println!("{}", c);
    }
}
