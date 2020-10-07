use std::fmt;

use crate::ItemSet;


/// A itemset capable of storing up to 8 items. For test purposes only.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ItemSet8(u8);


pub struct ItemSet8Iter {
	ix: usize,
	data: u8,
}


impl Iterator for ItemSet8Iter {
	type Item = usize;

	fn next(&mut self) -> Option<Self::Item> {
		if self.ix == 8 {
			return None;
		}

		let bit = self.data & (1 << self.ix);
		self.ix += 1;

		if bit == 0 {
			self.next()
		}
		else {
			Some(self.ix - 1)
		}
	}
}


impl<'a> IntoIterator for &'a ItemSet8 {
	type Item = usize;
	type IntoIter = ItemSet8Iter;

	fn into_iter(self) -> Self::IntoIter {
		ItemSet8Iter {
			ix: 0,
			data: self.0
		}
	}
}


impl ItemSet for ItemSet8 {
	fn empty() -> Self { Self(0) }
	fn add(&mut self, item: usize) {
		assert!(item < 8);
		self.0 |= 1 << item;
	}
}


impl fmt::Debug for ItemSet8 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut iter = self.into_iter();

		f.write_str("{")?;

		if let Some(i) = iter.next() {
			write!(f, "{}", i)?;
		}

		for i in iter {
			write!(f, ", {}", i)?;
		}

		f.write_str("}")?;

		Ok(())
	}
}


impl<I> From<I> for ItemSet8
where
	I: Iterator<Item = usize>,
{
	fn from(iter: I) -> Self {
		let mut itemset = Self::empty();

		for i in iter {
			itemset.add(i);
		}

		itemset
	}
}
