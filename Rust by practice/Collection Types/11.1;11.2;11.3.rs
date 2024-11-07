// ? Collection Types
// * 11.1 String
// ! 1
fn main() {
    let mut s: String = String::from("hello, ");
    s.push_str("world");
    s.push('!');

    move_ownership(s.clone());

    assert_eq!(s, "hello, world!");

    println!("Success!")
}

fn move_ownership(s: String) {
    println!("ownership of \"{}\" is moved here!", s)
}

// ! 2
fn main() {  

    let mut s = String::from("hello, world");
 
    let slice1: &str = &s; 
    assert_eq!(slice1, "hello, world");
 
    let slice2 = &s[0..5];
    assert_eq!(slice2, "hello");
    let slice3: &mut String = &mut s; 
    slice3.push('!');
    assert_eq!(slice3, "hello, world!");
 
    println!("Success!")
 }

// ! 3
fn main() {  
    let s: String = String::from("hello, world!");

    let slice: &str = &s;

    let new_string: String = slice.to_string();

    assert_eq!(new_string, "hello, world!");

    println!("Success!");
}

// ! 4
fn main() {
    let s = String::from("hello, 世界");
    let slice1 = &s[0..1]; 
    assert_eq!(slice1, "h");

    let slice2 = &s[7..10];
    assert_eq!(slice2, "世");

    for (i, c) in s.chars().enumerate() {
        if i == 7 {
            assert_eq!(c, '世')
        }
    }

    println!("Success!")
}

// ! 5
fn main() {
    let mut s = String::new();
    s.push_str("hello");

    let v = vec![104, 101, 108, 108, 111];
    let s1 = String::from_utf8(v).unwrap();
    
    
    assert_eq!(s, s1);

    println!("Success!")
}

// ! 6
fn main() {
    let mut s = String::with_capacity(25);

    println!("{}", s.capacity());

    for _ in 0..2 {
        s.push_str("hello");
        println!("{}", s.capacity());
    }

    println!("Success!")
}

// ! 7
use std::mem;

fn main() {
    let story = String::from("Rust By Practice");

    let mut story = mem::ManuallyDrop::new(story);

    let ptr = story.as_mut_ptr();
    let len = story.len();
    let capacity = story.capacity();

    assert_eq!(16, len);

    let s = unsafe { String::from_raw_parts(ptr, len, capacity) };

    assert_eq!(*story, s);

    println!("Success!")
}

// * 11.2 Vector
// ! 1
fn main() {
    let arr: [u8; 3] = [1, 2, 3];
    
    let v = Vec::from(arr);
    is_vec(&v);

    let v = vec![1, 2, 3];
    is_vec(&v);

    let v = vec!(1, 2, 3);
    is_vec(&v);

    let mut v1 = Vec::new();
    for i in &v {
        v1.push(*i)
    }
    is_vec(&v1);
 
    assert_eq!(format!("{:?}",v), format!("{:?}",v1));

    println!("Success!")
}

fn is_vec(v: &Vec<u8>) {}

// ! 2
fn main() {
    let mut v1 = Vec::from([1, 2, 4]);
    v1.pop();
    v1.push(3);
    
    let mut v2 = Vec::new();
    v2.extend([1, 2, 3]);

    assert_eq!(format!("{:?}",v1), format!("{:?}",v2));

    println!("Success!")
}

// ! 3
fn main() {

    let arr = [1, 2, 3];
    let v1 = Vec::from(arr);
    let v2: Vec<i32> = arr.into();
 
    assert_eq!(v1, v2);
 
    let s = "hello".to_string();
    let v1: Vec<u8> = s.into();

    let s = "hello".to_string();
    let v2 = s.into_bytes();
    assert_eq!(v1, v2);

    let s = "hello";
    let v3 = Vec::from(s);
    assert_eq!(v2, v3);

    println!("Success!")
 }

// ! 4
fn main() {
    let mut v = Vec::from([1, 2, 3]);
    for i in 0..5 {
        println!("{:?}", v.get(i))
    }

    for i in 0..5 {
        if let Some(x) = v.get(i) {
            v[i] = x + 1
        } else {
            v.push(i + 2)
        }
    }
    
    assert_eq!(format!("{:?}",v), format!("{:?}", vec![2, 3, 4, 5, 6]));

    println!("Success!")
}

