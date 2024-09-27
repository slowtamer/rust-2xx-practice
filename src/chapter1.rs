fn main() {
    let x = String::from("Hello world");
    let y = &x; // Створюємо посилання на x
    println!("{}, {}", x, y); // Тепер можемо використовувати обидві змінні
}
