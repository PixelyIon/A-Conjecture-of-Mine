# This script is a simple test for the following conjecture:

# Let S: N -> N be the sum of the digits of a positive integer.
# For all A and B in N, S(A + B) = S(A) + S(B) - 9k, where k is an interger.

class Integer
    def sum
        part = self.abs()
        sum = 0

        while part > 0
            d, r = part.divmod(10)
            sum += r
            part = d
        end

        return sum
    end

    def self.sums(max)
        output = []
        
        for i in 0..((max + 1) * 2)
            output << i.sum()
        end

        return output
    end
end

def get_counterexpl(max)
    sums = Integer.sums(max)

    for a in (0..max)
        for b in (a..max)
            diff = sums[a + b] - sums[a] - sums[b]

            if diff % 9 != 0
                return [a, b]
            end
        end
    end

    return nil
end

puts "\nThis script is a simple test for the following conjecture:\n\n"
puts "Let S: N -> N be the sum of the digits of a positive integer."
puts "For all A and B in N, S(A + B) = S(A) + S(B) - 9k, where k is an integer.\n\n"
puts "What value would you like to test the conjecture for?"

max_str = gets.chomp
puts "\nLOADING. . ."

if /^\d+$/.match(max_str)
    max = max_str.chomp.to_i
    start = Time.now()
    counterexpl = get_counterexpl(max)

    elepsed = Time.now() - start
    puts "LOADED. . . in #{(elepsed * 1000).round()}ms\n\n"

    if counterexpl == nil
        puts "The conjecture is proved for all natural numbers smaller or equals to #{max}!"
    else
        puts "The conjecture is disproved! Here's a counterexample: (#{counterexpl[0]}, #{counterexpl[1]})"
    end
else
    puts "'#{max_str}' isn't a natural number!"
end