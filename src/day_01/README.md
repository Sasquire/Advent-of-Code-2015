# Day 01: [Not Quite Lisp](https://adventofcode.com/2015/day/1)

This problem is pretty simple. Part 1 is count the number of parenthesis and return `count('(') - count(')')`. Part 2 requires keeping track of where you are and breaking out of a loop whenever `count('(') < count(')')`.

{% include components/advent_of_code.html
	year = "2015" day = "01"
%}
