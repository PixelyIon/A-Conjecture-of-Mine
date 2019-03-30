#include <stdio.h>
#include <time.h>

// This program is a simple test for the following conjecture:

// Let S: N -> N be the sum of the digits of a positive integer.
// For all A and B in N, S(A + B) = S(A) + S(B) - 9k, where k is an integer.

int sum_digits(unsigned n)
{
    int parc = n;
    int sum = 0;

    while (parc > 0)
    {
        sum += parc % 10;
        parc /= 10;
    }

    return sum;
}

int main()
{
    printf("\nThis program is a simple test for the following conjecture:\n");
    printf("Let S: N -> N be the sum of the digits of a positive integer.\n");
    printf("For all A and B in N, S(A + B) = S(A) + S(B) - 9k, where k is an integer.\n");
    printf("\nWhat value would you like to test the conjecture for?");

    unsigned MAX = 0;
    scanf_s("%u", &MAX);

    printf("\nLOADING. . .");
    clock_t start = clock(), end;

    for (int a = 0; a <= MAX; a++)
        for (int b = a; b <= MAX; b++)
            if ((sum_digits(a + b) - (sum_digits(a) +sum_digits(b))) % 9 != 0)
            {
                end = clock();
                printf("\nLOADED. . . in %ums\n", (unsigned)((end - start) * 1000 / CLOCKS_PER_SEC));
                printf("\nThe conjecture is disproved! Here's a counterexample: (%u, %u)", a, b);

                return 0;
            }


    end = clock();
    printf("\nLOADED. . . in %ums\n", (unsigned)((end - start) * 1000 / CLOCKS_PER_SEC));
    printf("\nThe conjecture is proved for all natural numbers smaller or equals to %u!", MAX);

    return 0;
}