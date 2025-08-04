fn main() {
    let num : u8 = 3 ;
    if num == 2 {    // no traditional parenthesis 
        print!("ok proceed") ;
    }else {
        print!("Chala ja bosdike !") ;
    }

    // loop 
    //  Rust, you don’t specify the type of the loop variable directly in the for syntax. Instead, Rust infers the type of i from the iterator you're looping over.
    for i in 1u8..10{
        print!("{} " , i) ;
    }
}