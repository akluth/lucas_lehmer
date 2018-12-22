extern crate lucas_lehmer;

#[test]
fn m11213_is_a_mersenne_number() {
    // M11213 is a mersenne number, discovered in the year 1963
    let m11213: usize = 11213;
    assert!(lucas_lehmer::is_mersenne(m11213));
}

#[test]
fn m19_is_a_mersenne_number() {
    let m19: usize = 19;
    assert!(lucas_lehmer::is_mersenne(m19));
}

#[test]
fn m1337_is_not_a_mersenne_number() {
    let m1337: usize = 1337;
    assert_ne!(lucas_lehmer::is_mersenne(m1337), true);
}

#[test]
fn two_is_a_mersenne_number() {
    let two: usize = 2;
    assert!(lucas_lehmer::is_mersenne(two));
}
