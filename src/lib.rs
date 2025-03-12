// Used to build for web release
// cargo bundle-licenses --prefer MIT --format toml --output ./pkg/THIRD_PARTY_LICENSES.toml
// wasm-pack build --release --no-pack --no-typescript --target web

// Used to double check no infectious dependencies
// cargo license

mod space {
	pub mod space;
	pub mod point;
}

mod graph {
	pub mod graph;
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
	{ $day:ident, $tests:ident, $part_a:ident, $part_b:ident } => {
		mod $day { pub mod $day; }

		apply_attrib! {
			#![wasm_bindgen]
		
			pub fn $part_a (input: &str) -> String { $day::$day::$part_a(input) }
			pub fn $part_b (input: &str) -> String { $day::$day::$part_b(input) }
		}

		// Test with something like 
		// cargo test day_06 -- --show-output
		#[cfg(test)]
		mod $tests {
			fn setup_test () -> String {
				let file_name = format!("./inputs/{}.txt", stringify!($day));
				let contents = std::fs::read_to_string(&file_name).expect("Input file is not present");
				return contents;
			}

			fn interpret_result (part: usize, answer: String) {
				println!("\t{} part {} returned {}!", stringify!($day), part, answer);
			}

			#[test]
			fn part_1_test () {
				interpret_result(1, crate::$part_a(&setup_test()))
			}
		
			#[test]
			fn part_2_test () {
				interpret_result(2, crate::$part_b(&setup_test()))
			}
		}
	}
}

generate_bindings!(day_01, day_01_tests, day_01_1, day_01_2);
generate_bindings!(day_02, day_02_tests, day_02_1, day_02_2);
generate_bindings!(day_03, day_03_tests, day_03_1, day_03_2);
generate_bindings!(day_04, day_04_tests, day_04_1, day_04_2);
generate_bindings!(day_05, day_05_tests, day_05_1, day_05_2);
generate_bindings!(day_06, day_06_tests, day_06_1, day_06_2);
generate_bindings!(day_07, day_07_tests, day_07_1, day_07_2);
generate_bindings!(day_08, day_08_tests, day_08_1, day_08_2);
generate_bindings!(day_09, day_09_tests, day_09_1, day_09_2);
generate_bindings!(day_10, day_10_tests, day_10_1, day_10_2);
// generate_bindings!(day_11, day_11_tests, day_11_1, day_11_2);
// generate_bindings!(day_12, day_12_tests, day_12_1, day_12_2);
// generate_bindings!(day_13, day_13_tests, day_13_1, day_13_2);
// generate_bindings!(day_14, day_14_tests, day_14_1, day_14_2);
// generate_bindings!(day_15, day_15_tests, day_15_1, day_15_2);
// generate_bindings!(day_16, day_16_tests, day_16_1, day_16_2);
// generate_bindings!(day_17, day_17_tests, day_17_1, day_17_2);
// generate_bindings!(day_18, day_18_tests, day_18_1, day_18_2);
// generate_bindings!(day_19, day_19_tests, day_19_1, day_19_2);
// generate_bindings!(day_20, day_20_tests, day_20_1, day_20_2);
// generate_bindings!(day_21, day_21_tests, day_21_1, day_21_2);
// generate_bindings!(day_22, day_22_tests, day_22_1, day_22_2);
// generate_bindings!(day_23, day_23_tests, day_23_1, day_23_2);
// generate_bindings!(day_24, day_24_tests, day_24_1, day_24_2);
// generate_bindings!(day_25, day_25_tests, day_25_1, day_25_2);
