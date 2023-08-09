use specs::Component;
use specs::storage::BTreeStorage;
use crate::three::Euler;

#[derive(Component)]
#[storage(BTreeStorage)]
pub struct SpinTarget {
	pub euler: Euler,
}
