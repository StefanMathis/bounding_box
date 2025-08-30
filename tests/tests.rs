use bounding_box::{BoundingBox, ToBoundingBox};

#[test]
fn test_intersects() {
    let bb1 = BoundingBox::new(0.0, 1.0, 0.0, 1.0);
    let bb2 = BoundingBox::new(-0.5, 0.5, -0.5, 1.5);
    assert!(bb1.intersects(&bb2));

    let bb1 = BoundingBox::new(0.0, 1.0, 0.0, 1.0);
    let bb2 = BoundingBox::new(-0.5, 0.5, -0.5, 1.5);
    assert!(bb1.intersects(&bb2));

    let bb1 = BoundingBox::new(0.0, 1.0, 0.0, 1.0);
    let bb2 = BoundingBox::new(0.2, 0.8, std::f64::NEG_INFINITY, 0.5);
    assert!(bb1.intersects(&bb2));

    let bb1 = BoundingBox::new(0.0, 1.0, 0.0, 1.0);
    let bb2 = BoundingBox::new(0.2, 0.8, 0.5, std::f64::INFINITY);
    assert!(bb1.intersects(&bb2));

    let bb1 = BoundingBox::new(0.0, 1.0, 0.0, 1.0);
    let bb2 = BoundingBox::new(0.2, 0.8, std::f64::NEG_INFINITY, std::f64::INFINITY);
    assert!(bb1.intersects(&bb2));

    let bb1 = BoundingBox::new(0.0, 1.0, 0.0, 1.0);
    let bb2 = BoundingBox::new(std::f64::NEG_INFINITY, std::f64::INFINITY, 0.2, 0.8);
    assert!(bb1.intersects(&bb2));
}

#[test]
fn test_contains() {
    let bb1 = BoundingBox::new(0.0, 1.0, 0.0, 1.0);
    let bb2 = BoundingBox::new(-0.5, 1.5, -0.5, 1.5);
    assert!(bb2.contains(&bb1));
    assert!(!bb1.contains(&bb2));

    let bb1 = BoundingBox::new(0.0, 1.0, 0.0, 1.0);
    let bb2 = BoundingBox::new(0.2, 1.0, 0.2, 0.8);
    assert!(bb1.contains(&bb2));
}

#[test]
fn test_touches() {
    // bb1 touches bb2
    let bb1 = BoundingBox::new(0.0, 1.0, 0.0, 1.0);
    let bb2: BoundingBox = BoundingBox::new(1.0, 2.0, 0.0, 1.0);
    assert!(bb2.touches(&bb1));
    assert!(bb1.touches(&bb2));

    // bb1 intersects bb2
    let bb1 = BoundingBox::new(0.0, 1.0, 0.0, 1.0);
    let bb2: BoundingBox = BoundingBox::new(0.8, 2.0, 0.0, 1.0);
    assert!(!bb2.touches(&bb1));
    assert!(!bb1.touches(&bb2));
}

#[test]
fn test_is_finite() {
    let bb = BoundingBox::new(0.0, 1.0, 0.0, 1.0);
    assert!(bb.is_finite());

    let bb = BoundingBox::new(0.0, 1e14, 0.0, 1.0);
    assert!(bb.is_finite());

    let bb = BoundingBox::new(0.0, std::f64::INFINITY, 0.0, 1.0);
    assert!(!bb.is_finite());

    let bb = BoundingBox::new(std::f64::NEG_INFINITY, 1.0, 0.0, 1.0);
    assert!(!bb.is_finite());

    let bb = BoundingBox::new(-10.0, 1.0, std::f64::NEG_INFINITY, 1.0);
    assert!(!bb.is_finite());

    let bb = BoundingBox::new(-10.0, 1.0, 2.0, std::f64::INFINITY);
    assert!(!bb.is_finite());
}

#[test]
fn test_impl_to_bounding_box() {
    struct Dummy;
    impl ToBoundingBox for Dummy {
        fn bounding_box(&self) -> BoundingBox {
            return BoundingBox::new(0.0, 1.0, 0.0, 1.0);
        }
    }

    let dummy = Dummy {};
    let test_bb = BoundingBox::new(0.0, 1.0, 0.0, 1.0);

    let bb = dummy.bounding_box();
    assert_eq!(bb, test_bb);

    // Use reference
    let bb = BoundingBox::from(&dummy);
    assert_eq!(bb, test_bb);

    // Consuming
    let bb = BoundingBox::from(dummy);
    assert_eq!(bb, test_bb);
}

#[test]
fn test_contains_point() {
    let bb = BoundingBox::new(0.0, 1.0, 0.0, 1.0);
    assert!(bb.contains_point([0.0, 0.0]));
    assert!(!bb.contains_point([2.0, 0.0]));
    assert!(!bb.contains_point([2.0, 2.0]));
    assert!(bb.contains_point([0.7, 0.9]));
}

#[test]
fn test_from_nalgebra() {
    use nalgebra::{Point2, Vector2};

    let bb = BoundingBox::new(0.0, 1.0, 0.0, 1.0);
    assert!(bb.contains_point(Point2::new(0.0, 0.0)));
    assert!(bb.contains_point(Vector2::new(0.0, 0.0)));
}
