fn findSmallest(v: &mut Vec<u32>) -> u32 {
    let mut smallest = v[0] as u32;
    let mut smallestIndex = 0 as u32;
    for val in 1..v.len(){
        if v[val] < smallest{
            smallest = v[val];
            smallestIndex = val as u32;
        }
    }
    return smallest;
}

fn selectionSort(v: &mut Vec<u32>){
    let mut vec = Vec::new();
    for val in 0..v.len(){
        let mut smallest = findSmallest(v);
        vec.push(smallest);
        v.retain(|x| *x != smallest);
    }
    println!("{:?}", vec);
}

fn main(){
    let mut v = vec![5, 3, 2, 6, 10, 7];
    selectionSort(&mut v);
}