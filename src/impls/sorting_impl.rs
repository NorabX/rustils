// <editor-fold> # Uses

use std::cmp::Ordering;
use sorting::*;
// </editor-fold>

// <editor-fold> # Impls

// <editor-fold> ## Sort

impl<T: Ord + Clone> Sort for [T] {
    fn adv_sort_mut(&mut self, algo: SortingAlgorithmn) {
        match algo {
            SortingAlgorithmn::Bubble => bubble_sort_mut(self),
            SortingAlgorithmn::Quick => quick_sort_mut(self),
        }
    }
}
// </editor-fold>

// <editor-fold> ## SortBy

impl<T: PartialOrd, F: FnMut(&T, &T) -> Ordering> SortBy<T, F> for [T] {
    fn adv_sort_by_mut(&mut self, compare: &mut F, algo: SortingAlgorithmn) {
        match algo {
            SortingAlgorithmn::Bubble => bubble_sort_by_mut(compare, self),
            SortingAlgorithmn::Quick => quick_sort_by_mut(compare, self),
        }
    }
}
// </editor-fold>

// </editor-fold>
