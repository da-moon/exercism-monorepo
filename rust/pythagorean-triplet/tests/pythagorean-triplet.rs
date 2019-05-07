extern crate pythagorean_triplet;

#[test]
fn test_answer() {
    // println!("{:?}",pythagorean_triplet::find());
    assert_eq!(pythagorean_triplet::find(), Some(31875000));
    // assert_eq!(true,true);
}
