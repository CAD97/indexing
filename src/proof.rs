use core::{fmt, marker::PhantomData};

/// `Id<'id>` is _invariant_ w.r.t. `'id`.
///
/// This means that the inference engine is not allowed to
/// grow or shrink `'id` to unify with any other lifetime.
#[derive(Copy, Clone, Default)]
pub(crate) struct Id<'id> {
    id: PhantomData<&'id mut &'id ()>,
}

unsafe impl<'id> Sync for Id<'id> {}
unsafe impl<'id> Send for Id<'id> {}

impl<'id> fmt::Debug for Id<'id> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Id<'id>").finish()
    }
}

/// Length marker for range/index known to not be empty.
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum NonEmpty {}

/// Length marker for range/index of unknown length (may be empty).
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Unknown {}

/// Represents the combination of two proofs `P` and `Q` by a new type `Sum`.
pub trait ProofAdd {
    type Sum;
}

impl<Q> ProofAdd for (NonEmpty, Q) {
    type Sum = NonEmpty;
}
impl<Q> ProofAdd for (Unknown, Q) {
    type Sum = Q;
}
