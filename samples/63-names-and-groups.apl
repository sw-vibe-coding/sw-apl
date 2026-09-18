⍝ What a workspace holds, and how to handle several names as one.
⍝ )VARS lists the global variables and )FNS the defined functions,
⍝ alphabetically. A letter starts the listing there.
ZED←1
ALPHA←2
MID←3
)VARS
)VARS M
∇R←A HYP B
R←((A*2)+B*2)*0.5
∇
∇R←TWICE B
R←B×2
∇
)FNS
⍝ )VARS lists GLOBAL variables. Inside a suspended function the
⍝ locals are in scope, but they are not what )VARS shows.
∇SHOW;LOCAL
LOCAL←99
NOSUCH
∇
SHOW
)SI
LOCAL
)VARS
→
⍝ A group gives one name to a collection of names, so that they can
⍝ be copied or erased together. A member need not exist yet.
)GROUP TRIG HYP TWICE LATER
)GRPS
)GRP TRIG
⍝ )GRPS and the name listings sort; )GRP shows the group as it was
⍝ gathered. Naming the group again supersedes it; naming it among
⍝ its own members adds to it.
)GROUP TRIG TRIG ALPHA
)GRP TRIG
⍝ )ERASE removes global objects. A group name takes its members.
)ERASE TRIG
)FNS
)VARS
⍝ The group went with them.
)GRPS
⍝ A function on the state indicator is waiting to be taken up again,
⍝ so )ERASE leaves it and says which.
∇R←STUCK
NOSUCH
∇
STUCK
)ERASE STUCK MID
)FNS
→
⍝ )SYMBOLS says how many names the workspace holds. The size is what
⍝ the space still free would hold, so it moves as the workspace
⍝ fills; APL\360 set a symbol table aside at sign-on and sw-apl
⍝ charges names against the workspace like everything else.
)CLEAR
X←1
Y←2
)SYMBOLS
)OFF
