#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Some3<A, B, C> {
    A(A),
    B(B),
    C(C),
    AB(A, B),
    AC(A, C),
    BC(B, C),
    ABC(A, B, C),
}
use Some3::*;

impl<A, B, C> Some3<A, B, C> {
    pub fn try_from_options(a: Option<A>, b: Option<B>, c: Option<C>) -> Option<Self> {
        match (a, b, c) {
            (None, None, None) => None,
            (Some(a), None, None) => Some(A(a)),
            (None, Some(b), None) => Some(B(b)),
            (None, None, Some(c)) => Some(C(c)),
            (Some(a), Some(b), None) => Some(AB(a, b)),
            (Some(a), None, Some(c)) => Some(AC(a, c)),
            (None, Some(b), Some(c)) => Some(BC(b, c)),
            (Some(a), Some(b), Some(c)) => Some(ABC(a, b, c)),
        }
    }

    pub fn as_ref(&self) -> Some3<&A, &B, &C> {
        match self {
            A(a) => A(a),
            B(b) => B(b),
            C(c) => C(c),
            AB(a, b) => AB(a, b),
            AC(a, c) => AC(a, c),
            BC(b, c) => BC(b, c),
            ABC(a, b, c) => ABC(a, b, c),
        }
    }

    pub fn a(self) -> Option<A> {
        let (opta, _, _) = self.into();
        opta
    }

    pub fn b(self) -> Option<B> {
        let (_, optb, _) = self.into();
        optb
    }

    pub fn c(self) -> Option<C> {
        let (_, _, optc) = self.into();
        optc
    }
}

impl<A, B, C> TryFrom<(Option<A>, Option<B>, Option<C>)> for Some3<A, B, C> {
    type Error = &'static str;

    fn try_from(
        (opta, optb, optc): (Option<A>, Option<B>, Option<C>),
    ) -> Result<Self, Self::Error> {
        Self::try_from_options(opta, optb, optc).ok_or("no value of any accepted type present")
    }
}

impl<A, B, C> From<(A, B, C)> for Some3<A, B, C> {
    fn from((a, b, c): (A, B, C)) -> Self {
        ABC(a, b, c)
    }
}

impl<A, B, C> From<Some3<A, B, C>> for (Option<A>, Option<B>, Option<C>) {
    fn from(sp: Some3<A, B, C>) -> (Option<A>, Option<B>, Option<C>) {
        match sp {
            A(a) => (Some(a), None, None),
            B(b) => (None, Some(b), None),
            C(c) => (None, None, Some(c)),
            AB(a, b) => (Some(a), Some(b), None),
            AC(a, c) => (Some(a), None, Some(c)),
            BC(b, c) => (None, Some(b), Some(c)),
            ABC(a, b, c) => (Some(a), Some(b), Some(c)),
        }
    }
}
