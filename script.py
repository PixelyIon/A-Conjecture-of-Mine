# The following script is a simple test for the following conjecture:

# Let S: N -> N be the sum of the digits of a positive integer.
# For all A and B in N, S(A + B) = S(A) + S(B) - 9k, where k is an interger.

import sys
import os
import time

# A function for clearing the prompt
clear = (lambda : os.system("cls")) if sys.platform.startswith("win") else (lambda : os.system("clear"))

def is_multiple_of_nine(n):
    return (n / 9) % 1 == 0

def get_digits(n):
    output = []

    for ch in str(n):
        output.append(int(ch))
    
    return output

def sum_digits(n):
    digits = get_digits(n)
    output = 0

    for d in digits:
        output += d 
    
    return output

def get_counterexmpls(n):
    start_time = time.time()

    load_bar = 0
    counterexmpls = []

    for a in range(n + 1):
        
        # Print the progress on the screen
        if not round(a * 100 / n) == load_bar:
            load_bar = round(a * 100 / n)
            clear()
            print("LOADING. . . %s%%" % load_bar)

        for b in range(a, n + 1):
            difference = sum_digits(a + b) - (sum_digits(a) + sum_digits(b))

            if not is_multiple_of_nine(difference):
                counterexmpls.append([a, b])

    # Mesure the elepsed time
    elepsed_time = time.time() - start_time
    clear()
    print("LOADED. . . %s%% in %ds\n" % (load_bar, elepsed_time))
    
    return counterexmpls    


print("""The following script is a simple test for the following conjecture:

Let S: N -> N be the sum of the digits of a positive integer.
For all A and B in N, S(A + B) = S(A) + S(B) - 9k, where k is an interger.
""")
user_input = input("What value would you like to test the conjecture for? ")

try:
    maximum = abs(int(user_input))
    clear()
    counterexmpls = get_counterexmpls(maximum)

    if len(counterexmpls) == 0:
        print("The conjecture is proved for all natural numbers smaller or equals to %s!" % maximum)
    else:
        print("The conjecture is disproved! Here are the counter examples:")

        counterexmpls_str = ""
        for pair in counterexmpls:
            counterexmpls_str = "%s, (%d, %f)" % (counterexmpls_str, pair[0], pair[1])
        
        print(counterexmpls_str)
except:
    print("'%s' isn't a valid number!" % user_input)
