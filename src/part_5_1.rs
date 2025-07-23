// impl => methods for struct
// trait => like interface


pub fn f1_impl() {
    let mut tom = Person { name: "Tom".to_string(), age: 36};
    let     bob = Person { name: "Bob".to_string(), age: 41};

    bob.display();

    tom.say_hello(&bob, 12);
    bob.say_hello(&tom, 19);

    let is_older = tom.is_older(&bob);
    tom.change_age(22);

    let sam = Person::create("Sam", 12);
}

struct Person
{
    name: String,
    age: u8
}

impl Person {
    fn display(&self){
        println!("Name: {}  Age: {}", &self.name, &self.age);
    }

    fn change_age(&mut self, age: u8){
        self.age = age;
    }

    fn is_older(&self, other: &Person) -> bool{
        self.age > other.age
    }

    fn say_hello(&self, other: &Person, hour: u8){
        if hour < 16{
            println!("Добрый день, {}!", other.name);
        }
        else {
            println!("Добрый вечер, {}!", other.name);
        }
    }

    fn create(user_name: &str, user_age: u8) -> Self{
        Self {
            name: String::from(user_name),
            age: user_age
        }
    }
}



pub fn f2_trait() {
    let tom = Person{ name: String::from("Tom"), age: 36 };
    tom.print();
}

trait Printer{
    fn print(&self){
        println!("Вывод данных на консоль...");
    }
}

impl Printer for Person{
    fn print(&self) {
        println!("Person {}; age: {}", self.name, self.age);
    }
}

struct Message { text: String }
impl Printer for Message { }



pub fn f3_trait2() {
    let tom = Person {name: String::from("Tom"), age: 36 };
    let hello = Message { text: String::from("Hello Rust") };

    display(&tom);
    display(&hello);
}

fn display(printable: &impl Printer) {
    printable.print();
}



pub fn f4_generic() {
    let tom = User {id: 245, name: "Tom".to_string()};
    println!("id: {}  name: {}", tom.id, tom.name);

    let bob = User {id: String::from("fhe34u847"), name: "Bob".to_string()};
    println!("id: {}  name: {}", bob.id, bob.name);

    let some_point = Point { x: 3, y: 9 };
    println!("x={}  y={}", some_point.x, some_point.y);
}

struct User<T> {
    id: T,      // идентификатор
    name: String    // имя
}
struct Point<T> {
    x: T,
    y: T,
}



pub fn f5_overload() {
    let counter1 = Counter{value:5};
    let counter2 = Counter{value:15};
    let mut counter3 = counter1 + counter2;
    println!("{}", counter3.value);  // 20

    counter3 += 103;
    println!("{}", counter3.value);  // 123
}

struct Counter{
    value: u32
}

use std::ops::{Add, AddAssign};
impl Add for Counter{
    type Output = Counter;
    fn add(self, other: Counter) -> Counter {
        Counter {
            value: self.value + other.value
        }
    }
}
impl AddAssign<u32> for Counter{
    fn add_assign(&mut self, other: u32){
        self.value += other;
    }
}
