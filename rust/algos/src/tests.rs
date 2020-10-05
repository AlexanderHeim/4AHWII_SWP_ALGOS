use crate::search::*;

#[test]
fn sequencial_search_test_1(){
    let vector = vec![1, 2, 3, 4, 5];
    let result = sequencial_search(&vector, 3);
    assert_eq!(result.unwrap(), 2);
}

#[test]
fn sequencial_search_test_2(){
    let vector = vec![1, 2, 4, 5];
    let result = sequencial_search(&vector, 3);
    assert!(result.is_none());
}

#[test]
fn sequencial_search_test_3(){
    let vector = vec![5, 2, 2, 3];
    let result = sequencial_search(&vector, 3);
    assert!(result.is_some());
}

#[test]
fn binary_search_test_1(){
    let vector = vec![0, 2, 4, 6, 8];
    let result = binary_search(&vector, 6);
    assert_eq!(result.unwrap(), 3);
}

#[test]
fn binary_search_test_2(){
    let vector = vec![0, 2, 4, 5, 6, 7, 12, 33, 412, 3000];
    let result = binary_search(&vector, 0);
    assert_eq!(result.unwrap(), 0);
}

#[test]
fn binary_search_test_3(){
    let vector = vec![1, 2, 4];
    let result = binary_search(&vector, 3);
    assert!(result.is_none());
}

#[test]
fn binary_search_test_4(){
    let vector = vec![1];
    let result = binary_search(&vector, 2);
    assert!(result.is_none());
}

#[test]
fn binary_search_test_5(){
    let vector = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let result = binary_search(&vector, 16);
    assert_eq!(result.unwrap(), 16);
}