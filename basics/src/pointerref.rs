pub fn run () {
    // Primitive array
    let arr1 = [1,2,3];
    let arr2 = arr1;

    println!("{:?} {:?}",arr1, arr2);

    // With non-primitives, the variable being assigned from will
    // no longer hold the value

    let vec1 = vec![1,2,3];
    let vec2 = &vec1;

    println!("{:?} {:?}",&vec1, vec2);
}