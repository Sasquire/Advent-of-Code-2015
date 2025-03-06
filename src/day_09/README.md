# Day 09: [All in a Single Night](https://adventofcode.com/2015/day/9)

With a good graph library (not a 2d graph but a mathematical graph) traveling salesman becomes easy. Luckily I took the time to write a [graph library](./../graph/). It's not perfect but it works. I wanted to merge traveling salesman into the library and use a [BinaryHeap](https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html) to keep track of all distances but Ord is not implemented for floats which makes this difficult (and I have to make a new type to store in the heap).

Ignoring all that, implementing traveling salesman is painless.

{% include components/advent_of_code.html
	year = "2015" day = "09"
%}
