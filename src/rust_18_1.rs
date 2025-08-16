pub trait Draw {
    fn draw(&self);
}


pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}
impl Screen {
    pub fn run(&self) {
        for component in &self.components {
            component.draw();
        }
    }
}

#[derive(Debug)]
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}
#[derive(Debug)]
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for Button {
    fn draw(&self) {
        println!("draw button {self:?}");
    }
}
impl Draw for SelectBox {
    fn draw(&self) {
        println!("draw select box {self:?}");
    }
}
impl Draw for String {
    fn draw(&self) {
        println!("draw string {self:?}");
    }
}




pub fn f1_run() {
    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 0,
                height: 0,
                label: String::from("AAA"),
            }),
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
            Box::new(Button {
                width: 200,
                height: 170,
                label: String::from("Btn!"),
            }),
            Box::new(String::from("Hi"))
        ],
    };

    screen.run();
}

