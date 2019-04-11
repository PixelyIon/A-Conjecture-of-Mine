package main

// This program is a simple test for the following conjecture:

// Let S: N -> N be the sum of the digits of a positive integer.
// For all A and B in N, S(A + B) = S(A) + S(B) - 9k, where k is an interger.

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
	"time"
)

func main() {
	fmt.Println("\nThis program is a simple test for the following conjecture:")
	fmt.Println("\nLet S: N -> N be the sum of the digits of a positive integer.")
	fmt.Println("For all A and B in N, S(A + B) = S(A) + S(B) - 9k, where k is an interger.")
	fmt.Print("\nWhat value would you like to test the conjecture for? ")

	reader := bufio.NewReader(os.Stdin)
	maxStr, _ := reader.ReadString('\n')
	maxStr = strings.TrimSpace(maxStr)

	if max, err := strconv.ParseUint(maxStr, 10, 32); err == nil {
		max := uint(max)

		fmt.Println("\nLOADING. . .")
		tuple, found, elepsed := getCounterexample(max)
		fmt.Printf("LOADED in %dms\n", elepsed.Nanoseconds()/int64(math.Pow10(6)))

		if !found {
			fmt.Printf("\nThe conjecture is proved for all natural numbers smaller or equals to %d!\n", max)
		} else {
			fmt.Printf("\nThe conjecture is disproved! Here's a counterexample: (%d, %d)\n", tuple[0], tuple[1])
		}
	} else {
		fmt.Printf("\n'%s' isn't a natural number!\n", maxStr)
	}
}

func getCounterexample(max uint) (value [2]uint, found bool, elepsed time.Duration) {
	start := time.Now()

	sums := getSums(max)
	for a := uint(0); a <= max; a++ {
		for b := a; b <= max; b++ {
			diff := sums[a+b] - sums[a] - sums[b]

			if diff%9 != 0 {
				return [2]uint{a, b}, true, time.Since(start)
			}
		}
	}

	return [2]uint{0, 0}, false, time.Since(start)
}

func getSums(max uint) []int {
	maxRange := 2 * (max + 1)
	sums := make([]int, maxRange, maxRange)

	for i := uint(0); i < maxRange; i++ {
		sums[i] = sumDigits(i)
	}

	return sums
}

func sumDigits(n uint) int {
	var sum uint

	for {
		if n <= 0 {
			return int(sum)
		}

		sum += n % 10
		n /= 10
	}
}
