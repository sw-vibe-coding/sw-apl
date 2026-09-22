⍝ Equal and not-equal compare characters as well as numbers. Every
⍝ other scalar function takes numbers only.
W←'MISSISSIPPI'
W='S'
⍝ How many of a letter, and where they are.
+/W='S'
(W='S')/⍳⍴W
⍝ Everything but the letter.
(W≠'S')/W
⍝ Two words, letter by letter, and whether they match altogether.
'CAT'='COT'
'CAT'∧.='CAT'
'CAT'∧.='COT'
⍝ Which of a set each letter is: an outer product.
'AEIOU'∘.='AUDIO'
⍝ A character is never equal to a number.
'1'=1
⍝ Other functions still refuse characters.
'A'<'B'
