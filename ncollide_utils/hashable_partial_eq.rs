use std::hash::{Hash, Writer};
use std::hash::sip::SipState;
use AsBytes;

/// A structure that implements `Eq` and is hashable even if the wrapped data implements only
/// `PartialEq`.
#[derive(PartialEq, RustcEncodable, RustcDecodable, Clone, Rand, Show)]
pub struct HashablePartialEq<T> {
    value: T
}

impl<T> HashablePartialEq<T> {
    /// Creates a new `HashablePartialEq`. This is unsafe because you must be sure that you really
    /// want to transform the wrapped object's partial equality to an equivalence relation.
    pub unsafe fn new(value: T) -> HashablePartialEq<T> {
        HashablePartialEq {
            value: value
        }
    }

    /// Gets the wrapped value.
    pub fn unwrap(self) -> T {
        self.value
    }
}

impl<T: PartialEq> Eq for HashablePartialEq<T> { }

impl<T: AsBytes> Hash for HashablePartialEq<T> {
    #[inline]
    fn hash(&self, state: &mut SipState) {
        state.write(self.value.as_bytes())
    }
}
