use std::ops::Add;
use rand::{
    distributions::{Distribution, Standard},
    prelude::*,
};
use std::fmt::{self, Formatter};


struct A;
struct B;
struct C;
struct D;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Color{
    Red, Yellow, Blue
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Red => write!(f, "🔴"),
            Self::Yellow => write!(f, "🟡"),
            Self::Blue => write!(f, "🔵"),
        }
    }
}

impl Color {
    fn next(self) -> Self {
        match self {
            Self::Red => Self::Yellow,
            Self::Yellow=> Self::Blue,
            Self::Blue => Self::Red,
        }
    }
}

impl Distribution<Color> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Color {
        let i : u32 = rng.gen();
        match i % 3 {
            0 => Color::Red,
            1 => Color::Yellow,
            2 => Color::Blue,
            _ => unreachable!()
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Status(Color, Color, Color, Color);

impl fmt::Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {} {}", self.0, self.1, self.2, self.3)
    }
}

impl Distribution<Status> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Status {
        Status(rng.gen(), rng.gen(), rng.gen(), rng.gen())
    }
}

impl Add<A> for Status {
    type Output = Status;
    fn add(self, _: A) -> Status {
        Status(self.0.next(), self.1.next(), self.2.next(), self.3)
    }
}

impl Add<B> for Status {
    type Output = Status;
    fn add(self, _: B) -> Status {
        Status(self.0, self.1.next(), self.2, self.3.next())
    }
}

impl Add<C> for Status {
    type Output = Status;
    fn add(self, _: C) -> Status {
        Status(self.0, self.1.next(), self.2.next(), self.3)
    }
}

impl Add<D> for Status {
    type Output = Status;
    fn add(self, _: D) -> Status {
        Status(self.0.next(), self.1, self.2, self.3.next())
    }
}

#[derive(Copy, Clone, Debug)]
enum Times {
    Zero, One, Two
}

impl From<i32> for Times {
    fn from(value: i32) -> Self {
        match value {
            0 => Times::Zero,
            1 => Times::One,
            2 => Times::Two,
            _ => panic!("out of bound")
        }
    }
}

impl Iterator for Times {
    type Item = ();

    fn next(&mut self) -> Option<()> {
        match self {
            Times::Zero => None,
            Times::One => {
                *self = Times::Zero;
                Some(())
            }
            Times::Two => {
                *self = Times::One;
                Some(())
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Solution {
    a: Times,
    b: Times,
    c: Times,
    d: Times,
}

impl Add<Solution> for Status {
    type Output = Status;

    fn add(mut self, rhs: Solution) -> Self::Output {
        for _ in rhs.a {
            self = self + A;
        }
        for _ in rhs.b {
            self = self + B;
        }
        for _ in rhs.c {
            self = self + C;
        }
        for _ in rhs.d {
            self = self + D;
        }
        self
    }
}

fn resolve(src: Status, tgt: Status) -> Solution {
    for i in 0 .. 3 * 3 * 3 * 3 {
        let a = (i % 3).into();
        let b = (i / 3 % 3).into();
        let c = (i / 3 / 3 % 3).into();
        let d = (i / 3 / 3 / 3 % 3).into();
        let s = Solution { a, b, c, d};
        if src + s == tgt {
            return s;
        }
    }
    panic!()
}

fn main() {
    let x: Status = random();
    let y: Status = random();
    let s = resolve(x, y);
    println!("resolve {x:?} to {y:?}: {s:?}");

    let mut v = x;
    println!("{x} => {y}");
    for _ in s.a {
        v = v + A;
        println!("A: {v}")
    }
    for _ in s.b {
        v = v + B;
        println!("B: {v}")
    }
    for _ in s.c {
        v = v + C;
        println!("C: {v}")
    }
    for _ in s.d {
        v = v + D;
        println!("D: {v}")
    }

}
