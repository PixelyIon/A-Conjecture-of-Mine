// The following script is a simple test for the following conjecture:

// Let S: N -> N be the sum of the digits of a positive integer.
// For all A and B in N, S(A + B) = S(A) + S(B) - 9k, where k is an interger.

const readline = require('readline');

class Int {
    private val: number;

    private constructor(n: number) {
        this.val = n;
    }

    public add(n: Int): Int {
        return new Int(this.val + n.val);
    }

    public subtrack(n: Int): Int {
        return new Int(this.val - n.val);
    }

    public multiply(n: Int): Int {
        return new Int(this.val * n.val);
    }

    public divide(n: Int): Int {
        return new Int(Math.floor(this.val / n.val));
    }

    public incrementBy(n: Int) {
        this.val += n.val;
    }

    public divides(n: Int): boolean {
        return (n.val / this.val) % 1 == 0;
    }

    public getDigits(): Int[] {
        return Array.from(this.val.toString()).map(char => Int.from(char));
    }

    public sumDigits(): Int {
        let sum = Int.zero();
        const digits = this.getDigits();

        for (const n of digits)
            sum.incrementBy(n);

        return sum;
    }

    public toString(): string {
        return `${this.val}`;
    }

    static from(a: number | string): Int {
        return new Int(Math.round(Number(a)));
    }

    static getRange(a: Int, b: Int): Int[] {
        const start = a.val, end = b.val, range: Int[] = [];

        for (let i = start; i < end; i++) {
            range.push(Int.from(i));
        }

        return range;
    }

    static zero(): Int {
        return new Int(0);
    }

    static one(): Int {
        return new Int(1);
    }
}

function askQuestion(query: string): Promise<string> {
    const rl = readline.createInterface({
        input: process.stdin,
        output: process.stdout,
    });

    return new Promise(resolve => rl.question(query, (ans: string) => {
        rl.close();
        resolve(ans);
    }));
}

function getCounterExp(max: Int) {
    const starTime = new Date(), counterexmpls: [Int, Int][] = [], nine = Int.from(9);
    let loadBar = Int.zero();

    for (const a of Int.getRange(Int.one(), max)) {

        const newLoadBar = a.multiply(Int.from(100)).divide(max);
        if (loadBar != newLoadBar) {
            console.clear();
            loadBar = newLoadBar;
            console.log(`LOADING. . . ${loadBar}%`);
        }

        for (const b of Int.getRange(a, max)) {
            // Check if the difference between S(a + b) and (S(a) + S(b)) is a multiple of 9
            const conjectureHolds = nine.divides(a.add(b).sumDigits().subtrack(a.sumDigits().add(b.sumDigits())));
            if (!conjectureHolds) counterexmpls.push([a, b]);
        }
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

askQuestion("What value would you like to test the conjecture for? ").then(ans => {
    if (!isNaN(Number(ans))) {
        const max = Int.from(ans), counterexmpls = getCounterExp(max);

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