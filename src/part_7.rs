use std::collections::*;




pub fn f1_vec() {
    let mut vec: Vec<i32> = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    let _ = vec![7.7; 10];
    let _ = vec![1, 2, 3];
}

pub fn f2_vec() {
    let mut users = vec!["Tom", "Sam", "Bob"];
    println!("{:?}", users);

    users[1] = "Alice";
    users.push("Tod");
    println!("{:?}", users);

    let last = users.pop().unwrap();
    println!("{:?}", last);

    let mid = users.get(1);
    match mid {
        Some(user) => println!("The second user is {}", user),
        None => println!("User not found")
    }


    let users = vec!["Tom", "Sam", "Bob"];
    for user in &users{
        println!("{}", user);
    }
    println!("length: {}", users.len());
}

enum Phones {
    Number(u64),
    Text(String),
}

pub fn f3_vec() {
    let phones = vec![
        Phones::Number(19876542310),
        Phones::Text(String::from("1-987-654-3210"))
    ];

    for phone in &phones {
        match phone {
            Phones::Number(n) => println!("{n}"),
            Phones::Text(s) => println!("{s}"),
        }
    }
}




pub fn f4_string() {
    let message = String::new();
    let message = String::from("Hello");
    let message = "Hello".to_string();

    let length = message.chars().count();
    println!("length: {}", length);  // 5


    let sentence = String::from("Hello Rust! Hello METANIT.COM!");
    let count = sentence.matches("Hello").count();

    println!("{}", count);  // 2
}

pub fn f5_string() {
    let mut msg = String::from("Hello,");
    msg.push_str(" Rust");
    msg.push('?');
    println!("Message: {}", msg);   // Message: hello Rust


    let hello = String::from("Hello, ");
    let rust = String::from("Rust!");
    let message1 = hello.clone() + &rust;
    let message2 = format!("{}{} on Metanit.com", hello, rust);
    println!("Message: {} | {}", message1, message2);


    let sentence = String::from("Hello METANIT.COM");
    let first_word = &sentence[0..5]; // Берем первые 5 символов
    println!("{}", first_word); // Hello
}




pub fn f6_hashmap() {
    let mut people = HashMap::from([
        ("Alice", 35),
        ("Tom", 39)
    ]);

    people.remove("Alice");
    people.clear();

    println!("{:?}", people);
}

pub fn f7_hashset() {
    let my_set = HashSet::from([1, 2, 3, 4, 5, 3, 2, 4]);
    println!("my_set: {:?}", my_set);     // my_set: {4, 3, 1, 5, 2}

    let mut numbers = HashSet::from([1, 2, 3, 4]);
    numbers.remove(&2);
    println!("{:?}", numbers);     // {3, 1, 4}

    let users = HashSet::from(["Tom", "Bob", "Alice"]);
    for user in &users{
        println!("{}", user);
    }

    let users2 = HashSet::from(["Sam", "Kate", "Bob"]);
    let users3: HashSet<&_> = users.union(&users2).collect();

    println!("{:?}", users3);  // {"Sam", "Bob", "Alice", "Kate", "Tom"}
}




pub fn f8_iterator() {
    let prime_sequence = PrimeGenerator { current: 1 };
    for prime in prime_sequence.take(10) {
        print!("{} ", prime);
    }   println!();


    let mut people = vec![
        Person{ name:"Tom".to_string(),   age: 38},
        Person{ name:"Kate".to_string(),  age: 31},
        Person{ name:"Bob".to_string(),   age: 42},
        Person{ name:"Alice".to_string(), age: 34},
        Person{ name:"Sam".to_string(),   age: 25}
    ];
    // // 1
    // let view: Vec<Person> = people.into_iter().filter(|p| p.age > 33).collect();
    // let view: Vec<String> = people.into_iter().map(|p| p.name).collect();
    // for person in view {
    //     println!("{:?}", person);
    // }

    // // 2
    // people.into_iter().for_each(|p| println!("{:?}", p));

    // // 3
    // let view = people.into_iter().filter(|p| p.age > 33);
    // let view = view.map(|p| p.name);
    // let view = view.skip(1);
    // let view = view.take(3);
    // let result: Vec<String> = view.collect();   // отложенное выполнение
    // for person in result{
    //     println!("{}", person);
    // }

    // // 4
    // people.sort_by(|a, b| a.name.cmp(&b.name));
    // println!("{:?}", people); // ["Alice", "Bob", "Kate", "Sam", "Tom"]


    let numbers1 = vec![1, 2, 3, 4, 5];
    let numbers2 = vec![6, 7, 8, 9, 0];

    let sum: i32 =
        numbers1.iter()
        .chain(numbers2.iter())
        .map(|n| n * 2)
        .fold(0, |acc, x| acc + x); // вычисляем сумму чисел

    println!("Sum: {}", sum);   // Sum: 15
}



struct PrimeGenerator {
    current: u32
}
impl Iterator for PrimeGenerator {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.current += 1;
            if is_prime(self.current) {
                return Some(self.current);
            }
        }
    }
}
fn is_prime(num: u32) -> bool {
    if num <= 1 {
        return false;
    }

    for i in 2..(num / 2 + 1) {
        if num % i == 0 { return false; }
    }

    true
}

#[derive(Debug)]
struct Person{
    name: String,
    age: u32
}
