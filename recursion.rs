fn recursiveCall(x: &mut i16) -> i16{
    if *x == 1{
        return *x;
    }
    else{
        return *x * recursiveCall(&mut (*&mut *x-1));
    }
}

fn main(){
    let mut x = 5;
    println!("{}",recursiveCall(&mut x));
}