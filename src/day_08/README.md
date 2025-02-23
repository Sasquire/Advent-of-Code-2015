# Day 08: [Matchsticks](https://adventofcode.com/2015/day/8)

This one is pretty simple! For part 1 we can use regex to replace the escaped characters with their actual character (or a dummy character) and subtract 2 for the ending quotes. For part 2 we replace all the characters that need to be escaped with their escaped version and add 2 for the ending quotes.

{% include components/advent_of_code.html
	year = "2015" day = "08"
%}
