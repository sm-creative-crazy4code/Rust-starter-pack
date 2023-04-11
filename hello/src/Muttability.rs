fn main(){
    //in rust variables are immutable by default unless nade mutable using mut keyword

    // immutable
    let num =5;
    println!("{}",num);


    //mutable
    let mut n2=9;
    println!("n2={}",n2);
    n2=8;
    println!("n2={}",n2);
}