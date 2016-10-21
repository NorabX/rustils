use SortingAlgorithmn;
use std::cmp::Ordering;

pub trait Sort {
    fn adv_sort(&mut self, algo: SortingAlgorithmn);
}

impl<T: Ord + Clone> Sort for [T]{
    fn adv_sort(&mut self, algo: SortingAlgorithmn){
        match algo {
            SortingAlgorithmn::Bubble => bubble_sort(self),
            SortingAlgorithmn::Quick => quick_sort(self),
        }
    }
}

pub fn bubble_sort<T: Ord + Clone>(list: &mut [T]){
    for i in 0..list.len() - 1 {
        for j in 0..list.len() - i - 1 {
            if list[j] > list[j + 1] {
                list.swap(j, j + 1);
            }
        }
    }
}

pub fn quick_sort<T: Ord + Clone>(list: &mut [T]){
    if 1 < list.len() {
        let (mut p, mut x) = (0, list.len()-1);
        for _ in 0..list.len() - 1 {
            if list[p] < list[p + 1] {
                list.swap(p + 1, x);
                x -= 1;
            }else{
                list.swap(p, p + 1);
                p += 1;
            }
        }

        quick_sort(&mut list[..p]);
        quick_sort(&mut list[p + 1..]);
    }
}

pub trait SortBy<T: PartialOrd, F: FnMut(&T, &T) -> Ordering> {
    fn adv_sort_by(&mut self, compare: &mut F, algo: SortingAlgorithmn);
}

impl<T: PartialOrd, F: FnMut(&T, &T) -> Ordering> SortBy<T,F> for [T] {
    fn adv_sort_by(&mut self, compare: &mut F, algo: SortingAlgorithmn){
        match algo{
            SortingAlgorithmn::Bubble => bubble_sort_by(self,compare),
            SortingAlgorithmn::Quick => quick_sort_by(self,compare),
        }
    }
}

pub fn bubble_sort_by<T, F: FnMut(&T,&T) -> Ordering>(list: &mut [T], compare: &mut F){
    for i in 0..list.len() - 1 {
        for j in 0..list.len() - i - 1 {
            if compare(&list[j],&list[j + 1]) == Ordering::Greater {
                list.swap(j, j + 1);
            }
        }
    }
}

pub fn quick_sort_by<T, F: FnMut(&T,&T) -> Ordering>(list: &mut [T], compare: &mut F){
    if 1 < list.len() {
        let (mut p, mut x) = (0, list.len()-1);
        for _ in 0..list.len() - 1 {
            if compare(&list[p],&list[p + 1]) == Ordering::Less {
                list.swap(p + 1, x);
                x -= 1;
            }else{
                list.swap(p, p + 1);
                p += 1;
            }
        }

        quick_sort_by(&mut list[..p],compare);
        quick_sort_by(&mut list[p + 1..],compare);
    }
}
