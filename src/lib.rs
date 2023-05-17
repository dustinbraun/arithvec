pub struct ArithVec<T, const SIZE: usize>
where
    T: Default,
    T: Copy,
    T: std::ops::AddAssign<T>,
    T: std::ops::SubAssign<T>,
    T: std::ops::MulAssign<T>,
    T: std::ops::DivAssign<T>,
{
    array: [T; SIZE],
}

impl<T, const SIZE: usize> Default for ArithVec<T, SIZE>
where
    T: Default,
    T: Copy,
    T: std::ops::AddAssign<T>,
    T: std::ops::SubAssign<T>,
    T: std::ops::MulAssign<T>,
    T: std::ops::DivAssign<T>,
{
    fn default() -> Self {
        ArithVec {
            array: [T::default(); SIZE],
        }
    }
}

impl<T, const SIZE: usize> ArithVec<T, SIZE>
where
    T: Default,
    T: Copy,
    T: std::ops::AddAssign<T>,
    T: std::ops::SubAssign<T>,
    T: std::ops::MulAssign<T>,
    T: std::ops::DivAssign<T>,
{
    pub fn new(array: [T; SIZE]) -> ArithVec<T, SIZE> {
        ArithVec {
            array,
        }
    }
}

impl<T, const SIZE: usize> std::ops::Index<usize> for ArithVec<T, SIZE> 
where
    T: Default,
    T: Copy,
    T: std::ops::AddAssign<T>,
    T: std::ops::SubAssign<T>,
    T: std::ops::MulAssign<T>,
    T: std::ops::DivAssign<T>,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.array[index]
    }
}

impl<T, const SIZE: usize> std::ops::IndexMut<usize> for ArithVec<T, SIZE> 
where
    T: Default,
    T: Copy,
    T: std::ops::AddAssign<T>,
    T: std::ops::SubAssign<T>,
    T: std::ops::MulAssign<T>,
    T: std::ops::DivAssign<T>,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.array[index]
    }
}

impl<T, const SIZE: usize> std::ops::Mul<T> for ArithVec<T, SIZE>
where
    T: Default,
    T: Copy,
    T: std::ops::AddAssign<T>,
    T: std::ops::SubAssign<T>,
    T: std::ops::MulAssign<T>,
    T: std::ops::DivAssign<T>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let mut res = self;
        for x in &mut res.array {
            *x *= rhs;
        }
        res
    }
}

impl<T, const SIZE: usize> std::ops::Div<T> for ArithVec<T, SIZE>
where
    T: Default,
    T: Copy,
    T: std::ops::AddAssign<T>,
    T: std::ops::SubAssign<T>,
    T: std::ops::MulAssign<T>,
    T: std::ops::DivAssign<T>,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        let mut res = self;
        for x in &mut res.array {
            *x /= rhs;
        }
        res
    }
}

impl<T, const SIZE: usize> std::ops::Add<ArithVec<T, SIZE>> for ArithVec<T, SIZE>
where
    T: Default,
    T: Copy,
    T: std::ops::AddAssign<T>,
    T: std::ops::SubAssign<T>,
    T: std::ops::MulAssign<T>,
    T: std::ops::DivAssign<T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut res = self;
        for i in 0..SIZE {
            res[i] += rhs[i];
        }
        res
    }
}

impl<T, const SIZE: usize> std::ops::Sub<ArithVec<T, SIZE>> for ArithVec<T, SIZE>
where
    T: Default,
    T: Copy,
    T: std::ops::AddAssign<T>,
    T: std::ops::SubAssign<T>,
    T: std::ops::MulAssign<T>,
    T: std::ops::DivAssign<T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut res = self;
        for i in 0..SIZE {
            res[i] -= rhs[i];
        }
        res
    }
}

