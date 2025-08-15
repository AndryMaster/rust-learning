use std::cell::RefCell;
use std::ops::Deref;
use std::rc::{Rc, Weak};

use crate::rust_15::List1::{Cons as Cons1, Nil as Nil1};
use crate::rust_15::List2::{Cons as Cons2, Nil as Nil2};



pub fn f1_box() {
    let b = Box::new(5);
    println!("b = {b}");

    let x = 5;
    let y = &&&x;

    assert_eq!(5, x);
    assert_eq!(5, ***y);

    let y = MyBox::new(x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    fn hello(name: &str) {
        println!("Hello, {name}!");
    }
}

struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x) }
}
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0 }
}






pub fn f2_box() {
    let c = MySmartPointer {
        data: String::from("my stuff"),
    };
    let d = MySmartPointer {
        data: String::from("other stuff"),
    };
    println!("MySmartPointers created.");

    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}

struct MySmartPointer {
    data: String,
}
impl Drop for MySmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}






pub fn f3_ref() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons1(Rc::clone(&value), Rc::new(Nil1)));

    let b = Cons1(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons1(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");
}

#[derive(Debug)]
enum List1 {
    Cons(Rc<RefCell<i32>>, Rc<List1>),
    Nil,
}





pub fn f4_ref_cycle() {
    let a = Rc::new(Cons2(5, RefCell::new(Rc::new(Nil2))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons2(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle it will overflow the stack.
    // println!("a next item = {:?}", a.tail());
}

#[derive(Debug)]
enum List2 {
    Cons(i32, RefCell<Rc<List2>>),
    Nil,
}
impl List2 {
    fn tail(&self) -> Option<&RefCell<Rc<List2>>> {
        match self {
            Cons2(_, item) => Some(item),
            Nil2 => None,
        }
    }
}





pub fn f5_ref_cycle() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        
        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}
