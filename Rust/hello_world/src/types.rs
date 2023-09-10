pub fn type_testing() {
    let mut p = Person {
        name: "Joe".to_string(),
        gender: Gender::Male,
        age: 10
    };
    println!("Name: {}, Gender: {:?}, Age: {}", p.name, p.gender, p.age);
    p.age = 34;
    p.print();
    let p1 = Person::create("Jill", Gender::Female, 546);
    p1.print();
    let op = is_nice("Joe");
    match op {
        Some(p2) => p2.print(),
        None => println!("Not cool")
    }
    let v = Thing{
        val: "Hello"
    };
    println!("{}", v.val);
    let b = Person {
        name: "Bob".to_string(),
        gender: Gender::Male,
        age: 45
    };
    b.printt();
    let v = vec![1, 2, 3];
    v.printt()
}

fn is_nice(name: &str) -> Option<Person>{
    if name.to_lowercase() == "joe" {
        Some(Person::create(name, Gender::Male, 24))
    }else {
        None
    }
}

pub struct Person {
    pub name: String,
    pub gender: Gender,
    pub age: i32
}

impl Person {
    fn print(&self){
        println!("Name: {}, Gender: {:?}, Age: {}", self.name, self.gender, self.age);
    }

    fn create(name: &str, gender: Gender, age: i32) -> Person{
        Person { name: name.to_string(), gender: gender, age: age }
    }
}

#[derive(Debug)]
pub enum Gender {
    Male,Female
}

struct Thing<T>{
    val: T
}

trait Printable {
    fn printt(&self);
}

impl Printable for Person {
    fn printt(&self) {
        println!("{}{:?}{}", self.name, self.gender, self.age)
    }
}

impl Printable for Vec<i32> {
    fn printt(&self) {
        println!("{:?}", &self)
    }
}