use num::traits::Signed;

// https://cmcenroe.me/2016/08/22/rust-extending-type.html
// http://xion.io/post/code/rust-extension-traits.html

/*
Add some convenience functions for vector slices via extention traits
*/
pub trait ArrExt<T> {
    fn min_val(&self) -> &T; // calculates the minimum value of the slice
    fn max_val(&self) -> &T; // calculates the maximum value of the slcie
    fn abs(&self) -> Vec<T>;
    fn clamp_zero(&self) -> Vec<T>;

    // inplace functions
    fn abs_inplace(&mut self);
}

/*
- vector slices seem to be equal to or treated like native arrays, hence we are implementing our extension trait for [T] (and not for &[T] as I thought)
- we only want to implement these functions for vectors of ordered types (i.e. numbers) -> PartialOrd ensures that we have <, =<, > and >=
- plus we want wo ensure that the abs function exists, we can use the Signed trait from the num crate for that
*/
impl<T: PartialOrd + Signed + Copy> ArrExt<T> for [T] {  
    fn min_val(&self) -> &T {
        let min_val = self.iter().fold(None, |min, x| match min {
            None => Some(x),
            Some(y) => Some(if x < y { x } else { y })
        }).unwrap();

        min_val
    }

    fn max_val(&self) -> &T {
        let max_val = self.iter().fold(None, |max, x| match max {
            None => Some(x),
            Some(y) => Some(if x > y { x } else { y })
        }).unwrap();

        max_val
    }

    fn abs(&self) -> Vec<T> {
        let abs_wave: Vec<T> = self.iter().map(|val| val.abs()).collect();  
        abs_wave
    }

    // BLOG
    // why the hell is there no trait bound for primitive types??
    // shit only works when we have Copy trait (what is copy vs clone anyways 0o)
    // need to use *val to dereference the shizzle
    fn clamp_zero(&self) -> Vec<T> {
        let clamped_wave: Vec<T> = self.into_iter().map(|val| if *val > T::zero() { *val } else { T::zero() }).collect();
        clamped_wave
    }

    fn abs_inplace(&mut self) {
        for val in self.iter_mut() {
            *val = val.abs();
        }
    }
}
