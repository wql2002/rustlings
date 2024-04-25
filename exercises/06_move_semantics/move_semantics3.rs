// move_semantics3.rs
//
// Make me compile without adding new lines -- just changing existing lines! (no
// lines with multiple semicolons necessary!)
//
// Execute `rustlings hint move_semantics3` or use the `hint` watch subcommand
// for a hint.


// --------------[ORIGIN]-------------------
// #[test]
// fn main() {
//     let vec0 = vec![22, 44, 66];

//     let mut vec1 = fill_vec(vec0);

//     assert_eq!(vec1, vec![22, 44, 66, 88]);
// }

// fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
//     vec.push(88);

//     vec
// }

// --------------[SOLUTION 1]-------------------
// method: change vec0 to mutable, then pass the reference into the function
//         and return the mutable reference. In `assert_eq`, dereference it as argument.
// #[test]
// fn main() {
//     let mut vec0 = vec![22, 44, 66];

//     let mut vec1 = fill_vec(&mut vec0);

//     assert_eq!(*vec1, vec![22, 44, 66, 88]);
// }

// fn fill_vec(vec: &mut Vec<i32>) -> &mut Vec<i32> {
//     vec.push(88);

//     vec
// }

// --------------[SOLUTION 2]-------------------
// method: only need to change the parameter of fn to be `mut vec`
#[test]
fn main() {
    let vec0 = vec![22, 44, 66];

    let vec1 = fill_vec(vec0);

    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(88);

    vec
}