pub fn f1_alive() {
    let x: &i32;
    {
        let n = 2;
        x = &n;
        println!("x: {}", x);
    }
    // println!("x: {}", x);  // Error

    let mes = get_message();
    let mes2 = get_message2();
    println!("mes: {} mes2: {}", mes, mes2);
}

// fn get_message() -> &str { "hello" }  // Error lifetime parameter
fn get_message() -> String { "hello".to_string() }
fn get_message2<'a>() -> &'a str { "hello" }




pub fn f2_alive_fn() {
    let username = String::from("Sam");
    let checked_username = check_name(&username);
    // let checked_username;
    // {
    //     let username = String::from("admin");
    //     checked_username = check_name(&username);
    // }
    println!("username 1: {}", checked_username);


    let username = String::from("Sam");
    {
        let default_name = String::from("Tom");
        let checked_username2 = check_name_2(&username, &default_name);
        println!("username 2: {}", checked_username2);
    }
    // println!("username 3: {}", checked_username2);


    let message = get_str();
    println!("Message: {}", message);
}

fn check_name(name: &str) -> &str {
    if name == "admin" { "Tom" }
    else { name }
}

fn check_name_2<'a>(name: &'a str, default: &'a str) -> &'a str {
    if name == "admin" { default }
    else { name }
}

fn get_str() -> String {
    String::from("Hello Rust")
}




pub fn f3_alive_struct() {
    let mut tom;
    {
        let username = String::from("Tom");
        tom = Person { name: &username };
        println!("{}", tom.name);
        tom.name = "Tomas";
        println!("{}", tom.name);
    }
    // tom.name = "Tomas";
    // println!("{}", tom.name);


    let bob = Person { name: "Bob"};
    bob.print();    // Имя пользователя: Tom

    let speech = String::from("Hello");
    let sam_saying;
    {
        let sam = Person { name: "Sam"};
        sam_saying = sam.say(&speech);
    }   // конец жизни объекта sam
    println!("text: {}", sam_saying); // ссылку sam_saying по прежнему можно использовать
}

struct Person<'a> {
    name: &'a str,
}

impl<'a> Person<'a>{
    fn print(&self) {
        println!("Имя пользователя: {}", self.name);
    }

    fn say<'b>(&self, text: &'b str) -> &'b str{
        println!("{} говорит: {}", self.name, text);
        text
    }
}




pub fn f4_alive_static() {
    let message1 = get_message();
    println!("message1: {}", message1);

    
    let message2;
    {
        let words = get_message();
        message2 = words;
    }
    // ссылка, возвращенная get_message, продолжает существовать
    println!("message2: {}", message2);
}

fn get_message_static() -> &'static str {
    "hello"
}
