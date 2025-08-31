bounding_box
============

A lightweight library for handling 2D bounding boxes / bounding rectangles.

[`BoundingBox`]: https://docs.rs/bounding_box/0.1.1/bounding_box/struct.BoundingBox.html
[`ToBoundingBox`]: https://docs.rs/bounding_box/0.1.1/bounding_box/trait.ToBoundingBox.html

A [minimum two-dimensional bounding box / bounding rectangle](https://en.wikipedia.org/wiki/Minimum_bounding_rectangle) describes the extents of an entity (shape, point set, line, ...) or a collection thereof in x-y coordinates. 

Bounding boxes are very useful in computational geometry. For example, if the bounding boxes of two entities don't intersect, the entities themselves also don't intersect. This property can be used to short-circuit intersection algorithms. In a similar fashion, they can be used as a first stage of an algorithm which checks if an entity contains a point. 

Another use case is to find the minimum space required for displaying an entity on a rectangular monitor. By comparing the bounding box to the actually available monitor space, scaling factors can be obtained so the entire entity can be shown on the monitor at once.

This library offers a struct [`BoundingBox`] and a trait [`ToBoundingBox`] which can be used to obtain bounding boxes of types which implement it. The [`BoundingBox`] type has various methods to e.g. calculate its dimensions, find its center, transform it, unite it with other [`BoundingBox`] instances, find intersections between [`BoundingBox`] instances and many more ...

The full API documentation is available at [https://docs.rs/bounding_box/0.1.1/bounding_box/](https://docs.rs/bounding_box/0.1.1/bounding_box/).

As an example, the following code snippet shows how a [`BoundingBox`] can be used with a `Circle` type:

```rust
use bounding_box::{BoundingBox, ToBoundingBox};

struct Circle {
    center: [f64; 2],
    radius: f64
}

impl ToBoundingBox for Circle {
    fn bounding_box(&self) -> BoundingBox {
        return BoundingBox::new(self.center[0] - self.radius, 
                                self.center[0] + self.radius,
                                self.center[1] - self.radius, 
                                self.center[1] + self.radius);
    }
}

let c1 = Circle {center: [0.0, 0.0], radius: 1.0};
let c2 = Circle {center: [0.0, 2.0], radius: 1.0};
let c3 = Circle {center: [0.0, 2.0], radius: 2.0};

// ===============================================================
// Intersection

/// This is an incomplete example of an intersection algorithm
fn circles_intersect(left: &Circle, right: &Circle) -> &'static str {
    let bb_l = left.bounding_box();
    let bb_r = right.bounding_box();

    // Short-circuit the evaluation here
    if !(bb_l.intersects(&bb_r)) {
        return "The circles definitely don't intersect!";
    }

    // Implement the detailed (expensive) algorithm here
    // ...

    return "The circles might intersect ...";
}

assert_eq!(circles_intersect(&c1, &c2), "The circles definitely don't intersect!");
assert_eq!(circles_intersect(&c1, &c3), "The circles might intersect ...");

// ===============================================================
// Contains a point

/// This is an incomplete example of a containment check algorithm
fn circle_contains_point(c: &Circle, pt: [f64; 2]) -> &'static str {
    if !c.bounding_box().contains_point(pt) {
        return "The point is not within the circle!";
    }

    // Implement the detailed (expensive) algorithm here
    // ...

    return "The point might be within the circle ...";
}

assert_eq!(circle_contains_point(&c1, [5.0, 1.0]), "The point is not within the circle!");
assert_eq!(circle_contains_point(&c1, [0.0, 0.5]), "The point might be within the circle ...");

// ===============================================================
// Find the common bounding box of all circles

// Using an iterator
let bb_common_iter = BoundingBox::from_bounded_entities([&c1, &c2, &c3].into_iter()).expect("iterator has at least one element");
assert_eq!(bb_common_iter.xmin(), -2.0);
assert_eq!(bb_common_iter.xmax(), 2.0);
assert_eq!(bb_common_iter.ymin(), -1.0);
assert_eq!(bb_common_iter.ymax(), 4.0);

// Alternatively, the bounding box could also be found by manually uniting the individual bounding boxes
let bb_common_man = c1.bounding_box().union(&c2.bounding_box().union(&c3.bounding_box()));
assert_eq!(bb_common_man, bb_common_iter);
```

The following code snippet shows how to find the smallest bounding box containing a given vertex set.

```rust
use bounding_box::BoundingBox;

let bb_all_pts = BoundingBox::from_vertices([[-1.0, -1.0], [1.0, 1.0]].into_iter()).expect("iterator has at least one element");
assert_eq!(bb_all_pts.xmin(), -1.0);
assert_eq!(bb_all_pts.xmax(), 1.0);
assert_eq!(bb_all_pts.ymin(), -1.0);
assert_eq!(bb_all_pts.ymax(), 1.0);
```

# Feature flags

All features are disabled by default.

## Serialization and deserialization

Bounding boxes can be serialized and deserialized using the [serde](https://crates.io/crates/serde) crate.

This functionality is gated behind the **serde** feature flag.

## Tolerances

Some methods of [`BoundingBox`] are gated behind the **approx** feature flag. Enabling this flag adds
the [approx](https://crates.io/crates/approx) crate as a dependency. The gated methods are prefixed with `approx_`
and are variants of other methods which habe absolute and ULPs (units of least precision) 
tolerances as additional arguments. For example, `approx_contains_point` is the tolerance 
variant of `contains_point` and checks if a given point is *approximately* in the bounding box.