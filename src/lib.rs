// cargo bundle-licenses --prefer MIT --format toml --output ./pkg/THIRD_PARTY_LICENSES.toml
// wasm-pack build --release --no-pack --no-typescript --target web

// Used to double check no infectious dependencies
// cargo license

mod space {
	pub mod space;
	pub mod point;
}

use wasm_bindgen::prelude::*;

// https://stackoverflow.com/a/77835585
// Apply the same attribute to all items in the block, but don't compile it as 
// an actual block.
macro_rules! apply_attrib {
	{ #!$attr:tt $($it:item)* } => {
		$(
			#$attr
			$it
		)*
	}
}

// https://doc.rust-lang.org/nightly/std/macro.concat_idents.html
// Waiting on concat_idents to be stable to make this cleaner
macro_rules! generate_bindings {
	{ $day:ident, $part_a:ident, $part_b:ident } => {
		mod $day { pub mod $day; }

		apply_attrib! {
			#![wasm_bindgen]
		
			pub fn $part_a (input: &str) -> String { $day::$day::$part_a(input) }
			pub fn $part_b (input: &str) -> String { $day::$day::$part_b(input) }
		}
	}
}

generate_bindings!(day_01, day_01_1, day_01_2);
generate_bindings!(day_02, day_02_1, day_02_2);
generate_bindings!(day_03, day_03_1, day_03_2);
generate_bindings!(day_04, day_04_1, day_04_2);
generate_bindings!(day_05, day_05_1, day_05_2);