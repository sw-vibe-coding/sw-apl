⍝ Set operations. APL\360 has no primitive for union, intersection,
⍝ difference or unique: they are built from membership, compression
⍝ and the index generator, as its programs built them.
A←3 1 4 1 5 9 2 6
B←2 7 1 8 2 8
⍝ The index generator takes the shape of a vector, which is a
⍝ one-element vector. Everything below turns on that.
⍴A
⍳⍴A
⍝ Intersection: what of A is also in B. A's own repeats stay -- it has
⍝ two 1s -- as they do in every idiom here until the nub removes them.
(A∊B)/A
⍝ Difference: what of A is not in B.
(~A∊B)/A
⍝ Union: A, then what of B is not already in A. B's own repeats come
⍝ along -- B has two 8s -- so for a set, take the nub of the result.
U←A,(~B∊A)/B
U
((U⍳U)=⍳⍴U)/U
⍝ The nub, which is unique: keep each element only where it first
⍝ appears -- where its index in A is its own position.
((A⍳A)=⍳⍴A)/A
⍝ And the same on characters.
W←'MISSISSIPPI'
((W⍳W)=⍳⍴W)/W
(W∊'AEIOU')/W
W,(~'PARIS'∊W)/'PARIS'
)OFF
