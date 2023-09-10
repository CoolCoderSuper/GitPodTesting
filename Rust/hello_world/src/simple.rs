pub fn basic() {
    println!("Basic Testing");
    println!("Hello, world!");
    /*println!("Enter name: ");
    let mut name = "".to_string();
    let res = std::io::stdin().read_line(&mut name);
    println!("Hello, {name}");*/
    let last_name = " Rogan";
    let name = "Joe".to_string() + last_name;
    let other_name = format!("Joe {last_name}");
    println!("Hello, {} {name}", "Bob");
    println!("{other_name}");
    let words: Vec<&str> = name.split_whitespace().collect();
    println!("{}", words[0]);
    for token in name.split_whitespace(){
        println!("{token}");
    }
    if name.contains("Joe") {
        println!("Contains 'Joe'")
    }else if name.contains("Rogan") {
        println!("Contains 'Rogan'")
    }else {
        println!("Does not contain 'Joe'")
    }
    let num = 3;
    match num {
        1 => println!("Num is 1"),
        2 => println!("Num is 2"),
        3 => println!("Num is 3"),
        _ => println!("None")
    }
    let res = match num {
        1 => 4,
        2 => 5,
        3 => 6,
        _ => 0
    };
    println!("{res}");
    for i in 1..21{
        match i {
            6 => continue,
            17 => break,
            13 => println!("Wow"),
            _ => println!("{i}")
        }
    }
    let mut x = 1;
    while x <= 10 {
        println!("{x}");
        x += 1;
    }
    let mut y = 1;
    loop {
        if y > 10 {
            break;
        }
        println!("{y}");
        y += 1;
    }
    hello("Joe");
    println!("{}", add(1, 2));
    println!("{}", subtract(1, 2));
    let mut add_res: i32 = 0;
    add_ref(1, 2, &mut add_res);
    println!("{add_res}");
    let t = ("Bob", 324, 234.234234);
    println!("{:?}", t);
    println!("{}, {}, {}", t.0, t.1, t.2);
    let (str, j, f) = t;
    println!("Str: {str}, j: {j}, f: {f}");
    let names = ["Bob", "Joe", "Jack"];
    println!("{:?}, {}", names, names.len());
    println!("{}", names[2]);
    for i in 0..names.len(){
        println!("{i}, {}", names[i])
    }
    for v in names.iter(){
        println!("{v}")
    }
    let mut other_names = ["Bill", "Rod"];
    println!("{:?}", other_names);
    other_names[0] = "Will";
    println!("{:?}", other_names);
    let h = "Hello".to_string();
    let s  = &h[2..4];
    println!("h: '{h}', s: '{s}'");
    let v = vec![1, 2, 3];
    let mut i  = v.iter();
    println!("{:?}", i);
    println!("{:?}", i.next());
    let mut i1 = v.iter();
    loop {
        let o = i1.next();
        match o {
            Some(d) => println!("{d}"),
            None => break
        }
    }
    let f = 2;
    let increment = |n: i32| -> i32{
        (n + 1) * f
    };
    println!("{}", increment(1))
}

fn hello(name: &str){
    println!("Hello, {name}")
}

fn add(num1: i32, num2: i32) -> i32{
    return num1 + num2
}

fn subtract(num1: i32, num2: i32) -> i32{
    num1 - num2
}

fn add_ref(num1: i32, num2: i32, val: &mut i32) {
    *val = add(num1, num2)
}

pub fn ownership() {
    println!("Ownership Testing");
    let mut num = 1;//primitive types not owned
    println!("{num}");
    let num2 = num;
    num = 2;
    println!("{num}");
    println!("{num2}");
    let a = vec![1, 2];
    println!("{:?}", a);
    let a1 = a;
    //println!("{:?}", a); //gives an error
    println!("{:?}", a1);
    let a2 = vec_print(a1);
    //println!("{:?}", a1); //gives an error
    println!("{:?}", a2);
    let w = vec![2, 3];
    let w1 = w.clone();
    println!("{:?}", w);
    println!("{:?}", w1);
}

fn vec_print(v: Vec<i32>) -> Vec<i32>{
    println!("print: {:?}", v);
    v
}

pub fn borrowing() {
    let o = vec![1, 2];
    vec_borrow_print(&o);
    println!("{:?}", o);
    let mut f = vec![2, 3];
    vec_borrow_print(&f);
    vec_mut_borrow(&mut f);
    vec_borrow_print(&f);
}

fn vec_mut_borrow(v: &mut Vec<i32>){
    v.reverse()
}

fn vec_borrow_print(v: &Vec<i32>){
    println!("Print: {:?}", v)
}