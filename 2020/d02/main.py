import sys
import re

def part1(puzzle_input: str):
    regex = r"^(\d+?)\-(\d+) (.{1}): (.*)$"
    valid_counter = 0

    with open(puzzle_input, "r") as in_file:
        for line in in_file:
            l_bound, u_bound, char, password = re.findall(regex, line)[0]

            occurences = password.count(char)
            if occurences >= int(l_bound) and occurences <= int(u_bound):
                valid_counter += 1

    return valid_counter
        

def part2(puzzle_input: str):
    regex = r"^(\d+?)\-(\d+) (.{1}): (.*)$"
    valid_counter = 0

    with open(puzzle_input, "r") as in_file:
        for line in in_file:
            pos_1, pos_2, char, password = re.findall(regex, line)[0]

            is_pos1_correct = password[int(pos_1) - 1] == char
            is_pos2_correct = password[int(pos_2) - 1] == char

            if is_pos1_correct != is_pos2_correct:
                valid_counter += 1

    return valid_counter

if __name__ == "__main__":
    puzzle_input = sys.argv[1]
    print(f"Solution p1: {part1(puzzle_input)}")
    print(f"Solution p2: {part2(puzzle_input)}")
