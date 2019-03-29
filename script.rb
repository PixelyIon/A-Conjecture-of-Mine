# This script is a simple test for the following conjecture:

# Let S: N -> N be the sum of the digits of a positive integer.
# For all A and B in N, S(A + B) = S(A) + S(B) - 9k, where k is an interger.

class Integer
    def divides(n)
        return n % self == 0
    end

    def digits(base: 10)
        (quotient, remainder) = divmod(base)
        return quotient == 0 ? [remainder] : [*quotient.digits(base: base), remainder]
    end

    def sum_digits
        return self.digits.inject(0, &:+)
    end
end

def get_counterexpls(max)
    counterexpls = []
    for a in (0..max)
        for b in (a..max)
            difference = (a + b).sum_digits - a.sum_digits - b.sum_digits

            if !9.divides(difference)
                counterexpls.push([a, b])
            end
        end
    end

    return counterexpls
end

puts "\nThis script is a simple test for the following conjecture:\n\n"
puts "Let S: N -> N be the sum of the digits of a positive integer."
puts "For all A and B in N, S(A + B) = S(A) + S(B) - 9k, where k is an integer.\n\n"
puts "What value would you like to test the conjecture for?"

user_input = gets
puts "\nLOADING. . ."

if /^\d+$/.match(user_input.chomp)
    max = user_input.chomp.to_i
    start = Time.now
    counterexpls = get_counterexpls(max)

    elepsed = Time.now - start
    puts "LOADED. . . in #{elepsed.round(1)}s\n\n"

    if counterexpls.length == 0
        puts "The conjecture is proved for all natural numbers smaller or equals to #{max}!"
    else
        puts "The conjecture is disproved! Here are the counter examples:"
        puts counterexpls.join(", ")
    end
else
    puts "'#{user_input.chomp}' isn't a natural number!"
end