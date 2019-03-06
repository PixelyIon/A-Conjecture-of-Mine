// The following script is a simple test for the following conjecture:

// Let S: N -> N be the sum of the digits of a positive integer.
// For all A and B in N, S(A + B) = S(A) + S(B) - 9k, where k is an interger.

const readline = require('readline');

Array.prototype.none = function() {
    return this.length == 0;
}

Number.prototype.getDigits = function() {
    return Array.from(String(this)).map(char => Number(char));
}

Number.prototype.divides = function(n) {
    return (n / this) % 1 == 0;
}

Number.prototype.sumDigits = function() {
    let sum = 0;
    const digits = this.getDigits();

    for (n of digits)
        sum += n;

    return sum;
}

function askQuestion(query) {
    const rl = readline.createInterface({
        input: process.stdin,
        output: process.stdout,
    });

    return new Promise(resolve => rl.question(query, ans => {
        rl.close();
        resolve(ans);
    }))
}

function getCounterExp(max) {
    const starTime = new Date();

    const counterexmpls = [];
    let loadBar = 0;

    for (let a = 1; a <= max; a++) {
        
        // Print the progress on the screen
        const newLoadBar = Math.round(a * 100 / max);
        if (loadBar != newLoadBar) {
            console.clear();
            loadBar = newLoadBar;
            console.log(`LOADING. . . ${loadBar}%`);
        }

        for (b = a; b <= max; b++) {

            // Check if the difference between S(a + b) and (S(a) + S(b)) is a multiple of 9
            const conjectureHolds = (9).divides(
                (a + b).sumDigits() - (a.sumDigits() + b.sumDigits())
            );
            if (!conjectureHolds) counterexmpls.push([a, b]);
        }
    }

    const elepsedTime = (new Date() - starTime) / 1000;
    console.clear();
    console.log(`LOADED. . . ${loadBar}% in ${elepsedTime}s\n`);
    return counterexmpls;
}

console.log(`This script is a simple test for the following conjecture:
    
Let S: N -> N be the sum of the digits of a positive integer.
For all A and B in N, S(A + B) = S(A) + S(B) - 9k, where k is an interger.
`);

askQuestion("What value would you like to test the conjecture for? ").then(ans => {
    if (!isNaN(Number(ans))) {
        const max = Math.round(Number(ans)), counterexmpls = getCounterExp(max);

        if (counterexmpls.none())
            console.log(`The conjecture is proved for all natural numbers smaller or equals to ${max}!`);
        else {
            console.log("The conjecture is disproved! Here are the counter examples:");

            let counterexmplsStr = "";
            for (pair in counterexmpls)
                counterexmpls = `${counterexmplsStr}, (${pair[0]}, ${pair[1]})`;
            
            console.log(counterexmplsStr);
        }
    } else console.log(`'${ans}' is not an interger!`);
});