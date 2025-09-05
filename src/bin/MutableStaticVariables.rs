static mut COUNTER: u32 = 0;
//SAFETY: Calling this from more than one thread at a time is undefined behaviour, so one *must* guarantee you only
//call it from a single thread at a time
unsafe fn add_to_count (inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}


fn main() {
    unsafe {
        //SAFETY: This is only called from a single thread `main
        add_to_count(3);
        println!("COUNTER: {}", *(&raw const COUNTER));
    }
}
//This code compiles and prints COUNTER: 3 since it is single threaded. Having multiple threads access COUNTER
//would likely result in data races, so it is undefined behaviour, Therefore we need to mark the entire function as UNSAFE
// and document the safety limitation,
