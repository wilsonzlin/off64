use num::Num;
use std::ops::Bound;
use std::ops::RangeBounds;

pub fn range_to_offset_and_len<N: Num + Copy, R: RangeBounds<N>>(range: R, total_len: N) -> (N, N) {
  let start = match range.start_bound() {
    Bound::Included(&n) => n,
    Bound::Excluded(&n) => n + N::one(),
    Bound::Unbounded => N::zero(),
  };
  let end = match range.end_bound() {
    Bound::Included(&n) => n + N::one(),
    Bound::Excluded(&n) => n,
    Bound::Unbounded => total_len,
  };
  (start, end - start)
}
