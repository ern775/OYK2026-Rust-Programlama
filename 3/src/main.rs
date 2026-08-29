fn main() {
    let sayi = 42;
    let r: *const i32 = &sayi;
    println!("{:?}", r);

    let a: &str = "merhaba";
    let b: String = a.to_string(); // heap'e kopyala, sahiplen
    let c: &str = &b; // geri pencere ac
    println!("{} {} {}", a, b, c);
}
