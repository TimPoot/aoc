import sys
import re

def part1(puzzle_input: str):
    ranges = []
    range_pattern = r"(\d+)-(\d+)"
    ingredient_pattern = r"^(\d+)$"
    fresh_ingredient_count = 0

    with open(puzzle_input, "r") as in_file:
        input_contents = in_file.read()
        for range_match in re.findall(range_pattern, input_contents):
            ranges.append(tuple(map(lambda x: int(x), range_match)))

        for ingredient in re.findall(ingredient_pattern, input_contents, re.MULTILINE):
            ingredient_int = int(ingredient)
            for range_match in ranges:
                l_bound, u_bound = range_match
                if ingredient_int >= l_bound and ingredient_int <= u_bound:
                    fresh_ingredient_count += 1
                    break

    return fresh_ingredient_count

def part2(puzzle_input: str):
    range_pattern = r"(\d+)-(\d+)"
    ranges_saved = set()

    with open(puzzle_input, "r") as in_file:
        input_contents = in_file.read()
        for range_match in re.findall(range_pattern, input_contents):
            current_range = (int(range_match[0]), int(range_match[1]))
            should_save_range = True
            print(f"LOG current_range: {current_range}")

            for saved_range in ranges_saved:
                # check if current range is entirely within existant range -> skip range
                if saved_range[0] <= current_range[0] and saved_range[1] >= current_range[1]:
                    print(f"LOG entirely within other range, skipping")
                    should_save_range = False
                    break

                # check if current range has lower lower bound but smaller upper bound -> decrease lower bound
                if saved_range[0] > current_range[0] and saved_range[1] >= current_range[1] and saved_range[0] <= current_range[1]:
                    print(f"LOG adjusting lower bound")
                    ranges_saved.remove(saved_range)
                    ranges_saved.add((current_range[0], saved_range[1]))
                    should_save_range = False
                    break

                # check if current range has higher upper bound but larger lower bound -> increase upper bound
                if saved_range[0] <= current_range[0] and saved_range[1] >= current_range[0] and saved_range[1] < current_range[1]:
                    print(f"LOG adjusting upper bound")
                    ranges_saved.remove(saved_range)
                    ranges_saved.add((saved_range[0], current_range[1]))
                    should_save_range = False
                    break
                
                # no overlap, add to set

            if should_save_range:
                print(f"LOG saving to set")
                ranges_saved.add(current_range)

            print(f"LOG saved ranges: {ranges_saved}")

    return ranges_saved

if __name__ == "__main__":
    puzzle_input = sys.argv[1]
    print(f"Solution p1: {part1(puzzle_input)}")
    print(f"Solution p2: {part2(puzzle_input)}")
