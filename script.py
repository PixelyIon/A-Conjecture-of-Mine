# The following script is a simple test for the following conjecture:

# Let S: N -> N be the sum of the digits of a positive integer.
# For all A and B in N, S(A + B) = S(A) + S(B) - 9k, where k is an interger.

import sys, os, time
from functools import reduce

# A function for clearing the prompt
clear = (lambda : os.system("cls")) if sys.platform.startswith("win") else (lambda : os.system("clear"))

def digits(n):
    return map(lambda c: int(c), list(str(n)))

def sum_digits(n):
    return reduce((lambda a, b: a + b), digits(n))

def get_counterexmpls(n):
    start = time.time()

    load_bar = 0
    counterexmpls = []

    for a in range(n + 1):
        
        # Print the progress on the screen
        if not round(a * 100 / n) == load_bar:
            load_bar = round(a * 100 / n)
            clear()
            print("LOADING. . . {}%".format(load_bar))

        for b in range(a, n + 1):
            diff = sum_digits(a + b) - (sum_digits(a) + sum_digits(b))

            if not diff % 9 == 0:
                counterexmpls.append((a, b))

    # Mesure the elepsed time
    elepsed = time.time() - start
    print("LOADED. . . in {:.1f}s\n".format(elepsed))
    
    return counterexmpls    


print("\nThe following script is a simple test for the following conjecture:\n")
print("Let S: N -> N be the sum of the digits of a positive integer.")
print("For all A and B in N, S(A + B) = S(A) + S(B) - 9k, where k is an interger.\n")
user_input = input("What value would you like to test the conjecture for? ")

try:
    maximum = abs(int(user_input))
    clear()
    counterexmpls = get_counterexmpls(maximum)

    if len(counterexmpls) == 0:
        print("The conjecture is proved for all natural numbers smaller or equals to {}!".format(maximum))
    else:
        print("The conjecture is disproved! Here are the counter examples:")
        print(", ".join(map(lambda pair: "({}, {})".format(pair[0], pair[1]), counterexmpls)))
except:
    print("'{}' isn't a natural number!".format(user_input))
