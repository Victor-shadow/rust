fn main(){
    let v1 = vec![1, 2, 3, 4];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), Some(&4));
    assert_eq!(v1_iter.next(), None);
}
//Note: v1_iter needs to be made mutable; calling next method on an iterator changes the internal state
//that the iterator uses to keep track of where it is in the sequence
//This code consumes or uses up the iterator, Each call to the next eats up an item from the iterator
//We did not need to make v1_iter mutable  then a for loop is used, because the loop took ownership of the v1_iter  and made it mutable

//Also note that the values we get from calls to next are immutable references to the values in the vector
//The iter method produces the iterator over immutable references
//If one wants to create an iterator that takes ownership of v1 and returns owned values  we can call into_iter method instead
//If One wants  to iterate over mutable references, we can call iter_mut instead