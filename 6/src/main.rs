struct Bookmark {
    text: String,
}

fn main() {
    let bookmark;
    {
        let temp_book = String::from("Rust Programlama");
        bookmark = Bookmark { text: temp_book };
    }
    println!("{}", bookmark.text);
}
