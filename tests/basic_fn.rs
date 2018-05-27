// Copyright 2018 KaguyaRs Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
#[macro_use]
extern crate kaguya_rs;

#[test]
/// macro compose!
fn compose() {
    let f = compose! {
        |x| x+1, |x| x*2
    };
    assert_eq!(f(3), 7);
    assert_eq!(f(-1), -1);

    fn add_one(x: i64) -> i64 {
        x + 1
    }
    fn multi_two(x: i64) -> i64 {
        x * 2
    }
    let f2 = compose!(add_one, multi_two);
    
    assert_eq!(f2(3), 7);
    assert_eq!(f2(-1), -1);
}

#[test]
/// macro compose! for type projection
fn compose_type_projection() {
    // f :: &str -> (&str -> Vec String -> usize) -> usize
    let f = compose! {
        |x: Vec<&str>| x.len(),
        |x: &'static str| x.split(' ').collect()
    };
    assert_eq!(f("Houraisan Kaguya"), 2);
}

#[test]
/// macro pipe!
fn pipe() {
    let f = pipe! {
        |x| x+1, |x| x*2
    };
    assert_eq!(f(3), 8);
    assert_eq!(f(-1), 0);

    fn add_one(x: i64) -> i64 {
        x + 1
    }
    fn multi_two(x: i64) -> i64 {
        x * 2
    }
    let f2 = pipe!(add_one, multi_two);
    
    assert_eq!(f2(3), 8);
    assert_eq!(f2(-1), 0);
}

#[test]
/// macro pipe! for type projection
fn pipe_type_projection() {
    // f :: &str -> (&str -> Vec String -> usize) -> usize
    let f = pipe! {
        |x: &'static str| x.split(' ').collect(),
        |x: Vec<&str>| x.len()        
    };
    assert_eq!(f("Houraisan Kaguya"), 2);
}

#[test]
// fn map and curry map
fn map() {
    use kaguya_rs::basic_fn::fun::map;
    let v = vec![1,2,3];
    let result: Vec<i32> = map(|x| x+1, v.iter()).collect();
    assert_eq!(result, vec![2,3,4]);

    let curry = map!(|x| x+1);
    let result2: Vec<i32> = curry(v.iter()).collect();
    assert_eq!(result2, vec![2,3,4]);
}

#[test]
// fn filter and curry filter
fn filter() {
    use kaguya_rs::basic_fn::fun::filter;
    let v = vec![1,2,3];
    let odd = filter(|&x| x & 1 == 1, v.iter()).map(|&x| x).collect::<Vec<i32>>();
    assert_eq!(odd, vec![1,3]);

    let curry = filter!(|&x| x&1 == 0);
    let even = curry(v.iter()).map(|&x| x).collect::<Vec<i32>>();
    assert_eq!(even, vec![2]);
}

#[test]
// fn filter_not and curry filter_not
fn filter_not() {
    use kaguya_rs::basic_fn::fun::filter_not;
    let v = vec![1,2,3];
    let even = filter_not(|&x| x & 1 == 1, v.iter()).map(|&x| x).collect::<Vec<i32>>();
    assert_eq!(even, vec![2]);

    let curry = filter_not!(|&x| x&1 == 0);
    let odd = curry(v.iter()).map(|&x| x).collect::<Vec<i32>>();
    assert_eq!(odd, vec![1,3]);
}

#[test]
// fn foldl and curry foldl
fn foldl() {
    use kaguya_rs::basic_fn::fun::foldl;
    let v = vec![1,2,3];
    let result = foldl(4, |x,y| x*y, v.iter());
    assert_eq!(result, 24);

    let curry1 = foldl!(5);
    let c_result1 = curry1(|x,y| x*y, v.iter());
    assert_eq!(c_result1, 30);

    let curry2 = foldl!(6, |x,y| x-y);
    let c_result2 = curry2(v.iter());
    assert_eq!(c_result2, 0);

    let step_curry = foldl!(0=>);
    let step_curry_2 = step_curry(|x,y| x+y);
    assert_eq!(step_curry_2(v.iter()), 6);
}

