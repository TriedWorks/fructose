use crate::algebra::helpers::bound::Bounded;
use crate::algebra::helpers::identity::{One, Zero};
use crate::algebra::helpers::mul_add::{MulAdd, MulAddAssign};
use crate::algebra::properties::archimedean::ArchimedeanDiv;
use crate::algebra::properties::general::Ordered;
use crate::algebra::properties::primality::Primality;
use crate::algebra::ring::{IntegerRing, NaturalCommutativeSemiring, SemiEuclideanDomain};
use crate::operators::bit::ClosedBitOps;
use crate::operators::{Additive, ClosedOps, Multiplicative};

// NOTE: right now everything can be used as a natural number, so this needs to be fixed asap.
pub trait Natural:
    NaturalCommutativeSemiring + Bounded + MulAdd + MulAddAssign + Zero + One
{
}

pub trait Integer: IntegerRing + Bounded + MulAdd + MulAddAssign + Zero + One {}

pub trait Bits:
    ClosedBitOps
    + ClosedOps
    + SemiEuclideanDomain
    + Primality
    + ArchimedeanDiv
    + Bounded
    + Zero
    + One
    + Ordered<Additive>
    + Ordered<Multiplicative>
{
}

impl<T> Natural for T where
    T: NaturalCommutativeSemiring + Bounded + MulAdd + MulAddAssign + Zero + One
{
}

impl<T> Integer for T where T: IntegerRing + Bounded + MulAdd + MulAddAssign + Zero + One {}

impl<T> Bits for T where
    T: ClosedBitOps
        + ClosedOps
        + SemiEuclideanDomain
        + Primality
        + ArchimedeanDiv
        + Bounded
        + Zero
        + One
        + Ordered<Additive>
        + Ordered<Multiplicative>
{
}