impl<T, const SIZE: usize> std::ops::Mul<ArithVec<T, SIZE>> for ArithVec<T, SIZE>
where
    T: Default,
    T: Copy,
    T: std::ops::AddAssign<T>,
    T: std::ops::SubAssign<T>,
    T: std::ops::MulAssign<T>,
    T: std::ops::DivAssign<T>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut res = self;
        for i in 0..SIZE {
            res[i] *= rhs[i];
        }
        res
    }
}

impl<T, const SIZE: usize> std::ops::Div<ArithVec<T, SIZE>> for ArithVec<T, SIZE>
where
    T: Default,
    T: Copy,
    T: std::ops::AddAssign<T>,
    T: std::ops::SubAssign<T>,
    T: std::ops::MulAssign<T>,
    T: std::ops::DivAssign<T>,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let mut res = self;
        for i in 0..SIZE {
            res[i] /= rhs[i];
        }
        res
    }
}

impl<T, const SIZE: usize> std::ops::MulAssign<T> for ArithVec<T, SIZE>
where
    T: Default,
    T: Copy,
    T: std::ops::AddAssign<T>,
    T: std::ops::SubAssign<T>,
    T: std::ops::MulAssign<T>,
    T: std::ops::DivAssign<T>,
{
    fn mul_assign(&mut self, rhs: T) {
        for x in &mut self.array {
            *x *= rhs;
        }
    }
}

impl<T, const SIZE: usize> std::ops::DivAssign<T> for ArithVec<T, SIZE>
where
    T: Default,
    T: Copy,
    T: std::ops::AddAssign<T>,
    T: std::ops::SubAssign<T>,
    T: std::ops::MulAssign<T>,
    T: std::ops::DivAssign<T>,
{
    fn div_assign(&mut self, rhs: T) {
        for x in &mut self.array {
            *x /= rhs;
        }
    }
}

impl<T, const SIZE: usize> std::ops::AddAssign for ArithVec<T, SIZE>
where
    T: Default,
    T: Copy,
    T: std::ops::AddAssign<T>,
    T: std::ops::SubAssign<T>,
    T: std::ops::MulAssign<T>,
    T: std::ops::DivAssign<T>,
{
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..SIZE {
            self[i] += rhs[i];
        }
    }
}

impl<T, const SIZE: usize> std::ops::SubAssign for ArithVec<T, SIZE>
where
    T: Default,
    T: Copy,
    T: std::ops::AddAssign<T>,
    T: std::ops::SubAssign<T>,
    T: std::ops::MulAssign<T>,
    T: std::ops::DivAssign<T>,
{
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..SIZE {
            self[i] -= rhs[i];
        }
    }
}

impl<T, const SIZE: usize> std::ops::MulAssign for ArithVec<T, SIZE>
where
    T: Default,
    T: Copy,
    T: std::ops::AddAssign<T>,
    T: std::ops::SubAssign<T>,
    T: std::ops::MulAssign<T>,
    T: std::ops::DivAssign<T>,
{
    fn mul_assign(&mut self, rhs: Self) {
        for i in 0..SIZE {
            self[i] *= rhs[i];
        }
    }
}

