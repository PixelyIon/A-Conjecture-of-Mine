#include <chrono>
#include <iostream>
#include <string>

#define EXIT_FAILURE 1
#define EXIT_SUCCESS 0

#define DEBUG 0
#define DEBUG_F 0

static unsigned int pow10[10] = {
    1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000};

class number {
 private:
  inline unsigned int getlen(unsigned long n) {
    if (n < 100000) {
      if (n < 100) {
        if (n < 10)
          return 1;
        else
          return 2;
      } else {
        if (n < 1000)
          return 3;
        else {
          if (n < 10000)
            return 4;
          else
            return 5;
        }
      }
    } else {
      if (n < 10000000) {
        if (n < 1000000)
          return 6;
        else
          return 7;
      } else {
        if (n < 100000000)
          return 8;
        else {
          if (n < 1000000000)
            return 9;
          else
            return 10;
        }
      }
    }
  }

 public:
  unsigned short* state;
  unsigned int len = 0;
  unsigned int num = 0;
  number(unsigned int n) {
    if (DEBUG_F) std::cout << "CREATE: " << n << "\n";
    this->num = n;
    this->len = this->getlen(n);
    this->state = new unsigned short[this->len]();
    for (unsigned short i = 0; i < this->len; ++i) {
      this->state[i] = static_cast<unsigned short>(n % 10);
      n /= 10;
    }
  }
  ~number() {
    if (DEBUG_F) std::cout << "DEL: " << this->get() << "\n";
    delete[] this->state;
  }
  void set(unsigned int n) {
    if (DEBUG_F) std::cout << "SET: " << n << "\n";
    this->num = n;
    for (unsigned short i = 0; i < this->len; ++i) {
      this->state[i] = static_cast<unsigned short>(n % 10);
      n /= 10;
    }
  }
  bool zero() {
    if (DEBUG_F) std::cout << "ZERO: " << this->get() << "\n";
    for (unsigned int i = 0; i < this->len; i++) {
      if (this->state[i] != 0) return false;
    }
    return true;
  }
  unsigned int get() {
    unsigned int n = 0;
    for (unsigned int i = 0; i < this->len; i++) {
      n += static_cast<unsigned int>(this->state[i] * pow10[i]);
    }

    if (n != this->num) {
      std::cerr << "UNEQUAL: " << n << ", " << this->num << "\n";
      exit(1);
    }
    return n;
  }
  unsigned int sum() {
    if (DEBUG_F) std::cout << "SUM: " << this->get() << "\n";
    unsigned int n = 0;
    for (unsigned int i = 0; i < this->len; i++) {
      n += (this->state[i]);
    }
    return n;
  }
  void operator--() {
    this->num--;
    if (this->state[0] != 0)
      this->state[0]--;
    else {
      for (unsigned int i = 1; i < this->len; i++) {
        if (this->state[i] != 0) {
          this->state[i]--;
          for (unsigned int z = i - 1; 1; z--) {
            this->state[z] = 9;
            if (z == 0) return;
          }
        }
      }
    }
  }
  unsigned int operator+(number* b) {
    if (DEBUG_F) std::cout << "SUM: " << this->get() + b->get() << "\n";
    return (this->get() + b->get());
  }
};

int main(int argc, char** argv) {
  if (argc <= 1) {
    std::cerr << "Specify the interval as an argument";
    return EXIT_FAILURE;
  }
  unsigned int arg = std::stoi(argv[1]);
  number a = number(arg), b = number(arg), c = number(arg);
  auto start = std::chrono::steady_clock::now();
  for (a = number(std::stoul(argv[1])); !a.zero(); --a) {
    for (b.set(a.get()); !b.zero(); --b) {
      if (DEBUG)
        std::cout << "A: " << a.get() << ", B: " << b.get()
                  << ", A_SUM: " << a.sum() << ", B_SUM: " << b.sum() << "\n";
      c.set((a + &b));
      if (((a.sum() + b.sum()) - c.sum()) % 9 != 0) {
        std::cout << "The conjecture was disproved! Here is a counter example: "
                  << a.get() << ", " << b.get() << "\n";
        return EXIT_FAILURE;
      }
    }
  }
  auto end = std::chrono::steady_clock::now();
  std::cout << "The conjecture is proved for all natural numbers smaller or "
               "equals to "
            << argv[1] << ". The following was done in "
            << std::chrono::duration<double, std::milli>(end - start).count()
            << " ms.\n";
  return EXIT_SUCCESS;
}