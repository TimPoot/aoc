import sys

# Find where sum == 2020

def part1(puzzle_input: str):
    numbers_list: list[int] = list()

    with open(puzzle_input, "r") as in_file:
        for line in in_file:
            numbers_list.append(int(line))
        
    for number in numbers_list:
        needle: int = (2020 - number)
        if needle in numbers_list:
            return number * needle

def part2(puzzle_input: str):
    numbers_list: list[int] = list()

    with open(puzzle_input, "r") as in_file:
        for line in in_file:
            numbers_list.append(int(line))
        
    for number in numbers_list:
        difference_1: int = (2020 - number)
        numbers_less_than_difference = [n for n in numbers_list if n <= difference_1]

        for i, smaller_number_a in enumerate(numbers_less_than_difference):
            for smaller_number_b in numbers_less_than_difference[i+1:]:
                if number + smaller_number_a + smaller_number_b == 2020:
                    return number * smaller_number_a * smaller_number_b


if __name__ == "__main__":
    puzzle_input = sys.argv[1]
    print(part1(puzzle_input))
    print(part2(puzzle_input))
