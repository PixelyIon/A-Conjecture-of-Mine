// This script is a simple test for the following conjecture:

// Let S: N -> N be the sum of the digits of a positive integer.
// For all A and B in N, S(A + B) = S(A) + S(B) - 9k, where k is an interger.

const readline = require("readline");

Number.prototype.sum = function() {
    if (isNaN(this) || !isFinite(this) || !this)
        throw new TypeError

    let parc = Math.abs(this), sum = 0;

    while (parc > 0) {
        sum += parc % 10;
        parc = Math.floor(parc / 10);
    }

    return sum;
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
    for (let a = 1; a <= max; a++)
        for (let b = a; b <= max; b++) {
            const diff = (a + b).sum() - (a.sum() + b.sum());
            if (diff % 9 !== 0) return [a, b];
        }

    return null;
}

console.log("\nThis script is a simple test for the following conjecture:\n");
console.log("Let S: N -> N be the sum of the digits of a positive integer.");
console.log("For all A and B in N, S(A + B) = S(A) + S(B) - 9k, where k is an interger.\n");

ask("What value would you like to test the conjecture for? ").then(ans => {
    if (!isNaN(Number(ans))) {
        console.log("\nLOADING. . .");

        const max = Math.round(Number(ans))
            , starTime = new Date
            , counterexmpl = getCounterExp(max)
            , elepsed = new Date().getTime() - starTime.getTime();

        console.log(`LOADED. . . in ${elepsed}ms\n`);

        if (counterexmpl === null) console.log(`The conjecture is proved for all natural numbers smaller or equals to ${max}!`);
        else console.log(`The conjecture is disproved! Here's a counterexample: (${counterexmpl[0]}, ${counterexmpl[1]})`);
        
    } else console.log(`\n'${ans}' is not a natural number!`);
});