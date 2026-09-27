⍝ Number bases with decode and encode. Decode, the down tack, reads
⍝ a vector of digits in the radix on its left and gives one number.
⍝ The same four digits, read in base 2, 8, 10 and 16:
2⊥1 0 1 1
8⊥1 0 1 1
10⊥1 0 1 1
16⊥1 0 1 1
⍝ Encode, the up tack, is the inverse: a radix vector and a number
⍝ give the digits, one per element of the radix. 200 in binary,
⍝ and back again:
2 2 2 2 2 2 2 2⊤200
2⊥2 2 2 2 2 2 2 2⊤200
⍝ In octal:
8 8 8⊤200
⍝ Hexadecimal needs digits above 9. APL\360 has no format function,
⍝ so the digits index a vector of the characters that show them.
⍝ Each digit is 0 to 15, and the index origin is 1, hence the 1+.
16 16⊤200
HEX←'0123456789ABCDEF'
HEX[1+16 16⊤200]
HEX[1+16 16 16 16⊤48879]
⍝ Reading hex back is dyadic iota, the position of each character,
⍝ less the origin. Forget the 1 and the digits are one too big:
16⊥HEX⍳'BEEF'
16⊥(HEX⍳'BEEF')-1
⍝ What decode is really for is a mixed radix, which is no number
⍝ base at all. Hours, minutes and seconds, to seconds and back:
24 60 60⊥2 15 30
24 60 60⊤8130
⍝ A 0 at the front of the radix takes whatever is left over, here
⍝ the whole days in 200000 seconds:
0 24 60 60⊤200000
⍝ Pounds, shillings and pence: 20 shillings to the pound and 12
⍝ pence to the shilling.
20 12⊥3 7
0 20 12⊤1000
⍝ The traps. A radix vector too short for the number loses the high
⍝ digits without a word: 13 is 1101 in binary, and three places
⍝ give 101.
2 2 2⊤13
2 2 2 2⊤13
⍝ And the hex step depends on the index origin. At origin 0 the 1+
⍝ is one too many, and every digit comes out one high:
)ORIGIN 0
HEX[1+16 16⊤200]
HEX[16 16⊤200]
)ORIGIN 1
⍝ Encode of a negative number with a radix of 2s gives its two's
⍝ complement, the way a machine word holds it: ¯1 in 8 bits.
(8⍴2)⊤¯1
)OFF
