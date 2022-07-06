//Binary Search
fn binarySearch(v: &[u32]){
    let mut lowValue = 0 ; 
    let mut highValue = v.len() -1;
    let mut midPoint = 0;
    let mut guess = 0;
    let mut item = 8;
    while lowValue <= highValue {
        midPoint = lowValue + highValue;
        guess = v[midPoint];
        if guess == item{
            println!("{}", midPoint);
            break;
        }
        if guess > item{
            highValue = midPoint - 1;
            println!("Not Here, lower");
        }
        else {
            lowValue = midPoint + 1;
        }

    }
    println!("{:?}", v);
} 

fn main(){
    let v = vec![0,1,2,3,4,5,6,7,8,9,10];
    binarySearch(&v);
}