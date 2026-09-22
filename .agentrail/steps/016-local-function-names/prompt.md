Found 2026-09-22 building step 015 (system functions): a local name
does not hide a global function of the same name, in either mode.

    ∇R←G X
    R←X+1
    ∇
    ∇F;G
    G←5
    G
    ∇
    F

gives SYNTAX ERROR on F[2]: G←5 stores a variable beside the function
(the activation displaces only variables), and G still parses as the
function. APL\360 and APLSV shadow every referent of a local name, so
F should show 5, and G 1 afterwards should give 2 again.

- The activation displaces the function a local name holds, and puts
  it back on return; the displaced record and )VARS / )FNS / )SIV
  follow (a )FNS under a suspension lists global functions).
- This is a correctness fix to (A): a transcript it moves is moved
  with the manual as the reason (RED test first, a sample line).
- Then (B): a function fixed by ⎕FX under a name that is local in an
  active function is local, and disappears when that function returns
  (APL/CMS manual, Function Establishment; the 5110 manual's CHANGE
  example). Step 015 refuses such a ⎕FX (the header row's index) until
  this lands; lift that refusal and add the manual's example to the
  system functions sample.
- ⎕EX of such a local expunges the active referent only.
