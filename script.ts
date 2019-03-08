// The following script is a simple test for the following conjecture:

// Let S: N -> N be the sum of the digits of a positive integer.
// For all A and B in N, S(A + B) = S(A) + S(B) - 9k, where k is an interger.

const readline = require('readline');

function digits(n: number) {
    return Array.from(Math.round(Math.abs(n)).toString()).map(c => parseInt(c));
}

function sum(n: number) {
    return digits(n).reduce((a, b) => a + b);
}

function test(a: number, b: number) {
    return (sum(a + b) - (sum(a) + sum(b))) % 9 == 0;
}

function ask(question: string): Promise<string> {
    const rl = readline.createInterface({
        input: process.stdin,
        output: process.stdout,
    });

    return new Promise(resolve => rl.question(question, (ans: string) => {
        rl.close();
        resolve(ans);
    }));
}

function getCounterExp(max: number) {
    const starTime = new Date(), counterexmpls: [number, number][] = [];
    let loadBar = 0;

    for (let a = 0; a <= max; a++) {

        const newLoadBar = a * 100 / max;
        if (loadBar != newLoadBar) {
            console.clear();
            loadBar = newLoadBar;
            console.log(`LOADING. . . ${loadBar}%`);
        }

        for (let b = a; b <= max; b++)
            if (!test(a, b)) counterexmpls.push([a, b]);
    }

    const elepsedTime = (new Date().getTime() - starTime.getTime()) / 1000;
    console.clear();
    console.log(`LOADED. . . ${loadBar}% in ${elepsedTime}s\n`);
    return counterexmpls;
}

console.log(`This script is a simple test for the following conjecture:
    
Let S: N -> N be the sum of the digits of a positive integer.
For all A and B in N, S(A + B) = S(A) + S(B) - 9k, where k is an interger.
`);

ask("What value would you like to test the conjecture for? ").then(ans => {
    if (!isNaN(parseInt(ans))) {
        const max = parseInt(ans), counterexmpls = getCounterExp(max);

        if (counterexmpls.length == 0)
            console.log(`The conjecture is proved for all natural numbers smaller or equals to ${max}!`);
        else {
            console.log("The conjecture is disproved! Here are the counter examples:");

            let counterexmplsStr = "";
            for (const pair of counterexmpls)
                counterexmplsStr = `${counterexmplsStr}, (${pair[0]}, ${pair[1]})`;
            
            console.log(counterexmplsStr);
        }
    } else console.log(`'${ans}' is not an interger!`);
}).catch(console.error);