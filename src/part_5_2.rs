pub fn f6_type() {
    let circle = Circle{radius:10.0};
    println!("Area = {}", circle.area());   // Area = 314

    let rect = Rectangle{width:10., height:20.};
    println!("Area = {}", rect.area());     // Area = 200

    let mut my_shape: &dyn Area = &Circle { radius: 5.0 }; // определяем трейт объект, который хранит ссылку на структуру Circle
    println!("Circle area: {}", my_shape.get_area()); // вызываем метод area у объекта трейта my_shape
    print_area(my_shape);
    my_shape = &Rectangle {width:10., height: 20.};   // изменяем значение объекта трейта
    println!("Rectangle area: {}", my_shape.get_area());
    print_area(my_shape);
}

struct Circle { radius:f64 }
struct Rectangle { width:f64, height:f64 }

trait Shape{
    type Unit;  // ассоциированный тип
    fn area(&self)->Self::Unit;
}
impl Shape for Circle{
    type Unit = f64;
    fn area(&self) -> Self::Unit { self.radius * self.radius * std::f64::consts::PI }
}
impl Shape for Rectangle{
    type Unit = u32;
    fn area(&self) -> Self::Unit { (self.width * self.height) as u32 }
}

trait Area {
    fn get_area(&self) -> f64; // метод вычисления площади фигуры
}
impl Area for Circle {
    fn get_area(&self) -> f64 {
        self.radius * self.radius * 3.14
    }
}
impl Area for Rectangle {
    fn get_area(&self) -> f64 {
        self.width * self.height
    }
}
fn print_area(shape: &dyn Area) {
    println!("Area: {}", shape.get_area());
}






pub fn f7_type_trait() {
    attack::<Knight>();
    attack::<Mage>();
}

// программирование на уровне типа
fn attack<C: Character>() {
    let weapon = C::create_weapon(); // создаем оружие ассоциированного типа
    weapon.attack();  // применяем оружие
}

// трейт персонажа в игре
trait Character {
    type WeaponType: Weapon;    // оружие, ассоциированое с персонажем
    // метод создания оружия, которое ассоциировано с персонажем
    fn create_weapon() -> Self::WeaponType;
}

// трейт оружия
trait Weapon {
    fn attack(&self);
}

struct Knight;
struct Mage;

struct Sword;
struct Staff;

// реализация меча
impl Weapon for Sword {
    fn attack(&self) {
        println!("Атакуем мечом");
    }
}

// реализация для посоха
impl Weapon for Staff {
    fn attack(&self) {
        println!("Применяем магию");
    }
}

// реализация воина
impl Character for Knight {
    type WeaponType = Sword;    // оружие - меч
    fn create_weapon() -> Self::WeaponType { Sword }
}

// реализация мага
impl Character for Mage {
    type WeaponType = Staff;    // оружие - магический посох
    fn create_weapon() -> Self::WeaponType { Staff }
}




pub fn f8_debug() {
    let tom = Person{name:"Tom".to_string(), age: 40};
    println!("{:?}", tom);  // Person { name: "Tom", age: 40 }
    dbg!(tom);
}

#[derive(Debug)]
struct Person
{
    name: String,
    age: u8
}
