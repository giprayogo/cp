use std::error::Error;

/// Sketching board for things
fn main() -> Result<(), Box<dyn Error>> {
    println!("{:?}", 'a' as usize);
    println!("{:?}", 'z' as u8);
    let s: &str = "123";
    let ptr: *const u8 = s.as_ptr();

    unsafe {
        println!("{}", *ptr.offset(1) as char);
        println!("{}", *ptr.offset(2) as char);
    }
    Ok(())
}
