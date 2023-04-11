fn main(){
let arr =[3,5,4,5,6]; //it is a reference and we arre actullly pointing to the address and memory 
let slice= &arr[1..3]; // last value is exclusive we dont know the length at compile timed
burrowing_slice(arr,slice)


}

fn burrowing_slice(arr:[u8:4],slice: &[u8]){
    println!("{?:}",arr);
    println!("{?:}",slice);
    println!("length array :{}",arr.len());
    println!("length slice :{}",slice.len());
    println!("{} {}",slice[0],slice[1]);
}