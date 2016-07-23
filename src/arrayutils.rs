use randomutils;

pub fn swap_i8(ary: &mut [i8], a: usize, b: usize) -> bool{
    if a == b { return true }
    else{
        if a > ary.len()-1 || b > ary.len()-1 { return false }
        else{
            let temp = ary[a];
            ary[a] = ary[b];
            ary[b] = temp;
            true
        }
    }
}

pub fn swap_i16(ary: &mut [i16], a: usize, b: usize) -> bool{
    if a == b { return true }
    else{
        if a > ary.len()-1 || b > ary.len()-1 { return false }
        else{
            let temp = ary[a];
            ary[a] = ary[b];
            ary[b] = temp;
            true
        }
    }
}

pub fn swap_i32(ary: &mut [i32], a: usize, b: usize) -> bool{
    if a == b { return true }
    else{
        if a > ary.len()-1 || b > ary.len()-1 { return false }
        else{
            let temp = ary[a];
            ary[a] = ary[b];
            ary[b] = temp;
            true
        }
    }
}

pub fn swap_i64(ary: &mut [i64], a: usize, b: usize) -> bool{
    if a == b { return true }
    else{
        if a > ary.len()-1 || b > ary.len()-1 { return false }
        else{
            let temp = ary[a];
            ary[a] = ary[b];
            ary[b] = temp;
            true
        }
    }
}

pub fn swap_u8(ary: &mut [u8], a: usize, b: usize) -> bool{
    if a == b { return true }
    else{
        if a > ary.len()-1 || b > ary.len()-1 { return false }
        else{
            let temp = ary[a];
            ary[a] = ary[b];
            ary[b] = temp;
            true
        }
    }
}

pub fn swap_u16(ary: &mut [u16], a: usize, b: usize) -> bool{
    if a == b { return true }
    else{
        if a > ary.len()-1 || b > ary.len()-1 { return false }
        else{
            let temp = ary[a];
            ary[a] = ary[b];
            ary[b] = temp;
            true
        }
    }
}

pub fn swap_u32(ary: &mut [u32], a: usize, b: usize) -> bool{
    if a == b { return true }
    else{
        if a > ary.len()-1 || b > ary.len()-1 { return false }
        else{
            let temp = ary[a];
            ary[a] = ary[b];
            ary[b] = temp;
            true
        }
    }
}

pub fn swap_u64(ary: &mut [u64], a: usize, b: usize) -> bool{
    if a == b { return true }
    else{
        if a > ary.len()-1 || b > ary.len()-1 { return false }
        else{
            let temp = ary[a];
            ary[a] = ary[b];
            ary[b] = temp;
            true
        }
    }
}

pub fn swap_f32(ary: &mut [f32], a: usize, b: usize) -> bool{
    if a == b { return true }
    else{
        if a > ary.len()-1 || b > ary.len()-1 { return false }
        else{
            let temp = ary[a];
            ary[a] = ary[b];
            ary[b] = temp;
            true
        }
    }
}

pub fn swap_f64(ary: &mut [f64], a: usize, b: usize) -> bool{
    if a == b { return true }
    else{
        if a > ary.len()-1 || b > ary.len()-1 { return false }
        else{
            let temp = ary[a];
            ary[a] = ary[b];
            ary[b] = temp;
            true
        }
    }
}

pub fn swap_usize(ary: &mut [usize], a: usize, b: usize) -> bool{
    if a == b { return true }
    else{
        if a > ary.len()-1 || b > ary.len()-1 { return false }
        else{
            let temp = ary[a];
            ary[a] = ary[b];
            ary[b] = temp;
            true
        }
    }
}

pub fn swap_isize(ary: &mut [isize], a: usize, b: usize) -> bool{
    if a == b { return true }
    else{
        if a > ary.len()-1 || b > ary.len()-1 { return false }
        else{
            let temp = ary[a];
            ary[a] = ary[b];
            ary[b] = temp;
            true
        }
    }
}

pub fn shuffle_i8(ary: &mut [i8]){
    for _ in 0..ary.len() {
        let a = randomutils::rand_usize_range(0,ary.len()-1);
        let b = randomutils::rand_usize_range(0,ary.len()-1);
        swap_i8(ary,a,b);
    }
}
