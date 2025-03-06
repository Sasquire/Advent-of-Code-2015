# Day 10: [Elves Look, Elves Say](https://adventofcode.com/2015/day/10)

I remember in the 6th grade our math teacher had a problem where they wrote the look-and-say sequence on the board and challenged all of us to figure out what came next. The whole class was stumped because we were focused on using addition, subtraction, multiplication, and other math concepts we were taught. It was not a math problem at all but instead a logic one.

The solution here was a bit tricky. Not because of the actual work but understanding Rust's types. It was explained to me that both `parse_input` and `look_and_say` return different *types* but both implement `Iterator`. Therefore I have to wrap the outputs of each with `dyn` which requires a `Box`.

{% include components/advent_of_code.html
	year = "2015" day = "10"
%}
