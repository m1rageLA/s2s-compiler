

#[test]
fn test_push_array() {
    let mut array = vec![1, 2, 3];

    let result = push(&mut array, 4);
    assert_eq!(result, 4);
    assert_eq!(array, vec![1, 2, 3, 4]);
}
