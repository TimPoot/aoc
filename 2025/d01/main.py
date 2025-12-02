import sys
import re

def part1(puzzle_input: str):
    return "See the excel file"

def part2(puzzle_input: str):
    regex = r"(\D)(\d+)"
    current_pos = 50
    zeros_passed = 0

    with open(puzzle_input, "r") as in_file:
        for line in in_file:
            direction, distance = re.findall(regex, line)[0]
            dir_parsed = 1 if direction == "R" else -1
            dis_parsed = int(distance)

            new_pos = current_pos + (dir_parsed * dis_parsed)
            passed_values = range(current_pos, new_pos, dir_parsed)
            passed_values_mod = list(map(lambda x: x % 100, passed_values))
            zeros_passed += passed_values_mod.count(0)
            current_pos = new_pos

    return zeros_passed


if __name__ == "__main__":
    puzzle_input = sys.argv[1]
    print(f"Solution p1: {part1(puzzle_input)}")
    print(f"Solution p2: {part2(puzzle_input)}")