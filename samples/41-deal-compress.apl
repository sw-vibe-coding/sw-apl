⍝ Deal and compress
3?10
5?5
1 0 1/10 20 30
0 1 0 1/10 20 30 40
1 1 0/'abc'
⍝ The shuffle: deal a permutation of a vector's own length and index
⍝ the vector by it. Deal takes the shape, a one-element vector, as its
⍝ scalar. Last in this file, so the rolls above are not moved.
A←'ABCDE'
A[(⍴A)?⍴A]
)OFF
