fn main(){
    let str : String = String::from("Hello , my name is harshit") ;
    let firstWord : String = first_word(str) ;
    print!("{}" , firstWord) ;
}

fn first_word(str : String) -> String{
    let mut ans : String = String::from("") ;
    for i in str.chars(){
        ans.push(i) ;
        if i == ' ' {
            break;
    }
}
return ans ;    
}