fn findSmallest(v: &[u32]) -> usize {
    let mut smallest = v[0];
    let mut smallestIndex = 0;
    for val in 1..v.len(){
        if v[val] < smallest{
            smallest = v[val];
            smallestIndex = val;
        }
    }
    return smallestIndex;
}

fn selectionSort(v: &[u32]){
    let mut vec = Vec::new();
    for val in 0..v.len(){
        let mut smallest = findSmallest(&v);
        vec.append(v.pop(smallest))
    }
}

fn main(){
    let v = vec![5, 3, 2, 5, 10, 7];
    findSmallest(&v);
}