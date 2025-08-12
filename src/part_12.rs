pub fn f1_unsafe() {
    let mut num = 5;

    let n1: *const i32 = &num;
    let n2: *mut i32 = &mut num;

    println!("n1={:p}", n1);
    println!("n2={:p}", n2);

    unsafe {
        let x = *n1;

        println!("n1={}", *n1);
        println!("n2={}", *n2);

        *n2 = 10;
    }
    println!("num={}", num);


    let addr = 0xffe0ff60c_usize;
    let p = addr as *const i32;
    println!("Address p: {:p}", p);
}



pub fn f2_ptr() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    let mut_ptr = numbers.as_mut_ptr();

    unsafe {
        for i in 0..numbers.len() {

            let current_value = *mut_ptr.add(i);
            *mut_ptr.add(i) = current_value * 2;
        }
    }

    println!("{:?}", numbers);  // [2, 4, 6, 8, 10]
}




pub fn f3_fn() {
    let mut num = 5;
    let num_pointer: *mut u32 = &mut num;
    unsafe{
        test(num_pointer);
    }
    println!("num: {}", num);
}

unsafe fn test(num_pointer: *mut u32){
    *num_pointer = *num_pointer * 2;
}


pub fn f3_fn_save() {
    let mut num = 5;
    test_save(&mut num);
    println!("num: {}", num);
}

fn test_save(num: &mut u32){

    *num = *num * 2;
}



// fn f4_c_code() {
//     unsafe {
//         println!("Absolute value of -3: {}", abs(-3));
//         printf(b"Hello World \n");
//     }
// }
//
// #[link(name = "my_c_library")]
// unsafe extern "C" {
//     fn abs(input: i32) -> i32;
//     fn printf(format_str:&[u8;13]);
// }


static COUNTER: u32 = 1;
static mut STATIC_VAL: u32 = 1;

pub fn f5_static() {
    println!("COUNTER  val: {COUNTER}"); // COUNTER: 1
    println!("COUNTER addr: {:p}", COUNTER as *const u32);

    add(5);
    unsafe {
        // STATIC_VAL += 1;
        println!("STATIC_VAL  val: {}", STATIC_VAL as u32);
        println!("STATIC_VAL addr: {:p}", STATIC_VAL as *const u32);
    }
}
fn add(n: u32) {
    unsafe {
        STATIC_VAL += n;   // для изменения нужен блок unsafe
    }
}





pub fn f6_union() {
    let s1 = Symbol{ letter: 'A' };
    unsafe {
        println!("s1: {}", s1.code);
    }

    let s2 = Symbol{ code: 65 };
    unsafe {
        println!("s2: {}", s2.letter);
    }
}

union Symbol {
    letter: char,
    code: u32,
}






pub fn f7_box() {
    let num = Box::new(22); // размещаем в куче число 22 с помощью Box
    println!("num: {num}"); // обращаемся к объекту в my_box - получим число

    let my_box = Box::new(vec![1, 2, 3, 4, 5]); // создаем вектор в куче с помощью Box
    let sum: i32 = my_box.iter().sum(); // обращаемся к объекту в my_box - получим сумму значений
    let len: usize = my_box.len(); // обращаемся к объекту в my_box - получим размер вектора
    println!("Sum: {}  Len: {}", sum, len);

}   // Завершение области жизни Box и освобождение связанной с ним памяти


pub fn f8_box() {
    // let ref_pt: &Point;
    // {
    //     let pt = Point {x: 1, y: 2};
    //     ref_pt = &pt;       // ! Ошибка
    // }
    // println!("{:?}", ref_pt);

    let box_pt: Box<Point>;
    {
        box_pt = Box::new(Point { x: 10, y: 20 });
    }
    println!("{:?}", box_pt);   // Point { x: 10, y: 20 }



    let numbers = List::Node(1,
                        Box::new(List::Node(2,
                            Box::new(List::Node(3,
                                Box::new(List::Nil))))));
    println!("{:?}", numbers);

    // let p4 = List::Nil;
    // let p3 = List::Node(3, &p4);
    // let p2 = List::Node(2, &p3);
    // let numbers = List::Node(1, &p2);
    // println!("{:?}", numbers);
    // // drop(p3);


    let p_num = Box::new(22);
    // let result = p_num > 10;        // !Ошибка - mismatched types
    let result = *p_num > 10;    // true
    println!("{}", result);     // true
}

#[derive(Debug)]
struct Point{ x: u64, y: u64 }

#[derive(Debug)]
enum List {
    Node(i32, Box<List>),
    Nil,
}
// #[derive(Debug)]
// enum List<'a> {
//     Node(i32, &'a List<'a>),
//     Nil,
// }


pub fn f9_rc() {
    use std::rc::Rc;

    let data = Rc::new(vec![1, 2, 3, 4, 5]); // определяем разделяемые/общие данные
    let clone1 = Rc::clone(&data); // копируем ссылку
    let clone2 = Rc::clone(&data); // копируем ссылку

    println!("Len: {}", data.len());    // Sum: 15   Len: 5
    println!("Data: {:?}", data);       // Data: [1, 2, 3, 4, 5]
    println!("Clone1: {:?}", clone1);   // Clone1: [1, 2, 3, 4, 5]
    println!("Clone2: {:?}", clone2);   // Clone2: [1, 2, 3, 4, 5]

    println!("ref count (after creating clone2) = {}", Rc::strong_count(&data));
    drop(clone2);
    println!("ref count (after deleting clone2) = {}", Rc::strong_count(&data));



    use std::cell::RefCell;

    let data = RefCell::new(vec![1, 2, 3]); // определяем данные
    {
        let data_ref = data.borrow(); // получаем неизменяемую ссылку
        println!("Data: {:?}", data_ref); // Data: [1, 2, 3]
    }
    {
        let mut data_ref_mut = data.borrow_mut(); // получаем изменяемую ссылку на данные
        println!("Original Data: {:?}", data_ref_mut); // Original Data: [1, 2, 3]
        data_ref_mut.push(4);
        println!("Modified Data: {:?}", data_ref_mut); // Modified Data: [1, 2, 3, 4]
    }
}
