fn main(){
    let str1 : String = String::from("hello world") ;
    let str2 : String = str1 ;     // if koi unused varialbe hai toh uske aage _ lagate hain
    println!("{}" , str1) ;   // after assigning s1 to s2 , rust will delete s1 for maintaing the proper memory management , this is somethinf=g called ownership rule in rust 
    String1() ;  
}

fn String1(){
    let string1 : String = String::from("harshit") ;
    print_name(string1) ;     // jo string hai , vo heap me pada hai RAM ke andar , or us heap ke 1st element ka address stack me pada hai => string1 points to heaps data , but jb mene print_name function me string1 ko pass kiya , to now string2 will ppoint to the heap data , in rust , ek data ko ek hi variable point kr skta hai toh now string1 pointer will be deleted => jb me print_name call krne ke baad string1 ko access krne ka try kr raha hi toh error aa raha hai => its solution is string1.clone() but it only makes a copy of the element in heap , which is very costly 
    print!("{}" , string1) ;    // throw an error 
}

fn print_name(string2 : String) {
    print!("{}" , string2) ;
}