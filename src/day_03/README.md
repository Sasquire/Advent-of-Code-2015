# Day 03: [Perfectly Spherical Houses in a Vacuum](https://adventofcode.com/2015/day/3)

Give me a point class, and I shall plot the world. This is the first appearance of my [Point2d class](./../space/) (closely related to my Space2d class). It is just a generic point class that I have built up after doing a few Advent-of-Code's.

For part 1 I simply kept track of everywhere I had been in a HashSet. Rust's [scan method](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.scan) of iterators is pretty nifty here! It allows you to transform an iterator into another iterator, like `map`, but contains an internal state, like `fold`. Combined these allow you to transform an iterator of instructions into an iterator of states.

For part 2, unfortunately `array_chunks` is currently nightly-only. I would either have to use [Itertools](https://github.com/rust-itertools/itertools) to chunk the array or do it myself. Once chunked into flesh santa and robot santa, we just combine the positions with the same method from part 1.

{% include components/advent_of_code.html
	year = "2015" day = "03"
%}
