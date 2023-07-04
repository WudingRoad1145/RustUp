pub mod module2;
pub fn print_chars_module1() {
    for i in 'a' as u8 ..= 'z' as u8 {
        println!("{}", i as char);
    }
    for i in 'A' as u8 ..= 'Z' as u8 {
        println!("{}", i as char);
    }
}