01687458006

use std::ops::Add;

fn double<T>(x: T) -> T {
    where 
        T: Add<Output = T> + Copy,

        {
            x + x
        }
}