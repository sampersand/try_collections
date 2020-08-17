use crate::{TryEq, TryPartialEq, TryHash, TryReserveError};
use hashbrown::raw::{RawTable, Bucket};
use std::hash::{BuildHasher, Hasher};
use std::collections::hash_map::RandomState;
use std::borrow::Borrow;

pub type DefaultHashBuilder = RandomState;

/// A hashmap that supports fallible hashing and equality checking.
///
/// All the normal invariants for a [`HashMap`](std::collections::HashMap) are still required
/// such as keys not changing their hash.
pub struct TryHashMap<K, V, S=DefaultHashBuilder> {
	table: RawTable<(K, V)>,
	hash_builder: S
}

impl<K, V> TryHashMap<K, V, DefaultHashBuilder> {
	/// Create an empty `TryHashMap` without allocating anything.
	pub fn new() -> Self {
		Self::default()
	}

	/// Creates a `TryHashMap` with the given starting capacity.
	pub fn with_capacity(capacity: usize) -> Self {
		Self::with_capacity_and_hasher(capacity, DefaultHashBuilder::default())
	}
}

impl<K, V, S> TryHashMap<K, V, S> {
	/// Create an empty `TryHashMap` with the given hash builder.
	pub fn with_hasher(hash_builder: S) -> Self {
		Self { hash_builder, table: RawTable::new() }
	}

	/// Creates a `TryHashMap` with the given starting capacity and hash builder.
	pub fn with_capacity_and_hasher(capacity: usize, hash_builder: S) -> Self {
		Self { hash_builder, table: RawTable::with_capacity(capacity) }
	}

	/// Gets a reference to the hash builder.
	pub fn hasher(&self) -> &S {
		&self.hash_builder
	}

	/// Gets the capacity for `self`.
	pub fn capacity(&self) -> usize {
		self.table.capacity()
	}

	/// Gets the length for `self`.
	pub fn len(&self) -> usize {
		self.table.len()
	}

	/// Checks to see if `self` is empty.
	pub fn is_empty(&self) -> bool {
		self.len() == 0
	}

	/// Clears the table.
	pub fn clear(&mut self) {
		self.table.clear();
	}

	pub fn keys(&self) -> ! {
		todo!()
	}

	pub fn values(&self) -> ! {
		todo!()
	}

	pub fn values_mut(&mut self) -> ! {
		todo!()
	}

	pub fn iter(&self) -> ! {
		todo!()
	}

	pub fn iter_mut(&mut self) -> ! {
		todo!()
	}

	pub fn drain(&mut self) -> ! {
		todo!()
	}

	pub fn drain_filter(&mut self, f: impl FnMut(&K, &mut V) -> bool) -> ! {
		todo!()
	}

	pub fn retain(&mut self, f: impl FnMut(&K, &mut V) -> bool) -> ! {
		todo!()
	}
}


impl<K, V, S: BuildHasher> TryHashMap<K, V, S> {
	fn hash_key<Q: ?Sized + TryHash>(&self, key: &Q) -> Result<u64, <Q as TryHash>::Error> {
		let mut hasher = self.hash_builder.build_hasher();
		key.try_hash(&mut hasher)?;
		Ok(hasher.finish())
	}

	fn bucket_iter<'a, Q: ?Sized + TryHash>(&'a self, key: &Q)
		-> Result<impl Iterator<Item=Bucket<(K, V)>> + 'a, <Q as TryHash>::Error>
	{
		Ok(self.bucket_iter_hash(self.hash_key(key)?))
	}

	fn bucket_iter_hash<'a>(&'a self, hash: u64) -> impl Iterator<Item=Bucket<(K, V)>> + 'a {
		// SAFETY: The returned iterator is explicitly constrained to live for as long as `self`, so
		// we are guaranteed that it won't outlive `self.table`, fulfilling the safety requirement.
		unsafe { self.table.iter_hash(hash) }
	}
}

