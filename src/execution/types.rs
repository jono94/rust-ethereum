
use std::{ cmp, ops };


#[derive(Debug, Copy, Clone)]
pub struct u256 {
    upper: u128,
    lower: u128,
}

impl u256 {
    pub fn zero() -> u256 {
        u256 { upper: 0, lower: 0 }
    }

    pub fn one() -> u256 {
        u256 { upper: 0, lower: 1 }
    }

    pub fn max() -> u256 {
        u256 { upper: u128::MAX, lower: u128::MAX }
    }

    pub fn from_u8(lower: u8) -> u256 {
        u256 { upper: 0, lower: lower as u128 }
    }

    pub fn from_u128(lower: u128) -> u256 {
        u256 { upper: 0, lower }
    }

    pub fn from_u128s(upper: u128, lower: u128) -> u256 {
        u256 { upper, lower }
    }
}

// Arithmetic

impl ops::Add for u256 {
    type Output = Self;
    fn add(self, rhs: u256) -> Self {
        let (lower, overflow) = u128::overflowing_add(self.lower, rhs.lower);
        let mut intermediate_upper: u128 = rhs.upper;
        if overflow {
            intermediate_upper = u128::overflowing_add(intermediate_upper, 1).0;
        }
        let (upper, overflow) = u128::overflowing_add(self.upper, intermediate_upper);
        u256::from_u128s(upper, lower)
    }
}

impl ops::Mul for u256 {
    type Output = Self;
    fn mul(self, rhs: Self) ->  Self {
        // TODO: Improve algorithm
        let mut acc = self;
        let mut i = u256::one(); // set the initial acc to skip the first iteration
        while i < rhs {
            acc = acc + self;
            i = i + u256::one();
        }
        acc
    }
}

impl ops::Sub for u256 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        if self >= rhs {
            // no overflow issues, so do addition with 2s complement of rhs
            let twos_complement_rhs = !rhs + u256::one();
            self + twos_complement_rhs
        } else {
            // overflow issues, so 2s complement subtraction doesn't work properly I guess, so manually handle wrapping
            u256::max() - (rhs - self)
        }
    }
}

impl ops::Rem for u256 {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self {
        // TODO: Improve algorithm
        let mut acc = self;
        while acc >= rhs {
            acc = acc - rhs;
        }
        acc
    }
}

// 0x10s: Comparisons and Bitwise Logic Operations
// eq, neq
impl cmp::PartialEq<u256> for u256 {
    fn eq(&self, other: &Self) -> bool {
        (self.upper == other.upper) && (self.lower == other.lower)
    }
}

// lt, le, gt, ge
impl u256 {
    fn less_than(&self, other: &Self, equal: bool) -> bool {
        if self.upper < other.upper {
            return true;
        } else if self.upper > other.upper {
            return false;
        } else {
            if self.lower < other.lower {
                return true;
            } else if self.lower > other.lower {
                return false;
            } else {
                return equal;
            }
        }
    }
}
impl cmp::PartialOrd for u256 {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        if self.lt(other) {
            return Some(cmp::Ordering::Less);
        } else if self.gt(other) {
            return Some(cmp::Ordering::Greater);
        } else {
            return Some(cmp::Ordering::Equal);
        }
    }

    fn lt(&self, other: &Self) -> bool {
        self.less_than(other, false)
    }

    fn le(&self, other: &Self) -> bool {
        self.less_than(other, true)
    }

    fn gt(&self, other: &Self) -> bool {
        !self.less_than(other, true)
    }

    fn ge(&self, other: &Self) -> bool {
        !self.less_than(other, false)
    }

}

// not operatator (binary inverse)
impl ops::Not for u256 {
    type Output = Self;
    fn not(self) -> Self {
        u256 { upper: !self.upper, lower: !self.lower }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        // lower overflow
        let var1: u256 = u256::from_u128s(0, 1<<127);
        let var2: u256 = u256::from_u128s(0, 1<<127);
        let res: u256 = u256::from_u128s(1, 0);
        assert_eq!(res, var1 + var2);

        // upper overflow
        let var1: u256 = u256::from_u128s(1<<127, 0);
        let var2: u256 = u256::from_u128s(1<<127, 0);
        let res: u256 = u256::from_u128s(0, 0);
        assert_eq!(res, var1 + var2);

        // lower and upper overflow
        let var1: u256 = u256::from_u128s(1<<127, 1<<127);
        let var2: u256 = u256::from_u128s(1<<127, 1<<127);
        let res: u256 = u256::from_u128s(1, 0);
        assert_eq!(res, var1 + var2);
    }

    #[test]
    fn mul() {
        // 10 * 33 = 330
        let var1: u256 = u256::from_u128s(0, 10);
        let var2: u256 = u256::from_u128s(0, 33);
        let res: u256 = u256::from_u128s(0, 330);
        assert_eq!(res, var1 * var2);
    }

    #[test]
    fn sub() {
        // 2 - 1
        let var1: u256 = u256::from_u128s(0, 2);
        let var2: u256 = u256::one();
        let res: u256 = u256::from_u128s(0, 1);
        assert_eq!(res, var1 - var2);
 
        // overflow
        let var1: u256 = u256::zero();
        let var2: u256 = u256::one();
        let res: u256 = u256::from_u128s(u128::MAX, u128::MAX-1);
        assert_eq!(res, var1 - var2);
    }

    #[test]
    fn rem() {
        // 10 % 3 = 1
        let var1: u256 = u256::from_u128s(0, 10);
        let var2: u256 = u256::from_u128s(0, 3);
        let res: u256 = u256::one();
        assert_eq!(res, var1 % var2);

        // 12 % 3 = 0
        let var1: u256 = u256::from_u128s(0, 12);
        let var2: u256 = u256::from_u128s(0, 3);
        let res: u256 = u256::zero();
        assert_eq!(res, var1 % var2);
    }
}
