
use super::*;

#[test]
fn test_constructor() {
    let three = vectors::ThreeVector::new(1.0, 2.0, 3.0);
    
    let x = three.x;
    let y = three.y;
    let z = three.z;

    assert_eq!((x, y, z), (1., 2., 3.))
}

#[test]
fn test_dot_product() {
    let v1 = vectors::ThreeVector::new(1.0, 2.0, 3.0);
    let v2 = vectors::ThreeVector::new(1.0, 2.0, 3.0);

    assert_eq!(v1.dot(v2), 14.0)
}

#[test]
fn test_mul() {
    let three = vectors::ThreeVector::new(1.0, 2.0, 3.0);
    let mul = three * 2.0;
    let (x1, y1, z1) = (mul.x, mul.y, mul.z);
    
    let (x2, y2, z2) = (2.0, 4.0, 6.0);
    let cmp = vectors::ThreeVector::new(x2, y2, z2);

    assert_eq!(mul, cmp);
    assert_eq!((x1, y1, z1), (x2, y2, z2));

}

#[test]
fn test_sub() {
    let v1 = vectors::ThreeVector::new(1.0, 2.0, 3.0);
    let v2 = vectors::ThreeVector::new(1.0, 2.0, 3.0);

    assert_eq!(v1 - v2, vectors::ThreeVector::zeros());
}

#[test]
fn test_cross() {
    let v1 = vectors::ThreeVector::new(1.0, 2.0, 3.0);
    let v2 = vectors::ThreeVector::new(4.0, 5.0, 6.0);

    assert_eq!(v1.cross(v2), vectors::ThreeVector::new(-3.0, 6.0, -3.0))

}