impl<K, V, S> TryHashMap<K, V, S>
where
	K: TryEq + TryHash,
	<K as TryHash>::Error: From<<K as TryPartialEq>::Error>,
	S: BuildHasher
{
	pub fn reserve(&mut self, additional: usize) {
		todo!()
	}

	pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
		todo!()
	}

	pub fn shrink_to_fit(&mut self)  {
		todo!()
	}

	pub fn shrink_to(&mut self, min_capacity: usize) {
		todo!()
	}

	pub fn entry<Q: ?Sized>(&mut self, key: K) -> Result<u8, <Q as TryHash>::Error>
	where
		K: Borrow<Q>,
		Q: TryHash + TryEq,
		<Q as TryHash>::Error: From<<Q as TryPartialEq>::Error>,
	{
		todo!()
	}

	/// Tries to retrieve the associated value, returning an error if either hashing or comparison
	/// failed.
	pub fn get<Q: ?Sized>(&self, key: &Q) -> Result<Option<&V>, <Q as TryHash>::Error>
	where
		K: Borrow<Q>,
		Q: TryHash + TryEq,
		<Q as TryHash>::Error: From<<Q as TryPartialEq>::Error>,
	{
		Ok(self.get_key_value(key)?.map(|(_, v)| v))
	}

	/// Tries to retrieve the associated value, returning an error if either hashing or comparison
	/// failed.
	pub fn get_mut<Q: ?Sized>(&mut self, key: &Q) -> Result<Option<&mut V>, <Q as TryHash>::Error>
	where
		K: Borrow<Q>,
		Q: TryHash + TryEq,
		<Q as TryHash>::Error: From<<Q as TryPartialEq>::Error>,
	{
		Ok(self.get_key_value_mut(key)?.map(|(_, v)| v))
	}

	/// Tries to retrieve the associated key and its value, returning an error if either hashing or
	/// comparison failed.
	pub fn get_key_value<Q: ?Sized>(&self, key: &Q)
		-> Result<Option<(&K, &V)>, <Q as TryHash>::Error>
	where
		K: Borrow<Q>,
		Q: TryHash + TryEq,
		<Q as TryHash>::Error: From<<Q as TryPartialEq>::Error>,
	{
		for bucket in self.bucket_iter(key)? {
			// SAFETY: These values will not outlive `self.table`, as they're constrained by the
			// return type, so we won't have dangling pointers.
			let (k, v) = unsafe { bucket.as_ref() };
			if key.try_eq(k.borrow())? {
				return Ok(Some((k, v)));
			}
		}

		Ok(None)
	}

	/// Tries to retrieve the associated key and its mutable value, returning an error if either
	/// hashing or comparison failed.
	pub fn get_key_value_mut<Q: ?Sized>(&mut self, key: &Q)
		-> Result<Option<(&K, &mut V)>, <Q as TryHash>::Error>
	where
		K: Borrow<Q>,
		Q: TryHash + TryEq,
		<Q as TryHash>::Error: From<<Q as TryPartialEq>::Error>,
	{
		for bucket in self.bucket_iter(key)? {
			// SAFETY: These values will not outlive `self.table`, as they're constrained by the
			// return type, so we won't have dangling pointers.
			let (ref mut k, ref mut v) = unsafe { bucket.as_mut() };
			if key.try_eq((&*k).borrow())? {
				return Ok(Some((k, v)));
			}
		}

		Ok(None)
	}

	/// Adds a new element to the map, returning its old value (or an error if hashing or comparison
	/// failed.)
	pub fn insert(&mut self, key: K, value: V) -> Result<Option<V>, <K as TryHash>::Error> {
		let hash = self.hash_key(&key)?;
		for bucket in self.bucket_iter_hash(hash) {
			// SAFETY: Neither reference outlives `self`, so no dangling pointers.
			let (ref mut k, ref mut v) = unsafe { bucket.as_mut() };
			if key.try_eq((&*k).borrow())? {
				return Ok(Some(std::mem::replace(v, value)));
			}
		}

		// if it doesn't exist, and we need to resize, resize and also rehash.
		if self.table.capacity() <= self.table.len() {
			let new_table = RawTable::with_capacity(2 * self.table.len() + 1);
			for entry in std::mem::replace(&mut self.table, new_table) {
				let hash = self.hash_key(&entry.0)?;
				self.table.insert_no_grow(hash, entry);
			}
		}

		// insert the newest item
		self.table.insert_no_grow(hash, (key, value));
		Ok(None)
	}

	/// Tries to remove a single element from the map, returning its old value if it existed
	pub fn remove<Q: ?Sized>(&mut self, key: &Q) -> Result<Option<V>, <Q as TryHash>::Error>
	where
		K: Borrow<Q>,
		Q: TryHash + TryEq,
		<Q as TryHash>::Error: From<<Q as TryPartialEq>::Error>,
	{
		Ok(self.remove_entry(key)?.map(|(_, v)| v))
	}

	/// Tries to remove a single entry from the hashmap, returning its old value.
	pub fn remove_entry<Q: ?Sized>(&mut self, key: &Q)
		-> Result<Option<(K, V)>, <Q as TryHash>::Error>
	where
		K: Borrow<Q>,
		Q: TryHash + TryEq,
		<Q as TryHash>::Error: From<<Q as TryPartialEq>::Error>,
	{
		let mut found_bucket: Option<Bucket<(K, V)>> = None;

		for bucket in self.bucket_iter(key)? {
			// SAFETY: the key is only used within the `try_eq` function and will thus be dropped
			// before we remove it.
			if key.try_eq(unsafe { bucket.as_ref().0.borrow() })? {
				found_bucket = Some(bucket);
				break;
			}
		}

		if let Some(bucket) = found_bucket {
			// SAFETY: We know for a fact that the bucket lives within `self` (as we just got it)
			// and that it's unique (because we have a mutable reference to `self`).
			Ok(Some(unsafe { self.table.remove(bucket) }))
		} else {
			Ok(None)
		}
	}
}

impl<K, V> Default for TryHashMap<K, V, DefaultHashBuilder> {
	fn default() -> Self {
		Self::with_hasher(Default::default())
	}
}
