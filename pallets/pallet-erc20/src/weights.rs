use frame_support::weights::Weight;

pub trait WeightInfo {
	fn init() -> Weight;

	fn transfer() -> Weight;

	fn approve() -> Weight;

	fn transfer_from() -> Weight;

	fn mint() -> Weight;

	fn burn() -> Weight;
}

impl WeightInfo for () {
	fn init() -> Weight {
		Weight::from_parts(9_000_000, 0)
	}

	fn transfer() -> Weight {
		Weight::from_parts(13_000_000, 0)
	}

	fn approve() -> Weight {
		Weight::from_parts(11_000_000, 0)
	}

	fn transfer_from() -> Weight {
		Weight::from_parts(20_000_000, 0)
	}

	fn mint() -> Weight {
		Weight::from_parts(12_000_000, 0)
	}

	fn burn() -> Weight {
		Weight::from_parts(14_000_000, 0)
	}
}
