use std::fmt::Display;



pub fn f1_generic() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.1, y: 4.5 };

    println!("p.x = {}, p.y = {}", integer.x(), integer.y);
    println!("p.x = {}, p.y = {}", float.x(), float.y);
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

struct Point<T> {
    x: T,
    y: T,
}




pub fn f2_lifetime() {
    let x = 5;
    let r = &x;
    println!("r: {r}");

    let string1 = "abcd";
    let string2 = "xyz";
    let result = longest(&string1, string2);
    println!("The longest string is '{result}'");

    let result = longest_with_an_announcement(&string1, string2, 777);
    println!("The longest string is '{result}'");
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}
