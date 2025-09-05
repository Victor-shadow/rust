fn main(){
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2:  Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}
//The results of iterating over the iterator that's returned from the call to map into a vector
//the vector ends up containing each item from the original vector, incremented by one
//Because map takes any closure one can specify any kind of operation  we want to perform on each item
