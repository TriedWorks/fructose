use crate::operators::Operator;

pub trait Associative: PartialEq { }

pub trait Commutative { }

pub trait Distributive { }

pub trait Total { }

pub trait Identity<Op: Operator> {
    fn identity() -> Self;
}

pub trait Invertible<Op: Operator>: Sized {
    fn inverse(&self) -> Self;
    fn invert(&mut self) {
        *self = self.inverse();
    }
}