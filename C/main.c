#include <stdlib.h>
#include <stdio.h>
#include <math.h>
#include <errno.h>
#include <time.h>

// This program is a simple test for the following conjecture:

// Let S: N -> N be the sum of the digits of a positive integer.
// For all A and B in N, S(A + B) = S(A) + S(B) - 9k, where k is an integer.

enum capacity {None, Single, Tuple};

typedef struct option
{
    enum capacity status;
    unsigned int *value;
}option;

option *none()
{
    option *output = malloc(sizeof(option));
    output->status = None;

    return output;
}

option *single(unsigned int n)
{
    option *output = malloc(sizeof(option));
    output->status = Single;
    output->value = &n;

    return output;
}

option *tuple(unsigned int a, unsigned int b)
{
    option *output = malloc(sizeof(option));
    output->status = Tuple;

    static unsigned int value[2];
    value[0] = a;
    value[1] = b;
    output->value = &value;

    return output;
}

int sum_digits(unsigned int n) {
    int parc = n;
    int sum = 0;

    while (parc > 0)
    {
        sum += parc % 10;
        parc /= 10;
    }

    return sum;
}

option *get_counter_exp(unsigned int max) {
    for (int a = 0; a <= max; a++)
        for (int b = a; b <= max; b++)
            if ((sum_digits(a + b) - (sum_digits(a) +sum_digits(b))) % 9 != 0)
                return tuple(a, b);

    return none();
}

option *parse_int(const char * str) {
    char *end;
    for (long i = strtol(str, &end, 10); str != end;)
    {
        if (errno == ERANGE)
        {
            errno = 0;
            return none();
        }

        return i >= 0 ? single((unsigned int)i) : none();
    }

    return none();
}

char *scan_line(char *line)
{
    int ch;

    if((line = malloc(sizeof(char))) == NULL)
    {
        printf("Unsuccessful allocation");
        exit(1);
    }

    line[0]='\0';

    for(int i = 0; ((ch = getchar())!='\n') && (ch != EOF) ; i++)
    {
        if((line = realloc(line, sizeof(char) * (i + 2))) == NULL)
        {
            printf("Unsuccessful reallocation");
            exit(1);
        }

        line[i] = (char)ch;
        line[i + 1] = '\0'; //inserting null character at the end
    }

    return line;
}

int main() {
    printf("\nThis program is a simple test for the following conjecture:\n");
    printf("Let S: N -> N be the sum of the digits of a positive integer.\n");
    printf("For all A and B in N, S(A + B) = S(A) + S(B) - 9k, where k is an integer.\n");
    printf("\nWhat value would you like to test the conjecture for?");

    const char *MAX_STR = scan_line(NULL);
    unsigned int MAX;
    option *MAX_OPT = parse_int(MAX_STR);

    if (MAX_OPT->status != None)
        MAX = *MAX_OPT->value;
    else
    {
        printf("\n'%s' is not a natural number!", MAX_STR);
        return 0;
    }

    printf("\nLOADING. . .");

    clock_t start, end;
    start = clock();
    const option *counter_expl = get_counter_exp(MAX);
    end = clock();
    printf("\nLOADED. . . in %.1fs\n", (float)(end - start) / CLOCKS_PER_SEC);

    if (counter_expl->status == None)
        printf("\nThe conjecture is proved for all natural numbers smaller or equals to %u!", MAX);
    else printf("\nThe conjecture is disproved! Here is a counter example: (%u, %u)",
            *counter_expl->value,*(counter_expl->value + sizeof(unsigned int)));

    return 0;
}