// ! 5
fn main() {
    let mut v = vec![1, 2, 3];

    let slice1 = &v[..];
    let slice2 = &v[0..3];
    
    assert_eq!(slice1, slice2);

    let vec_ref: &mut Vec<i32> = &mut v;
    (*vec_ref).push(4);
    let slice3 = &mut v[0..4];
    slice3[3] = 42;

    assert_eq!(slice3, &[1, 2, 3, 42]);
    assert_eq!(v, &[1, 2, 3, 42]);

    println!("Success!");
}

// ! 6
fn main() {
    let mut vec = Vec::with_capacity(10);

    
    assert_eq!(vec.len(), 0);
    assert_eq!(vec.capacity(), 10);

    for i in 0..10 {
        vec.push(i);
    }
    assert_eq!(vec.len(), 10);
    assert_eq!(vec.capacity(), 10);

    vec.push(11);
    assert_eq!(vec.len(), 11);
    assert!(vec.capacity() >= 11);


    let mut vec = Vec::with_capacity(100);
    for i in 0..100 {
        vec.push(i);
    }

    assert_eq!(vec.len(), 100);
    assert_eq!(vec.capacity(), 100);

    println!("Success!")
}

// ! 7
#[derive(Debug, PartialEq)]
enum IpAddr {
    V4(String),
    V6(String),
}

fn main() {
    let v: Vec<IpAddr> = vec![
        IpAddr::V4("127.0.0.1".to_string()),
        IpAddr::V6("::1".to_string())
    ];

    assert_eq!(v[0], IpAddr::V4("127.0.0.1".to_string()));
    assert_eq!(v[1], IpAddr::V6("::1".to_string()));

    println!("Success!")
}

// ! 8
trait IpAddr {
    fn display(&self);
}

struct V4(String);
impl IpAddr for V4 {
    fn display(&self) {
        println!("ipv4: {:?}",self.0)
    }
}
struct V6(String);
impl IpAddr for V6 {
    fn display(&self) {
        println!("ipv6: {:?}",self.0)
    }
}

fn main() {
    let v: Vec<Box<dyn IpAddr>> = vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];

    for ip in v {
        ip.display();
    }
}

// * 11.3 HashMap
// ! 1
use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();
    scores.insert("Sunface", 98);
    scores.insert("Daniel", 95);
    scores.insert("Ashley", 69);
    scores.insert("Katie", 58);

    let score = scores.get("Sunface");
    assert_eq!(score, Some(&98));

    if scores.contains_key("Daniel") {
       
        let score = scores["Daniel"];
        assert_eq!(score, 95);
        scores.remove("Daniel");
    }

    assert_eq!(scores.len(), 3);

    for (name, score) in scores {
        println!("The score of {} is {}", name, score)
    }
}

// ! 2
use std::collections::HashMap;
fn main() {
    let teams = [
        ("Chinese Team", 100),
        ("American Team", 10),
        ("France Team", 50),
    ];

    let mut teams_map1 = HashMap::new();
    for team in &teams {
        teams_map1.insert(team.0, team.1);
    }

    let teams_map2: HashMap<_,_> = teams.into_iter().collect();

    assert_eq!(teams_map1, teams_map2);

    println!("Success!")
}

// ! 3
use std::collections::HashMap;

fn main() {
    let mut player_stats = HashMap::new();

    player_stats.entry("health").or_insert(100);

    assert_eq!(player_stats["health"], 100);
    player_stats.entry("health").or_insert_with(random_stat_buff);
    assert_eq!(player_stats["health"], 100);

    let health = player_stats.entry("health").or_insert(50);
    assert_eq!(health, &100);
    *health -= 50;
    assert_eq!(*health, 50);

    println!("Success!")
}

fn random_stat_buff() -> u8 {
    42
}

// ! 4
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Viking {
    name: String,
    country: String,
}

impl Viking {
    fn new(name: &str, country: &str) -> Viking {
        Viking {
            name: name.to_string(),
            country: country.to_string(),
        }
    }
}

fn main() {
    let vikings = HashMap::from([
        (Viking::new("Einar", "Norway"), 25),
        (Viking::new("Olaf", "Denmark"), 24),
        (Viking::new("Harald", "Iceland"), 12),
    ]);

    for (viking, health) in &vikings {
        println!("{:?} has {} hp", viking, health);
    }
}

// ! 5
use std::collections::HashMap;

fn main() {
    let v1 = 10;
    let mut m1 = HashMap::new();
    m1.insert(v1, v1);
    println!("v1 is still usable after inserting to hashmap : {}", v1);

    let v2 = "hello";
    let mut m2 = HashMap::new();
    m2.insert(v2, v1);

    assert_eq!(v2, "hello");
}