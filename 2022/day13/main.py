# Using Python to be able to parse and handle types more easily.

from ast import literal_eval
from itertools import zip_longest
from typing import Any, List, Optional

def read_data(path: str) -> List[str]:
    with open(path) as f:
        return f.readlines()

def get_chunk(input: List[Any], chunk_size: int) -> List[Any]:
    for i in range(0, len(input), chunk_size):
        yield input[i:i+chunk_size]

def validate_order(list_1: List[Any], list_2: List[Any]) -> Optional[bool]:
    for left, right in zip_longest(list_1, list_2):
        if left is None:
            return True
        elif right is None:
            return False
        elif (type(left) == int) and (type(right) == int):
            if left < right:
                return True
            elif left > right:
                return False
            else:
                continue
        elif (type(left) == list) and (type(right) == list):
            result = validate_order(left, right)
            if result is not None:
                return result
        else:
            if type(left) == int:
                result = validate_order([left], right)
                if result is not None:
                    return result
            elif type(right) == int:
                result = validate_order(left, [right])
                if result is not None:
                    return result
    return None

def solve_part_1(data: List[str]) -> int:
    index = 1
    solution = 0

    for lines in get_chunk(data, 3):
        list_1 = literal_eval(lines[0].strip())
        list_2 = literal_eval(lines[1].strip())
        if validate_order(list_1, list_2):
            solution += index
        index +=1
    return solution

def solve_part_2(data: List[str]) -> int:
    packets = [[2], [6]]
    for line in data:
        if line != "\n":
            packets.append(literal_eval(line.strip()))

    all_passed = False
    while not all_passed:
        all_passed = True
        for i in range(0, len(packets) - 1):
            if not validate_order(packets[i], packets[i + 1]):
                # Swap packets. Effectively a bubble sort.
                original_packet = packets[i]
                packets[i] = packets[i + 1]
                packets[i + 1] = original_packet
                all_passed = False

    # Find indexes of relevant positions.
    solution = 1
    for i, packet in enumerate(packets):
        if packet in [[2], [6]]:
            solution *= i + 1

    return solution

if __name__ == "__main__":
    data = read_data("data/input.txt")
    print(f"Part 1: {solve_part_1(data)}")
    print(f"Part 2: {solve_part_2(data)}")
