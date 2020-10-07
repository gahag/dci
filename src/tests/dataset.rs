use std::{
	marker::PhantomData,
	ops::{Index, IndexMut}
};

use crate::{DataSet, ItemSet};


/// A boolean matrix, for test purposes only. In a real life scenario, one should use a bit
/// matrix.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Matrix<I> {
	data: Box<[bool]>,
	height: usize,
	width: usize,
	phantom: PhantomData<I>,
}


impl<I> Matrix<I> {
	pub fn new(height: usize, width: usize) -> Self {
		let size = height * width;

		Self {
			data: {
				let mut vec = Vec::with_capacity(size);
				vec.resize(size, false);
				vec.into_boxed_slice()
			},

			height,
			width,

			phantom: PhantomData
		}
	}


	pub fn row(&self, ix: usize) -> &[bool] {
		let begin = ix * self.width;
		let end = (ix + 1) * self.width;

		&self.data[begin .. end]
	}
}


impl<I> Index<(usize, usize)> for Matrix<I> {
	type Output = bool;

	fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
		&self.data[i * self.width + j]
	}
}


impl<I> IndexMut<(usize, usize)> for Matrix<I> {
	fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
		&mut self.data[i * self.width + j]
	}
}


impl<I> DataSet for Matrix<I>
where
	I: ItemSet,
	for<'a> &'a I: IntoIterator<Item = usize>
{
	type ItemSet = I;
	type Cover = Box<[bool]>;


	fn items_count(&self) -> usize {
		self.height
	}

	fn transactions_count(&self) -> usize {
		self.width
	}


	fn item_support(&self, item: usize) -> usize {
		self.row(item)
			.iter()
			.filter(|&&b| b)
			.count()
	}

	fn support(&self, itemset: &Self::ItemSet) -> usize {
		self
			.cover(itemset)
			.iter()
			.filter(|&&b| b)
			.count()
	}


	fn supports(&self, item: usize, cover: &Self::Cover) -> bool {
		let item_iter = self.row(item).iter();
		let cover_iter = cover.iter();

		item_iter
			.zip(cover_iter)
			.all(
				|(&a, &b)| (!b) || a
			)
	}

	fn cover(&self, itemset: &Self::ItemSet) -> Self::Cover {
		let length = self.transactions_count();

		let mut cover = {
			let mut vec = Vec::with_capacity(length);
			vec.resize(length, true);
			vec.into_boxed_slice()
		};

		for item in itemset.into_iter() {
			let item_iter = self.row(item).iter().copied();
			let cover_iter = cover.iter_mut();

			for (a, b) in cover_iter.zip(item_iter) {
				*a &= b;
			}
		}

		cover
	}
}
