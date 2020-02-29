use std::fmt;

fn intersect(argc: i32) {
    match argc {
        i if i < 0 => println!(" < 0"),
        i if i < 10 => println!(" < 10"),
        _ => println!("default case .")
    }
}

struct Point {
    x: f64,
    y: f64,
}
fn test_match1() {
    let p = Point{x:1.0, y:2.0};
    match p {
        Point{x:x1,y:y1} => println!("{} {}.",x1,y1),
    }
}

fn test_match2() {
    let p = Point{x:1.0, y:2.0};
    match p {
        Point{y, ..} => println!("{} .",y),
    }
}

fn test_match_num1 () {
    let x = 33;
    match x {
        1 ..= 10 => println!("1 - 10"),
        _ => println!("default case."),
    }
}

fn test_match_num2 (x:i32) {
    match x {
        e@ 1 ..= 10 | e@ 20 ..= 30 => println!("num is {}",e),
        _ => println!("default case."),
    }
}

#[derive(Debug)]
struct Person {
    name: Option<String>,
}
fn test_match3() {
    let name = "steve".to_string();
    let person = Some(Person{name:Some(name)});
    match person {
        Some(Person{name: ref a @ Some(_),..}) => println!("{:?}",a),
        _ => println!("default.")
    }
}

fn test_match_if() {
    let f = false;
    let x = 33;
    match x {
        4|5 if f => println!("4 or 5"),
        _ => println!("no"),
    }
}

fn test_match4() {
    let mut s : Option<String>= Some("hello ".into());
    match &mut s {
        Some(i) => i.push_str("world."),
        None => println!("none."),
    }
    println!("{:?}",s);
}

struct T {
    item1: char,
    item2: bool,
}
fn test_match5(T{item1:arg1,item2:arg2}:T){
    println!("item1:{},item2:{}",arg1,arg2);
}

fn main() {
    let x = 24;
    match x {
        i if x > 5 => println!("x > 5,{}",i),
        i if x <= 5 => println!("x <= 5,{}",i),
        _ => unreachable!(),
    }

    intersect(1);

    test_match1();
    test_match2();

    test_match_num1 ();

    test_match_num2(5);
    test_match_num2(15);
    test_match_num2(25);
    test_match3();
    test_match_if();

    test_match4();

    let t:T = T {
        item1: 'c',
        item2: true,
    };
    test_match5(t);
}
