pub fn f1_owner() {
    let s1 = "hello".to_string();   // s1 - владелец строки "hello"
    println!("s1: {}", s1);

    let s2 = s1;                    // Меняем владельца строки на s2

    // println!("s1: {}", s1);             // ! Ошибка - s1 теперь нельзя использовать
    println!("s2: {}", s2);
    drop(s2);

    let s1 = "new hello".to_string();
    println!("s1: {}", s1);
}

pub fn f2_reference() {
    let s1 = "hello".to_string();   // s1 - владелец строки "hello"
    let s2 = &s1;                  // s2 получает ссылку на значение переменной s1

    println!("s1: {}", s1);     // s1: hello
    println!("s2: {}", s2);     // s2: hello


    let s2: &String;
    {
        let s1 = "hello".to_string();
        s2 = &s1;                             // s2 получает ссылку на s1
        println!("s2 (inner): {}", s2);       // s2: hello
    }   // конец контекста переменной s1

    // здесь s2 уже не инициализирована, так как жизненный цикл s1 закончился
    // println!("s2 (outer): {}", s2); // ! Ошибка - s1 уже не существует
}

pub fn f3_return_ref() {
    let res = create_let();
    fn create_let() -> String {
        let s = String::from("hello");
        s
    }

    // let res = create_ref();
    // fn create_ref() -> &String {
    //     let s = String::from("hello");
    //     &s    // ! Ошибка - возвращаем ссылку на строку
    // }
}

pub fn f4_mut_reference() {
    let num = 22;
    let p_num = &num;
    let result = *p_num > 10;
    println!("{}", result);


    let mut s1 = "hello".to_string();
    let s2 = &mut s1;   // первая изменяемая ссылка
    // let s3 = &mut s1;   // вторая изменяемая ссылка
    s2.push('!');
    println!("{}", s1); // hello!


    let mut s1 = "hello".to_string();
    // let s2 = &mut s1;    // если определить здесь, будет ошибка
    {
        let s3 = &mut s1;
        s3.push('?');
    }
    let s2 = &mut s1;
    s2.push('!');
    println!("{}", s1); // hello?!


    let mut s1 = "hello".to_string();
    let s2 = &mut s1;   // изменяемая ссылка
    s2.push('!');
    println!("{}", s1); // hello!
    // дальше изменяемая ссылка s2 НЕ используется
    let s3 = &s1;   // неизменяемая ссылка
    println!("{}", s3); // hello!
}

pub fn f5_owner_2() {
    let hello = "Hello METANIT.COM".to_string();

    let print_message = || {    // замыкание
        let message = hello;  // получаем владение значением
        println!("{}", message);
    };

    print_message();            // +
    // print_message();         // -
    // println!("{}", hello);   // -



    let hello = "Hello METANIT.COM".to_string();

    let print_message = || {      // замыкание
        let message = &hello;  // сохраняем ссылку на переменную
        println!("{}", message);
    };

    print_message();        // +
    print_message();        // +
    println!("{}", hello);  // +
}

pub fn f6_owner_3() {
    // Fn borrow
    let mut number = 22;
    let print_number = || {
        println!("number in print_number: {}", number);
    };
    // number += 1; // ! Ошибка, здесь нельзя
    println!("number: {}", number); // используем number
    print_number(); // вызываем замыкание
    print_number(); // вызываем замыкание
    number += 1; // number = 22 + 1 = 23
    println!("number: {}\n\n", number); // используем number


    // FnMut borrow
    let mut number = 22;
    let mut print_number = || {
        number += 1;
        println!("number in print_number: {}", number);
    };
    // number += 1; // ! Ошибка, здесь нельзя
    // println!("number in main: {}", number); // ! Ошибка, здесь нельзя
    print_number(); // вызываем замыкание
    print_number(); // вызываем замыкание
    number += 1;
    println!("number: {}\n\n", number);


    // FnOnce borrow
    let number = 22;
    let print_number = || {
        let mut num = number;
        num += 1;
        println!("num in print_number: {}", num);
    };
    println!("number: {}", number);
    print_number();
    print_number();
    println!("number: {}\n\n", number);

    let hello = "hello".to_string();
    let print_message = || {
        let mut message = hello;
        message.push('!');
        println!("message: {}", message); // используем number
    };
    print_message();
    // print_message();  // ! Ошибка - здесь нельзя
    // println!("main message: {}", hello); // ! Ошибка - здесь нельзя
}
