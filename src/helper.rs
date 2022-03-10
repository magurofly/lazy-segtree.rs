pub trait LazySegtreeHelper {
  type S;
  type F;

  /// binary operation
  fn op(x: &Self::S, y: &Self::S) -> Self::S;

  /// identity element of binary operation
  fn e() -> Self::S;

  /// map
  fn mapping(f: &Self::F, x: &Self::S) -> Self::S;

  /// binary operation on operator monoid
  fn composition(f: &Self::F, g: &Self::F) -> Self::F;

  /// identity element of operator
  fn id() -> Self::F;

  /// (Segment Tree Beats) true when failed to accumulate
  fn is_failed(_: &Self::S) -> bool {
    false
  }
}