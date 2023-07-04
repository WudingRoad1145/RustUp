pub fn print_chars_module2() {
    for i in 'A' as u8 ..= 'z' as u8 {
        if i <= 'Z' as u8 || i >= 'a' as u8 {
            println!("{}", i as char);
        }
    }
}