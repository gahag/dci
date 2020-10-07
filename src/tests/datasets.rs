use std::collections::HashSet;

use lazy_static::lazy_static;

use crate::{
	DataSet,
	Support
};
use super::{dataset, itemset};


/// Generic type for a test dataset.
pub struct TestDataSet<D: DataSet> {
	pub dataset: D,
	pub result: HashSet<(D::ItemSet, Support)>,
	pub min_sup: usize,
}


lazy_static! {
	pub static ref TOY: TestDataSet<dataset::Matrix::<itemset::ItemSet8>> = TestDataSet {
		dataset: {
			let mut dataset = dataset::Matrix::new(5, 6);

			dataset[(0, 0)] = true;
			dataset[(0, 3)] = true;
			dataset[(0, 5)] = true;

			dataset[(1, 1)] = true;
			dataset[(1, 4)] = true;

			dataset[(2, 0)] = true;
			dataset[(2, 1)] = true;
			dataset[(2, 2)] = true;
			dataset[(2, 4)] = true;
			dataset[(2, 5)] = true;

			dataset[(3, 0)] = true;
			dataset[(3, 3)] = true;

			dataset
		},

		result: [
			([0, 2].iter().copied().into(), 2),
			([0, 3].iter().copied().into(), 2),
			([1, 2].iter().copied().into(), 2),
			([0].iter().copied().into(), 3),
			([2].iter().copied().into(), 5),
		]
			.iter()
			.cloned()
			.collect(),

		min_sup: 2,
	};
}
