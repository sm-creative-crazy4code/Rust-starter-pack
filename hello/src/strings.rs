fn main(){

let str:&str="hello world";
let mut string: String = String ::from("Hellow World");
println("{}",string);
let slice =&string[..6];
println("{}",slice);
slice.len();

string.push('!');
string.push_str(" Alice");
println("{}",string);
let str2 = string.replace("Hello","Bye");
println("{}",string);
     
}