impl<T, const SIZE: usize> std::ops::DivAssign for ArithVec<T, SIZE>
where
    T: Default,
    T: Copy,
    T: std::ops::AddAssign<T>,
    T: std::ops::SubAssign<T>,
    T: std::ops::MulAssign<T>,
    T: std::ops::DivAssign<T>,
{
    fn div_assign(&mut self, rhs: Self) {
        for i in 0..SIZE {
            self[i] /= rhs[i];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index() {
        let v = ArithVec::new([0u32, 1u32, 2u32, 3u32]);
        assert_eq!(v[0], 0);
        assert_eq!(v[1], 1);
        assert_eq!(v[2], 2);
        assert_eq!(v[3], 3);
    }

    #[test]
    fn test_scalar_mul() {
        let mut v = ArithVec::new([0i32, 1, 2, 3]);
        let s = 2i32;
        v = v * s;
        assert_eq!(v[0], 0);
        assert_eq!(v[1], 2);
        assert_eq!(v[2], 4);
        assert_eq!(v[3], 6);
    }

    #[test]
    fn test_scalar_div() {
        let mut v = ArithVec::new([0i32, 2, 4, 6]);
        let s = 2i32;
        v = v / s;
        assert_eq!(v[0], 0);
        assert_eq!(v[1], 1);
        assert_eq!(v[2], 2);
        assert_eq!(v[3], 3);
    }

    #[test]
    fn test_add() {
        let a = ArithVec::new([0i32, 1, 2, 3]);
        let b = ArithVec::new([1i32, 2, 3, 4]);
        let c = a + b;
        assert_eq!(c[0], 1);
        assert_eq!(c[1], 3);
        assert_eq!(c[2], 5);
        assert_eq!(c[3], 7);
    }

    #[test]
    fn test_sub() {
        let a = ArithVec::new([0i32, 1, 2, 3]);
        let b = ArithVec::new([1i32, 2, 3, 4]);
        let c = a - b;
        assert_eq!(c[0], -1);
        assert_eq!(c[1], -1);
        assert_eq!(c[2], -1);
        assert_eq!(c[3], -1);
    }

    #[test]
    fn test_mul() {
        let a = ArithVec::new([0i32, 1, 2, 3]);
        let b = ArithVec::new([1i32, 2, 3, 4]);
        let c = a * b;
        assert_eq!(c[0], 0);
        assert_eq!(c[1], 2);
        assert_eq!(c[2], 6);
        assert_eq!(c[3], 12);
    }

    #[test]
    fn test_div() {
        let a = ArithVec::new([1i32, 4, 9, 16]);
        let b = ArithVec::new([1i32, 2, 3, 4]);
        let c = a / b;
        assert_eq!(c[0], 1);
        assert_eq!(c[1], 2);
        assert_eq!(c[2], 3);
        assert_eq!(c[3], 4);
    }

    #[test]
    fn test_scalar_mul_assign() {
        let mut v = ArithVec::new([0i32, 1, 2, 3]);
        let s = 2i32;
        v *= s;
        assert_eq!(v[0], 0);
        assert_eq!(v[1], 2);
        assert_eq!(v[2], 4);
        assert_eq!(v[3], 6);
    }

    #[test]
    fn test_scalar_div_assign() {
        let mut v = ArithVec::new([0i32, 2, 4, 6]);
        let s = 2i32;
        v /= s;
        assert_eq!(v[0], 0);
        assert_eq!(v[1], 1);
        assert_eq!(v[2], 2);
        assert_eq!(v[3], 3);
    }

    #[test]
    fn test_add_assign() {
        let mut a = ArithVec::new([0i32, 1, 2, 3]);
        let b = ArithVec::new([1i32, 2, 3, 4]);
        a += b;
        assert_eq!(a[0], 1);
        assert_eq!(a[1], 3);
        assert_eq!(a[2], 5);
        assert_eq!(a[3], 7);
    }

    #[test]
    fn test_sub_assign() {
        let mut a = ArithVec::new([0i32, 1, 2, 3]);
        let b = ArithVec::new([1i32, 2, 3, 4]);
        a -= b;
        assert_eq!(a[0], -1);
        assert_eq!(a[1], -1);
        assert_eq!(a[2], -1);
        assert_eq!(a[3], -1);
    }

    #[test]
    fn test_mul_assign() {
        let mut a = ArithVec::new([0i32, 1, 2, 3]);
        let b = ArithVec::new([1i32, 2, 3, 4]);
        a *= b;
        assert_eq!(a[0], 0);
        assert_eq!(a[1], 2);
        assert_eq!(a[2], 6);
        assert_eq!(a[3], 12);
    }

    #[test]
    fn test_div_assign() {
        let mut a = ArithVec::new([1i32, 4, 9, 16]);
        let b = ArithVec::new([1i32, 2, 3, 4]);
        a /= b;
        assert_eq!(a[0], 1);
        assert_eq!(a[1], 2);
        assert_eq!(a[2], 3);
        assert_eq!(a[3], 4);
    }
}
