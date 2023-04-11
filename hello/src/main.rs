// // // // // fn main() {
// // // // //     println!("Hello, world!");
// // // // //     // it is not a function but a macro as evident by the exclamanation mark
// // // // // }

// // // // fn main(){
// // // //     // signed and unsigned int
// // // //     let unsignedNum:u8 =12;
// // // //     let u_sint =12;
// // // //     let signedNum:i8 = -23; //stored in 8 bit
// // // //     let s_int = -90; //stored in 8 bit
    
// // // //     // float
// // // //     let floatIn:f32 =1.23;
    
// // // //     // printing variables onto console
    
// // // //     println!("usingned:{},signed:{},float:{}",u_sint,s_int,floatIn);
    
// // // //     // charecters and emojis uisng utf encoding
    
// // // //     let letter="R";
// // // //     let emoji ="\u{1F600}";
    
// // // //     println!("letter:{},empji:{}",letter,emoji);
    
// // // //     let is_true= true;
    
// // // //     println!("boolean:{}",is_true);
    
    
    
// // // //     }

// // // fn main(){
// // //     // declaring array
// // //     let arr:[u8;3] =[1,3,4];
// // //     let arr2:[u8;2]=[100,5];
    
// // //     // printng elements and their indexes
// // //     println!("index:{},length:{}",arr[0],arr2.len());
    
// // //     println!("{:?}",arr2);
    
// // //     // declaring tuples and vars
    
// // //     let tuple:(u8,bool,f32)=(5,true,4.34);
// // //     let t2=(4,54);
    
// // //     //accessing tuples via their indices
// // //     println!("first:{},second:{},third:{}",tuple.0,tuple.1,tuple.2);
    
// // //     let (a,b,c)= tuple;
// // //     // destructuring
// // //     println!("first:{},second:{},third:{}",a,b,c)
    
    
    
    
    
// // //     }

// // // fn main(){

// // //     println!("{}",is_even(9));

// // // }

// // // // unless declared public functions are private in rust
// // // pub fn is_even(num:u8)->bool {
// // // let digit:u8 =num%2;
// // // digit==0 //return value doesnot have semicoloun

// // // }

// // fn main(){
// //     //in rust variables are immutable by default unless nade mutable using mut keyword

// //     // immutable
// //     let num =5;
// //     println!("{}",num);


// //     //mutable
// //     let mut n2=9;
// //     println!("n2={}",n2);
// //     n2=8;
// //     println!("n2={}",n2);
// // }

// fn main(){
//     // indexing starts from 0
//     let arr =[3,5,4,5,6]; //it is a reference and we arre actullly pointing to the address and memory 
//     let slice= &arr[1..3]; // last value is exclusive we dont know the length at compile timed
//     burrowing_slice(arr,slice)
    
    
//     }
    
//     fn burrowing_slice(arr:[u8;5],slice: &[u8]){
//         println!("{:?}",arr);
//         println!("{:?}",slice);
//         println!("length array :{}",arr.len());
//         println!("length slice :{}",slice.len());
//         println!("{} {}",slice[0],slice[1]);
//     }

fn main(){

    let str:&str="hello world";
    let mut string: String = String ::from("Hellow World");
    println!("{}",string);
    let slice =&string[..6];
    println!("{}",slice);
    slice.len();
    
    string.push('!');
    string.push_str(" Alice");
    println!("{}",string);
    let str2 = string.replace("Hello","Bye");
    println!("{}",str2);
         
    }