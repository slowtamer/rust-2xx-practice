// Don't modify code in main!
fn main() {
    let s1 = String::from("Hello world");
    let s2 = take_ownership(s1);

    println!("{}", s2); // Тепер s2 може бути використане
}

// Only modify the code below!
fn take_ownership(s: String) -> String { // Додаємо повернення типу String
    println!("{}", s);
    s // Повертаємо s назад
}

