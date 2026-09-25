# Emulation policy

Some of what an APL answers was never the language's to decide. The
machine decided it: what time it is, how many terminals are signed
on, how big a workspace may be, who else is on the system. sw-apl
runs on a laptop, a server or a phone, none of which is a System/360
or a 5110, so each such answer needs a policy. This document gives
the policy, case by case.

`parity.md` says what is done and what is restricted; this says why
each machine-decided answer is what it is. `mode-b.md` has the
detail of (B)'s system variables, and `i-beam-reference.md` (A)'s
I-beams.

## The classes

Every case is one of these.

| Class | Meaning |
|---|---|
| **historical** | As the modelled machine did it, even where a modern host could do more |
| **host** | The modern host's own answer: its clock, its processor time |
| **fixed** | A constant, the same on every run, chosen so a transcript reproduces |
| **unavailable** | Not emulated: the command or facility is absent, and says so |
| **extension** | sw-apl's own, which no historical system had |

## The cases

| Case | Mode | Class | What sw-apl does |
|---|---|---|---|
| `⎕TS`, the time stamp | (B) | historical | `1900 0 0 0 0 0 0`, as the 5110 gives it: the machine had no clock. It may be assigned, as the manual allows, and then holds what was assigned for the session. It never reads the host's clock |
| `⎕AI`, `⎕TT`, `⎕UL`, `⎕DL` | (B) | historical | The 5110's compatibility values for one user and no clock: `⎕AI` four zeros, `⎕TT` 0, `⎕UL` 1, `⎕DL` a variable holding 0 |
| `⌶20` time of day, `⌶24` sign-on time, `⌶25` the date | (A) | host | The host's local clock. A 360 had a clock and APL\360 read it, so the nearest thing is the machine sw-apl runs on. In the browser it is the page's clock |
| `⌶21` processor time | (A) | host | The processor time this process has used, in sixtieths of a second; on a modern machine usually 0. In the browser, always 0: a page cannot ask |
| `)OFF` and `)CONTINUE` | both | host | The moment, the time connected, and the processor time, from the host, as `⌶20`, `⌶24` and `⌶21` read them |
| `⌶23` terminals connected | (A) | fixed | 1. A session is one user; the service holds sessions that cannot see each other, so none is ever told about another |
| The random link | both | fixed | A clear workspace starts from 16807 in both modes (in (B), `⎕RL`), so a transcript that rolls dice reproduces |
| The size of a workspace | both | host | 1048576 bytes unless `--ws-size` says otherwise, the same in both modes. A 5110 held at most 64 KB; sw-apl does not impose it, and `⎕WA` and `⌶22` report what is left of the size in force |
| `)SYMBOLS` | both | fixed | Reports how many names would fit in the workspace size, and how many are held. Nothing is set aside for names, so `)SYMBOLS N` cannot set it and is INCORRECT COMMAND. The 5110's 125 and APL\360's 256 are not emulated |
| `)MSG`, `)OPR`, `)PORTS`, `)NUMBER` and the other commands of a shared machine | both | unavailable | Absent: INCORRECT COMMAND. They talked to other users and the operator of a time-shared 360, and sw-apl has one user |
| Shared variables | (B) | unavailable | Absent, as on the 5110 |
| A saved suspended function | both | unavailable | A workspace file is text, and text cannot resume part way through a call, so `)SAVE` and `)CONTINUE` keep the names but not the state indicator |
| `)DIALECT`, `)LIBS`, `)HELP` | both | extension | sw-apl's own commands. They are system commands, never quad names, so no program or saved workspace can come to depend on them |
| Libraries 2 and up | both | extension | Directories configured when the session starts (`--lib`, `sw-apl.toml`), read-only. APL\360's numbered libraries belonged to accounts on the one machine; these are wherever the workspaces are kept |

## Why these and not others

- **(B) keeps its machine's answers.** The owner's direction is "an
  IBM 5120 on my phone": a program written for the 5110 should see
  what a 5110 showed it, a clock in 1900 included. A program that
  needs the date asks the user for it, as it had to then.
- **(A) reads the host where APL\360 read its host.** A 360 had a
  clock, and APL\360 users wrote programs that read it; giving them
  a fixed value would be less faithful, not more.
- **Fixed values keep transcripts honest.** Where a value varies and
  nothing historical pins it, as `⌶23` and the random link, a
  constant keeps every sample's transcript the same on every run.
  Where a value must vary, a sample labels it `(VARIES)` for the
  regression filter (`testing.md`).
- **What cannot be emulated is absent, not faked.** A command for
  another user who is not there answers INCORRECT COMMAND rather
  than pretending to send.

## A switch between historical and host

A flag choosing between the historical answer and the host's, such
as `--environment historical` or `host`, is not implemented. The one
case it would change is (B)'s clock: `⎕TS` in (B) is always the
5110's, and nothing makes it read the host's clock.
