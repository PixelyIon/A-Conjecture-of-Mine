class Integer
    def divides n
        return (n / self) * self == n
    end

    def digits base: 10
        quotient, remainder = divmod(base)
        return quotient == 0 ? [remainder] : [*quotient.digits(base: base), remainder]
    end

    def sum_digits
        sum = 0

        for n in self.digits
            sum += n
        end

        return sum
    end
end

def get_counterexpls max
    start_time = Time.now

    counterexpls = []
    progress_percent = 0

    for a in (0..max)
        new_progress = a * 100 / max

        if progress_percent != new_progress
            progress_percent = new_progress
            clear()
            puts "LOADING. . . #{new_progress}%"
        end

        for b in (a..max)
            difference = (a + b).sum_digits() - a.sum_digits() - b.sum_digits()

            if !9.divides(difference)
                counterexpls.push([a, b])
            end
        end
    end

    elepsed_time = Time.now - start_time
    clear()
    puts "LOADED. . . #{progress_percent}% in #{elepsed_time}s\n\n"

    return counterexpls
end

def clear
    if /win32|win64|\.NET|windows|cygwin|mingw32/i.match(RUBY_PLATFORM)
       system('cls')
    else
       system('clear')
    end
end

puts "The following script is a simple test for the following conjecture:\n\n"
puts "Let S: N -> N be the sum of the digits of a positive integer."
puts "For all A and B in N, S(A + B) = S(A) + S(B) - 9k, where k is an interger.\n\n"
puts "What value would you like to test the conjecture for?"

user_input = gets

# Check if the input provided by the user in an iteger
if /^(?:-?[1-9]\d*$)|(?:^0)$/.match(user_input)
    max = user_input.to_i
    counterexpls = get_counterexpls(max)

    if counterexpls.length == 0
        puts "The conjecture is proved for all natural numbers smaller or equals to #{max}!"
    else
        puts "The conjecture is disproved! Here are the counter examples:"

        counterexpls_str = ""
        for pair in counterexpls
            counterexpls_str = "#{counterexpls_str}, (#{pair[0]}, #{pair[1]})"
        end

        puts counterexpls_str
    end
else
    puts "'#{user_input.chomp}' isn't an iteger!"
end