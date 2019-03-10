# The following script is a simple test for the following conjecture:

# Let S: N -> N be the sum of the digits of a positive integer.
# For all A and B in N, S(A + B) = S(A) + S(B) - 9k, where k is an interger.

import time
from functools import reduce

def digits(n):
    return map(lambda c: int(c), list(str(n)))

def sum_digits(n):
    return reduce((lambda a, b: a + b), digits(n))

def get_counterexmpls(n):
    counterexmpls = []

    for a in range(n + 1):
        for b in range(a, n + 1):
            diff = sum_digits(a + b) - (sum_digits(a) + sum_digits(b))

            if not diff % 9 == 0:
                counterexmpls.append((a, b))
    
    return counterexmpls    


print("\nThis script is a simple test for the following conjecture:\n")
print("Let S: N -> N be the sum of the digits of a positive integer.")
print("For all A and B in N, S(A + B) = S(A) + S(B) - 9k, where k is an interger.\n")
user_input = input("What value would you like to test the conjecture for? ")
print("\nLOADING. . .")

try:
    maximum = abs(int(user_input))

    start = time.time()
    counterexmpls = get_counterexmpls(maximum)
    
    elepsed = time.time() - start
    print("LOADED. . . in {:.1f}s\n".format(elepsed))

    if len(counterexmpls) == 0:
        print("The conjecture is proved for all natural numbers smaller or equals to {}!".format(maximum))
    else:
        print("The conjecture is disproved! Here are the counter examples:")
        print(", ".join(map(lambda pair: "({}, {})".format(pair[0], pair[1]), counterexmpls)))
except:
    print("'{}' isn't a natural number!".format(user_input))
