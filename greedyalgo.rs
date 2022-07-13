fn knapsack(a: & Vec<u32>, b: &u32){
    let mut optimalVector  = Vec::new();
    for i in 1..a.len(){
        if optimalVector.iter().sum::<u32>() <= *b{
            optimalVector.push(a[i]);
            if optimalVector.iter().sum::<u32>() > *b{
                optimalVector.pop();
            }
        }
    }
    println!("{:?}", optimalVector);
}

fn main(){
    let a = vec![4,1,2,8,5,6];
    let optimal = 10;
    knapsack(&a, &optimal);
}