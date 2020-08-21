//! My answers to the challenges on chapter 1.

/// 1. Implement, as best as you can, the identity function in your favorite language (or the
/// second favorite, if your favorite language happens to be Haskell).
pub fn id<T>(x: T) -> T {
    x
}

/// 2. Implement the composition function in your favorite language. It takes two functions as
/// arguments and returns a function that is their composition.
pub fn compose<A, B, C, F, G>(f: F, g: G) -> impl FnOnce(A) -> C where
    F: FnOnce(A) -> B,
    G: FnOnce(B) -> C 
{
    |x| g(f(x))
}

/// 4. Is the world-wide web a category in any sense? Are links morphisms?
///     => Yes. Mostly because of composition: if site A links to site B, which
///     in turn links to site C, then you can go from site A to site C.
/// 5. Is Facebook a category, with people as objects and friendships as morphisms?
///     => No, by the same argument as the previous question. If person A is friends
///     with friend B, which in turn is friends with person C, it does not necessarily 
///     imply that person A is friends with person C.
/// 6. When is a directed graph a category?
///     => When there is an edge from every node to itself.

#[test]
fn identity() {
    assert_eq!(1, id(1));
    assert_eq!(Some(2), id(Some(2)));
    assert_eq!(&[1, 2, 3], id(&[1, 2, 3]));
}

#[test]
fn compose_identity() {
    let f = |x| 2*x;

    // Left identity
    assert_eq!(f(2), compose(id, f)(2));

    // Right identity
    assert_eq!(f(2), compose(f, id)(2));
}
