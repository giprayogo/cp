use std::{error::Error, collections::BinaryHeap};

/// Sketching board for things
fn main() -> Result<(), Box<dyn Error>> {
    println!("{:?}", 'a' as usize);
    println!("{:?}", 'z' as usize);
    let s: &str = "123";
    let ptr: *const u8 = s.as_ptr();

    unsafe {
        println!("{}", *ptr.offset(1) as char);
        println!("{}", *ptr.offset(2) as char);
    }

    let mut h = BinaryHeap::from([(1,2), (2,2)]);
    println!("{:?}", h.pop().unwrap());
    println!("{:?}", h.pop().unwrap());
    Ok(())
}
