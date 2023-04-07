fs main(){
// signed and unsigned int
let unsignedNum:u8 =12;
let u_sint =12;
let signedNum:i8 = -23; //stored in 8 bit
let s_int = -90; //stored in 8 bit

// float
let floatIn:f12 =1.23;

// printing variables onto console

println!("usingned:{},signed:{},float:{}",u_sint,s_int,floatIn);

// charecters and emojis uisng utf encoding

let letter="R";
let emoji ="\u{1F600}";

println!("letter:{},empji:{}",letter,emoji);

let is_true= true;

println!("boolean:{}",is_true);



}