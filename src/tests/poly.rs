use crate::poly::*;
use ff::Field;
use pairing::bls12_381::*;

#[test]
fn test_poly_eval() {
    let zero = Fr::zero();
    let one = Fr::one();
    let mut two = Fr::one();
    two.add_assign(&one);
    let mut three = Fr::one();
    three.add_assign(&two);

    // 0 -> x
    assert_eq!(vec![one, zero], evaluate(&vec![zero]));
    // 1 -> x+1
    assert_eq!(vec![one, one], evaluate(&vec![one]));

    // 1, 0 -> (x+1) * x = x^2 + x
    assert_eq!(vec![one, one, zero], evaluate(&vec![one, zero]));
    // 1, 1 -> (x+1) * (x+1) = x^2 + 2x + 1
    assert_eq!(vec![one, two, one], evaluate(&vec![one, one]));
    // 2, 0 -> (x+2) * x = x^2 + 2x
    assert_eq!(vec![one, two, zero], evaluate(&vec![zero, two]));
    assert_eq!(vec![one, two, zero], evaluate(&vec![two, zero]));
    // 1, 1, 1 -> (x+1) * (x+1) * (x+1) = x^3 + 3x^2 + 3x + 1
    assert_eq!(vec![one, three, three, one], evaluate(&vec![one, one, one]));
}
