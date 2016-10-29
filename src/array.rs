use random::in_range;

pub trait ArrayUtils{
    fn swap(&mut self, a: usize, b: usize) -> bool;
    fn shuffle(&mut self);
}

impl<T: Ord + Copy> ArrayUtils for [T] {
    fn swap(&mut self, a: usize, b: usize) -> bool {
        if a == b { return true }
        else{
            if a > self.len()-1 || b > self.len()-1 { return false }
            else{
                let temp = self[a];
                self[a] = self[b];
                self[b] = temp;
                true
            }
        }
    }

    fn shuffle(&mut self) {
        for _ in 0..self.len() {
            let a = in_range(0,self.len()-1);
            let b = in_range(0,self.len()-1);
            self.swap(a,b);
        }
    }
}
