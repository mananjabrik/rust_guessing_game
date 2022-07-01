// Reference Pointer - Point to a resource in memoru

pub fn run() {
    // primitive array
    let array1 = [1, 2, 3, 4, 5];
    let array2 = array1;

    // With non-primitvs, if you assign another varibale to a pieces of data, the first Variable will no longer  hold that value, You'll need to user a refrences {(&) to point to the resources

    //Vector
    let vec1 = vec![1, 2, 3, 4, 5];
    let vec2 = &vec1;

    println!("array1[0]: {:?}", (&vec1));
}
