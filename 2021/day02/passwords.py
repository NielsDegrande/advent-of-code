import os
import re

if __name__ == "__main__":
    with open(f"{os.path.dirname(__file__)}/data.txt") as file_:
        data = file_.read().splitlines()

    valid_old = 0
    valid_new = 0

    for line in data:
        parsed = re.split(r"^(\d*)-(\d*) (\D): (\D*)$", line)[1:-1]

        count = parsed[3].count(parsed[2])
        start = int(parsed[0])
        end = int(parsed[1])

        # If letter has min <= x <= max occurrences, then valid.
        if count >= start and count <= end:
            valid_old += 1

        # If letter shows up on either position start or end, then valid.
        if (
            parsed[3][start - 1] == parsed[2]
            and parsed[3][end - 1] != parsed[2]
        ) or (
            parsed[3][start - 1] != parsed[2]
            and parsed[3][end - 1] == parsed[2]
        ):
            valid_new += 1
            print(line)

    print(valid_old)
    print(valid_new)
