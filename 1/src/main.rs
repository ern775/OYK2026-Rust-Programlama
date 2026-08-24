fn main() {
    println!("Hello, world!");

    let x = 16;
    println!("x = {}", x);
    println!("x = {}", x);

    let mut y = 20;

    y = y + 4;
    println!("y = {}", y);
    y = y - 6;
    println!("y = {}", y);

    let mut z = String::from("OYK");
    println!("z = {}", z);

    z.push_str("new");
    println!("z = {}", z);

    const PI: f64 = 3.14;
    static AD: &str = "OYK Sınıf";
    println!("PI = {}", PI);
    println!("AD = {}", AD);

    let mut s = 30;
    {
        let s = 48;
        println!("s = {}", s);
    }
    println!("s = {}", s);

    s = 42;
    println!("s = {}", s);

    let i8_var = 126i8;
    let u16: u16 = 450;
    let u16: f64 = 450.0;
    println!("i8_var = {}", i8_var);
    println!("u16 = {}", u16);

    let mut content = String::from("OYK");
    println!("{:?}", content);
}
