fn main() {
    let x = (1, 2, (), "hello"); // Використовуємо &str замість String
    let y = x; // Тепер y копіюється, оскільки всі елементи реалізують Copy
    println!("{:?}, {:?}", x, y);
}
