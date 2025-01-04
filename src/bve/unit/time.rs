use num_traits::{cast, AsPrimitive, Num, NumCast};
use std::ops::{Add, Sub};

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct Time<T>(pub(super) T)
where
    T: 'static + Num + Copy + NumCast + PartialOrd;

impl<T> Time<T>
where
    T: 'static + Num + Copy + NumCast + PartialOrd,
{
    pub(super) fn raw_to_second() -> T {
        cast(1000).unwrap()
    }
    pub fn hours(value: T) -> Self {
        Self(value * cast(60 * 60 * 1000).unwrap())
    }
    pub fn minutes(value: T) -> Self {
        Self(value * cast(60 * 1000).unwrap())
    }
    pub fn seconds(value: T) -> Self {
        Self(value * cast(1000).unwrap())
    }
    pub fn milliseconds(value: T) -> Self {
        Self(value)
    }
    pub fn as_seconds(&self) -> T {
        self.0 / Self::raw_to_second()
    }
    pub fn as_milliseconds(&self) -> T {
        self.0
    }
    pub fn as_<U>(&self) -> Time<U>
    where
        U: Num + Copy + NumCast + PartialOrd + AsPrimitive<T>,
    {
        Time(cast(self.0).unwrap())
    }
}
impl<T> Add for Time<T>
where
    T: 'static + Num + Copy + NumCast + PartialOrd,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
impl<T> Sub for Time<T>
where
    T: 'static + Num + Copy + NumCast + PartialOrd,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct() {
        assert_eq!(1, Time::seconds(1).as_seconds());
        assert_eq!(1., Time::milliseconds(1.).as_milliseconds());
        assert_eq!(1000, Time::seconds(1).as_milliseconds());
        assert_eq!(3600, Time::hours(1).as_seconds());
        assert_eq!(0, Time::milliseconds(999).as_seconds());
        assert_eq!(0.999, Time::milliseconds(999.).as_seconds());
        assert_eq!(90., Time::minutes(1.5).as_seconds());
    }
    #[test]
    fn unit() {
        assert_eq!(Time::seconds(50), Time::milliseconds(50000));
        assert_eq!(Time::minutes(1), Time::seconds(60));
        assert_eq!(Time::hours(4), Time::seconds(4 * 60 * 60));
    }
    #[test]
    fn into() {
        assert_eq!(Time::milliseconds(1), Time::milliseconds(1.).as_());
        assert_eq!(Time::milliseconds(1), Time::milliseconds(1.5).as_());
        assert_eq!(Time::milliseconds(1), Time::milliseconds(1.001).as_());
        assert_eq!(Time::milliseconds(1.), Time::milliseconds(1).as_());
        assert_eq!(Time::milliseconds(10), Time::milliseconds(10).as_());
        assert_eq!(Time::milliseconds(100), Time::milliseconds(100).as_());
    }
    #[test]
    fn add() {
        assert_eq!(Time::seconds(35), Time::seconds(10) + Time::seconds(25));
        assert_eq!(Time::minutes(1.), Time::seconds(20.) + Time::seconds(40.))
    }
    #[test]
    fn sub() {
        assert_eq!(Time::seconds(-15), Time::seconds(10) - Time::seconds(25));
        assert_eq!(Time::minutes(1.5), Time::minutes(2.) - Time::minutes(0.5))
    }
}
