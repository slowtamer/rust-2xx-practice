fn main() {
    let s = String::from("Hello World");

    print_str(&s); // Передаємо посилання на s

    println!("{}", s); // Тепер можемо використовувати s
}

fn print_str(s: &String) { // Змінюємо тип параметра на посилання
    println!("{}", s);
}

