fn main(){
    let str : String = String::from("harshit") ;  // it can change at runtime 
    println!("{}" , str) ;

    // now lets print the first character of the string 

    let char1 : Option<char> = str.chars().nth(0) ; // here the chatr might exists or not , but rust ahve r=to make sure that befire printing it it should exists , so we cant just print char1 and move on 
    // before printing we have to check that the char1 exists or not 

    // this is somethiung called as pattern matching 
    match char1 {
        Some(chars) => println!("{}" , chars) ,
        None => println!("no char found , maybe the indexing is out of bound ") ,
    }

}