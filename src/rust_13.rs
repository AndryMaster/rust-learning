use std::thread;

pub fn f1_closure() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    return;

    #[derive(Debug, PartialEq, Copy, Clone)]
    enum ShirtColor {
        Red,
        Blue,
    }

    struct Inventory {
        shirts: Vec<ShirtColor>,
    }

    impl Inventory {
        fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
            user_preference.unwrap_or_else(|| self.most_stocked())
        }

        fn most_stocked(&self) -> ShirtColor {
            let mut num_red = 0;
            let mut num_blue = 0;

            for color in &self.shirts {
                match color {
                    ShirtColor::Red => num_red += 1,
                    ShirtColor::Blue => num_blue += 1,
                }
            }
            if num_red > num_blue {
                ShirtColor::Red
            } else {
                ShirtColor::Blue
            }
        }
    }
}




pub fn f2_closure() {
    fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x|             { x + 1 };
    let add_one_v4 = |x|               x + 1  ;
    add_one_v3(add_one_v4(1u32));



    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(std::time::Duration::from_secs(1));
        num
    };
    println!("Make closure");
    println!("Start res");
    let res = &expensive_closure(1);
    println!("Print result down");
    println!("Print result = {res}");
}



pub fn f3_closure() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");
    let only_borrows = || println!("From closure: {list:?}");
    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");

    println!("Before defining mut closure: {list:?}");
    let mut borrows_mutably = || list.push(7);
    borrows_mutably();
    println!("After calling mut closure: {list:?}");

    println!("Before defining closure: {list:?}");
    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
    // println!("After calling closure: {list:?}");



    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{list:#?}, sorted in {num_sort_operations} operations");
}



pub fn f4_iter() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    let r1 = v2;
    println!("{:?}", r1);
}
