// The following script is a simple test for the following conjecture:

// Let S: N -> N be the sum of the digits of a positive integer.
// For all A and B in N, S(A + B) = S(A) + S(B) - 9k, where k is an interger.

const readline = require('readline');

Number.prototype.digits = function() {
    return Array.from(Math.abs(Math.round(this)).toString()).map(char => Number(char));
}

Number.prototype.sumDigits = function() {
    return this.digits().reduce((a, b) => a + b);
}

function ask(question) {
    const rl = readline.createInterface({
        input: process.stdin,
        output: process.stdout,
    });

    return new Promise(resolve => rl.question(question, ans => {
        rl.close();
        resolve(ans);
    }))
}

function getCounterExp(max) {
    const counterexmpls = [];

    for (let a = 1; a <= max; a++)
        for (let b = a; b <= max; b++) {
            const diff = (a + b).sumDigits() - (a.sumDigits() + b.sumDigits());
            if (diff % 9 !== 0) counterexmpls.push([a, b]);
        }

    return counterexmpls;
}

console.log("\nThis script is a simple test for the following conjecture:\n");
console.log("Let S: N -> N be the sum of the digits of a positive integer.");
console.log("For all A and B in N, S(A + B) = S(A) + S(B) - 9k, where k is an interger.\n");

ask("What value would you like to test the conjecture for? ").then(ans => {
    if (!isNaN(Number(ans))) {
        console.log("\nLOADING. . .");

        const max = Math.round(Number(ans))
            , starTime = new Date
            , counterexmpls = getCounterExp(max)
            , elepsed = Math.round((new Date().getTime() - starTime.getTime()) / 100) / 10;

        console.log(`LOADED. . . in ${elepsed}s\n`);

        if (counterexmpls.length === 0)
            console.log(`The conjecture is proved for all natural numbers smaller or equals to ${max}!`);
        else {
            console.log("The conjecture is disproved! Here are the counter examples:");
            console.log(counterexmpls.map(pair => `(${pair[0]}, ${pair[1]})`).join(", "));
        }
    } else console.log(`\n'${ans}' is not a natural number!`);
});