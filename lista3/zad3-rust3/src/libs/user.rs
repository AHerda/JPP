use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use rand::Rng;

use crate::libs::dhsetup::{DHSetup, P};

pub struct User<'a, T>
where
	T: Add<Output = T>
		+ AddAssign
		+ Sub<Output = T>
		+ SubAssign
		+ Mul<Output = T>
		+ MulAssign
		+ Div<Output = T>
		+ DivAssign
		+ From<u64>
		+ Copy,
{
	secret: u64,
	setup: &'a DHSetup<T>,
	key: Option<T>,
}

impl<'a, T> User<'a, T>
where
	T: Add<Output = T>
		+ AddAssign
		+ Sub<Output = T>
		+ SubAssign
		+ Mul<Output = T>
		+ MulAssign
		+ Div<Output = T>
		+ DivAssign
		+ From<u64>
		+ Copy,
{
	pub fn new(setup: &'a DHSetup<T>) -> Self {
		let secret: u64 = rand::thread_rng().gen_range(1..P);
		let key = None;
		User {
			secret,
			setup,
			key,
		}
	}

	pub fn getPublicKey(&self) -> T {
		DHSetup::power(self.setup.getGenerator(), self.secret)
	}

	pub fn setKey(&mut self, key: T) {
		self.key = Some(DHSetup::power(key, self.secret));
	}

	pub fn encrypt(&self, message: T) -> T {
		if let Some(key) = self.key {
			message * key
		} else {
			panic!("Key is not set");
		}
	}

	pub fn decrypt(&self, cipher: T) -> T {
		if let Some(key) = self.key {
			cipher / key
		} else {
			panic!("Key is not set");
		}
	}
}
