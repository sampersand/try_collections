use std::hash::{Hash, Hasher};
use std::convert::Infallible;

/// Analagous to [`PartialEq`], but for fallible conversions.
pub trait TryPartialEq<Rhs: ?Sized = Self> {
	/// The error to return.
	type Error;

	/// Try to compare `self` to `rhs`, returning an [`Error`](Self::Error) if it's not possible.
	fn try_eq(&self, rhs: &Rhs) -> Result<bool, Self::Error>;
}

impl<T: PartialEq<Rhs>, Rhs> TryPartialEq<Rhs> for T {
	type Error = Infallible;

	#[inline]
	fn try_eq(&self, rhs: &Rhs) -> Result<bool, Self::Error> {
		Ok(self == rhs)
	}
}


/// Analogous to [`Eq`], but for fallible conversions.
pub trait TryEq : TryPartialEq<Self> {}

impl<T: Eq> TryEq for T {}

/// Try to hash something, returning an error if it's not possible.
pub trait TryHash {
	/// The error to return.
	type Error;

	/// Try to hash `self`, returning an [`Error`](Self::Error) if it's not possible.
	fn try_hash<H: Hasher>(&self, hasher: &mut H) -> Result<(), Self::Error>;
}

impl<T: Hash> TryHash for T {
	type Error = Infallible;

	#[inline]
	fn try_hash<H: Hasher>(&self, hasher: &mut H) -> Result<(), Self::Error> {
		self.hash(hasher);
		Ok(())
	}
}

