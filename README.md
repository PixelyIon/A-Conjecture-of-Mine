# A Conjecture of Mine
An exercise on _polyglossy_. The same problem solved on multiple languages.

## Problem
Let  <img src="https://latex.codecogs.com/gif.latex?S:\mathbb{N}&space;\mapsto&space;\mathbb{N}"/> be the sum of the digits of a positive integer.

Prove that <img src="https://latex.codecogs.com/gif.latex?\forall&space;a,&space;b&space;\in\mathbb{N}&space;:&space;S_{a&space;&plus;&space;b}&space;=&space;S_a&space;&plus;&space;S_b&space;&plus;&space;9&space;k,&space;k&space;\in&space;\mathbb{Z}"/>.

## Performance
The conjecture was [proved by exhaustion](https://en.wikipedia.org/wiki/Proof_by_exhaustion) for the interval <img src="https://latex.codecogs.com/gif.latex?[0;10^4]"/> in multiple language implementations. The performance of each language was then avaliated as the following:

|Language      |Milliseconds|
|--------------|------------|
|**Rust**      |80          |
|**C**         |271         | 
|**JavaScript**|656         |
|**Ruby**      |9850        |
|**Python**    |16032       |
|**Haskell**   |28365       |