#[test]
// fn foldr and curry foldr
fn foldr() {
    use kaguya_rs::basic_fn::fun::foldr;
    let v = vec!["Houraisan","Kaguya"];
    let result = foldr("".to_string(), |x,y| x + "<|>" + y, v.iter());
    assert_eq!(result, "<|>Kaguya<|>Houraisan");

    let curry1 = foldr!("This is:".to_string());
    let c_result1 = curry1(|x,&y| x+" "+y, v.iter());
    assert_eq!(c_result1, "This is: Kaguya Houraisan");
    
    let curry2 = foldr!("すごい！".to_string(), |x,&y| x+" "+y);
    let c_result2 = curry2(v.iter());
    assert_eq!(c_result2, "すごい！ Kaguya Houraisan");

    let step_curry = foldr!("楽しい〜".to_string()=>);
    let step_curry_2 = step_curry(|x,&y| x+" "+y);
    assert_eq!(step_curry_2(v.iter()), "楽しい〜 Kaguya Houraisan");
}

#[test]
// fn sum and macro sum
fn sum() {
    use kaguya_rs::basic_fn::fun::sum;
    let result = 10;
    assert_eq!(sum(1..=4), result);
    assert_eq!(sum!(1;4), result);
    assert_eq!(sum!(1,2,3,4), result);
}

#[test]
// macro ls: list comprehension
fn ls() {
    // 1. function on item;iter=>function to check condition
    assert_eq!(ls![|x| x+1; 1..=5 => |x| x&1==0], vec![3,5]);
    // 2. iter=>function to check condition
    assert_eq!(ls![0..=4 => |x| x&1==0], vec![0,2,4]);
    // 3. function on item;iter
    assert_eq!(ls![|x| x*x; 0..=4], vec![0,1,4,9,16]);
    // 4. iter
    assert_eq!(ls![0..=4], vec![0,1,2,3,4]);
}

#[test]
// fn head
fn head() {
    use kaguya_rs::basic_fn::fun::head;
    let empty_vec = Vec::new() as Vec<i8>;
    let vec = vec![1,2,3];

    assert_eq!(None, head(empty_vec.iter()));
    assert_eq!(Some(&1), head(vec.iter()));
}

#[test]
// fn tail
fn tail() {
    use kaguya_rs::basic_fn::fun::tail;
    let empty_vec = Vec::new() as Vec<i8>;
    let vec = vec![1, 2, 3];

    assert_eq!(None, tail(empty_vec.iter()));
    assert_eq!(Some(vec![&2, &3]), tail(vec.iter()));
}

#[test]
// fn last
fn last() {
    use kaguya_rs::basic_fn::fun::last;
    let empty_vec = Vec::new() as Vec<i8>;
    let vec = vec![1, 2, 3];

    assert_eq!(None, last(empty_vec.iter()));
    assert_eq!(Some(&3), last(vec.iter()));
}

#[test]
// fn init
fn init() {
    use kaguya_rs::basic_fn::fun::init;
    let empty_vec = Vec::new() as Vec<i8>;
    let vec = vec![1,2,3];

    assert_eq!(None, init(empty_vec.iter()));
    assert_eq!(Some(vec![&1,&2]), init(vec.iter()));
}

#[test]
// fn and macro skip
fn skip() {
    use kaguya_rs::basic_fn::fun::skip;
    let empty_vec = Vec::new() as Vec<i8>;
    let vec = vec![1,2,3];

    assert_eq!(Vec::new() as Vec<&i8>, skip(1,empty_vec.iter()));
    assert_eq!(vec![&2,&3], skip(1, vec.iter()));

    let curry = skip!(1);
    assert_eq!((Vec::new() as Vec<&i8>), curry(empty_vec.iter()));
    assert_eq!(vec![&2, &3], curry(vec.iter()));
}

#[test]
// fn and macro take
fn take() {
    use kaguya_rs::basic_fn::fun::take;
    let empty_rec = Vec::new() as Vec<i8>;
    let vec = vec![1,2,3];

    assert_eq!(Vec::new() as Vec<&i8>, take(2, empty_rec.iter()));
    assert_eq!(vec![&1,&2], take(2, vec.iter()));

    let curry = take!(2);
    assert_eq!(Vec::new() as Vec<&i8>, curry(empty_rec.iter()));
    assert_eq!(vec![&1,&2], curry(vec.iter()));
}