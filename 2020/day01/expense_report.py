import numpy as np
import os
from itertools import product

EXPENSE_SUM = 2020
NUMBER_OF_ENTRIES = 3

if __name__ == "__main__":
    with open(f"{os.path.dirname(__file__)}/data.txt") as file_:
        data = file_.read().splitlines()
        data = [int(x) for x in data]

    combinations = product(data, repeat=NUMBER_OF_ENTRIES)
    for combination in combinations:
        if sum(combination) == EXPENSE_SUM:
            print(
                f"The answer is multiplying these entries {combination} = "
                f"{np.prod(combination)}"
            )
            break
