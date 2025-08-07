use std::collections::*;



pub fn f1_vec() {
    let mut v1 = vec![1, 2, 3];
    v1.clear();

    let mut v2 = Vec::new();
    v2.push(5);
    v2.push(6);
    v2.push(7);
    v2.push(8);
}


pub fn f2_vec() {
    let mut v = vec![1, 2, 3, 4, 5];

    let third = &v[2];
    // v.push(6);  // mutable borrow occurs here
    println!("The third element is {third}");

    let third = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }


    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    for i in &v {
        println!("{i}");
    }
}


pub fn f3_vec() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}




pub fn f4_string() {
    let s = String::new();
    let s = String::from("text");
    let s = "text".to_string();

    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Hello");

    let s1 = String::from("hi");
    let h = s1.chars().nth(0).unwrap();
    // let h = s1[0];
    println!("{}", h);

    let hello = "hello";
    let hell = &hello[0..4];
}





pub fn f5_hashmap() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    scores.insert(String::from("Red"), 50);
    scores.entry(String::from("Red")).or_insert(55);
    scores.entry(String::from("Yellow")).or_insert(55);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
}

pub fn f6_hashmap() {
    let text = "hello world wonderful world big big World";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word.to_lowercase()).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}
