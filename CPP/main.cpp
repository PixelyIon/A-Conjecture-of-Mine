#include <chrono>
#include <iostream>
#include <string>
#include <thread>
#include <map>
#include <vector>

#define EXIT_FAILURE 1
#define EXIT_SUCCESS 0

std::map<unsigned int, unsigned int> sum_cache;

inline unsigned int sum_digits(unsigned int n) {
  unsigned int sum = 0;
  while (n != 0) {
    sum += (n % 10);
    n /= 10;
  }
  return sum;
}

void get_ce(unsigned int a, unsigned int m) {
  unsigned int ad;
  for (; a != m; --a) {
    ad = sum_cache[a];
    for (unsigned int b = a; b != 0; --b) {
      unsigned int r = (ad + sum_cache[b] - sum_cache[a + b]);
      switch (r) {
        case 0:
        case 9:
        case 18:
        case 27:
        case 36:
        case 45:
          break;
        default:
          if (r % 9 == 0) break;
          std::cout
              << "The conjecture was disproved! Here is a counter example: "
              << a << ", " << b << "\n";
          exit(EXIT_FAILURE);
          break;
      }
    }
  }
}

int main(int argc, char** argv) {
  if (argc <= 1) {
    std::cerr << "Specify the interval as an argument";
    return EXIT_FAILURE;
  }
  int i = std::thread::hardware_concurrency() * 2;
  unsigned int a = std::stoi(argv[1]) / i;
  std::vector<std::thread> threads;

  for (int z = 0; z != 2 * (a + 1); z++) {
    sum_cache[z] = sum_digits(z);
  }
  auto start = std::chrono::steady_clock::now();
  for (; i; i--) threads.push_back(std::thread(get_ce, (a * i), (a * (i - 1))));
  for (std::thread& thread : threads) thread.join();
  auto end = std::chrono::steady_clock::now();
  std::cout << "The conjecture is proved for all natural numbers smaller or "
               "equals to "
            << argv[1] << ". The following was done in "
            << std::chrono::duration<double, std::milli>(end - start).count()
            << " ms.\n";
  return EXIT_SUCCESS;
}