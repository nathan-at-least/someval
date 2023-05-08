#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Some2<A, B> {
    A(A),
    B(B),
    AB(A, B),
}
use Some2::*;

impl<A, B> Some2<A, B> {
    pub fn try_from_options(a: Option<A>, b: Option<B>) -> Option<Self> {
        match (a, b) {
            (None, None) => None,
            (Some(a), None) => Some(A(a)),
            (None, Some(b)) => Some(B(b)),
            (Some(a), Some(b)) => Some(AB(a, b)),
        }
    }

    pub fn as_ref(&self) -> Some2<&A, &B> {
        match self {
            A(a) => A(a),
            B(b) => B(b),
            AB(a, b) => AB(a, b),
        }
    }

    pub fn a(self) -> Option<A> {
        let (opta, _) = self.into();
        opta
    }

    pub fn b(self) -> Option<B> {
        let (_, optb) = self.into();
        optb
    }
}

impl<A, B> TryFrom<(Option<A>, Option<B>)> for Some2<A, B> {
    type Error = &'static str;

    fn try_from((opta, optb): (Option<A>, Option<B>)) -> Result<Self, Self::Error> {
        Self::try_from_options(opta, optb).ok_or("no value of any accepted type present")
    }
}

impl<A, B> From<(A, B)> for Some2<A, B> {
    fn from((a, b): (A, B)) -> Self {
        AB(a, b)
    }
}

impl<A, B> From<Some2<A, B>> for (Option<A>, Option<B>) {
    fn from(sp: Some2<A, B>) -> (Option<A>, Option<B>) {
        match sp {
            A(a) => (Some(a), None),
            B(b) => (None, Some(b)),
            AB(a, b) => (Some(a), Some(b)),
        }
    }
}
