pub fn f1() {
    let user: (&str, u8, f64) = ("Tom", 36, 1.78);
    let (name, age, height) = user;

    println!("Имя: {}", name);
    println!("Возраст: {}", age);
    println!("Рост: {}", height);



    let user  = (String::from("Tom"), 39);
    let employee = user;        // передача владения переменной employee
    let people = &employee;    // нет передачи владения

    // println!("User Name: {}", user.0);  // ! Ошибка
    println!("Employee Name: {}", employee.0);
    println!("People Name: {}", people.0);
}

pub fn f2() {
    let numbers: [i32; 5] = [34, 50, 25, 100, 65];
    println!("{:?}", numbers);
    for number in &numbers { print!("{} ", number); }
    println!();


    let mut users = ["Tom", "Bob", "Sam"];
    println!("{}", users[1]);
    users[1] = "Bill";
    println!("{}", users[1]);
    println!("len(): {}", users.len());


    let numbers = [777; 3];
    println!("numbers: {:?} ", numbers);
    let copied = numbers.to_owned();
    println!("copied: {:?} ", copied);

    let mut numbers: [i32; 7] = [10, 12, 354, 3, 4, 6, 5];
    numbers.sort();
    println!("numbers: {:?} ", numbers);


    let numbers: [[i32; 3]; 2] = [
        [1, 2, 3],
        [4, 5, 6]
    ];
    for array in numbers{
        println!("{:?}", array);
    }
}

pub fn f3() {
    struct Person {
        name: String,
        age: u8,
        weight: f32
    }

    let tom = Person {
        name: String::from("Tom"),
        age: 24,
        weight: 78.0
    };

    println!("Name: {}, age: {}, weight: {}", tom.name, tom.age, tom.weight);

    fn print_person(person: Person) {
        println!("Name: {}  Age: {}", person.name, person.age);
    }
    fn print_per(person: &Person) {
        println!("Name: {}  Age: {}", person.name, person.age);
    }
    let tom = Person { name: "Tom".to_string(), age: 40, weight: 100.0 };
    print_per(&tom);
    print_person(tom);  // передача владения структурой Person в функцию
    // print_person(tom);  // ! Ошибка - функция main уже не владеет структурой Person
}

pub fn f4() {
    enum Season {
        Winter,
        Spring,
        Summer,
        Autumn
    }

    let mut season = Season::Spring;
    season = Season::Summer;

    match season {
        Season::Winter => println!("Winter"),
        Season::Spring => println!("Spring"),
        Season::Summer => println!("Summer"),
        Season::Autumn => println!("Autumn")
    }
}
