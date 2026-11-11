# APL timeline

The APLs sw-apl touches, and the APL history around them, in order.
sw-apl has two modes: (A), modeled on APL\360 as it stood from its
December 1970 release, and (B), modeled on the APL of the IBM 5100
family. The site opens on a drawing of this timeline, made from
`data/timeline.json`; this page and that file say the same things, and
a change to one is made to the other.

Every entry comes from the sources listed at the foot of the page,
paraphrased. Where the sources disagree, or none was found, the entry
says so.

## Before sw-apl's first mode

| Date | What happened | For sw-apl |
|---|---|---|
| 1957 | FORTRAN | |
| 1958 | Lisp; ALGOL | |
| 1959 | COBOL | |
| 1962 | Kenneth Iverson publishes *A Programming Language* (Wiley), the notation APL is named after | |
| November 1966 | APL\360 first runs, on an IBM System/360 Model 50, as a time-sharing system. For about two years it is used inside IBM | |
| Autumn 1967 | APL\1130, for the IBM 1130, is written | |
| February 1968 | APL\1130 is first distributed; its Primer says labels, common libraries and passwords came later | The owner's separate project; a future demo on the timeline |
| August 1968 | APL\360 is released outside IBM as a Type-III Library program: source available, free, and unsupported. The *APL\360 User's Manual* is dated the same month | |
| March 1970 | The second edition of the *APL\360 User's Manual* (GH20-0683-1). Its primitives are the first edition's: take and drop are in both, domino in neither | Checked against alongside the first |
| June 1970 | IBM announces System/370, the successor to System/360, which runs System/360 software unchanged | |

## The years the owner used APL

| Date | What happened | For sw-apl |
|---|---|---|
| 1970 | Forth | |
| 1970 | APL*PLUS, STSC's time-sharing APL, is made available with I. P. Sharp Associates; later APL*PLUS/PC and APL*PLUS III | |
| December 1970 | APL\360-OS and APL\360-DOS become IBM Program Products (5734-XM6 and 5736-XM6), with their user's manual (SH20-0906-0). This APL\360 has domino and lamination | **(A) '70: the APL sw-apl builds.** The owner learns APL on it, on an IBM 2741, in 1972 |
| By 1973 | APL 1100, for the UNIVAC 1108, from the University of Maryland's Computer Science Center: workspaces of 16K to 256K bytes, domino, shared variables, and an APL print bar UNIVAC offered to all its customers | The owner used it on a UNIVAC 1110 at SUNY Albany in 1976-77; a future demo |
| May 1973 | The MCM/70, a Canadian microcomputer running APL on an Intel 8008, is shown at the Fifth International APL Users' Conference in Toronto | |
| 1973 | APLSV (also written APL.SV) adds shared variables, the quad system variables and functions, execute and format. It runs on System/370 | What (B)'s language derives from; the owner used it from 1977 |
| September 1975 | IBM announces the 5100, a portable computer with APL, BASIC or both; delivery begins the same month | **(B) '75: the APL sw-apl builds** |
| About 1976 | I. P. Sharp Associates' time-sharing APL takes the name SHARP APL | |
| 1976 | VS APL, the System/370 APL that adds files | |
| 1977 | An APL standard, begun inside IBM in 1974, is ratified as an IBM standard: the root of ISO 8485 | |
| January 1978 | IBM announces the 5110, a successor to the 5100 with a diskette option. The owner starts using IBM 51xx computers | (B) follows the 5110's APL manual |
| 1979 | Kenneth Iverson receives the Turing Award; his lecture is *Notation as a Tool of Thought* | |
| February 1980 | IBM releases the 5120, the desktop follow-on to the 5110, with two built-in diskette drives and APL and BASIC in ROM | The machine (B) recreates |
| 1980 | APL80, a large subset of APL for the TRS-80. Its manual is undated; 1980 is the owner's | C'80: a demo still to come. The owner used it in 1980 |
| 1982 | APL2, with nested arrays, is released as an Installed User Program: a prototype | |
| April 1983 | Dyalog APL v1.0 is launched at the APL83 conference in Washington | |
| June 1983 | IBM Personal Computer APL is announced; it was developed at the IBM Madrid Scientific Center. It required the 8087 math coprocessor, then an option in the PC | D'83: a demo still to come. The owner used it from 1983 |
| 1984 | APL2 is released | |
| December 1985 | APL2 Release 2 adds vector specification, `⎕EC`, and lowercase in names; on VM/CMS it is typed on a 3279 color graphics terminal | A future demo. The owner used APL2 from 1990 |

## Since

| Date | What happened |
|---|---|
| 1988 | A+, Arthur Whitney's APL at Morgan Stanley (after his A of 1985), the first built on leading-axis theory; K followed it. It was later released free under the GPL |
| January 1988 | I-APL 1.0, a free APL for schools and small computers (the BBC Micro, PC clones, the RML Nimbus, the Archimedes), written by Paul Chapman and backed by the British APL Association and SIGAPL |
| 1987 | ISO accepts an edited form of the IBM standard, published as ISO 8485:1989, the APL standard: flat arrays, from APLSV |
| 1990 | J, Kenneth Iverson and Roger Hui's successor to APL, in ASCII |
| About 1990 | TryAPL2, IBM's free APL2 for DOS, limited in its workspace size, for students |
| 1992 | K, Arthur Whitney's list-based array language, first available |
| 2001 | ISO/IEC 13751, Extended APL: nested arrays, mainly from APL2; the most recent APL standard |
| 2003 | q and kdb+, K's descendant built into a database |
| 2004 | Kenneth Iverson dies |
| September 2013 | GNU APL 1.0, a free APL2 for GNU systems |
| 2014 | Futhark, a data-parallel functional array language for GPUs from DIKU in Copenhagen; APL is among its influences |
| 2020 | BQN, Marshall Lochbaum's APL, begun at Dyalog and his own from early 2020 |
| 2020 | Kap, Elias Mårtenson's APL in Kotlin, with lazy evaluation and immutable values |
| April 2021 | IBM withdraws APL2 from marketing; support ends that September |
| 2023 | Uiua, Kai Schmidt's stack-based array language, its glyphs mostly from BQN |
| March 2024 | TinyAPL, Madeline Vergani's small APL in Haskell, first released |

The timeline ends with the owner's own array languages: sw-cor24-apl,
an integer-only subset of APL in C on the COR24-TB development board,
written with names in place of glyphs; sw-MLPL, a first custom array
language, in Rust, for machine learning and general-purpose work; and
X_eTaL, an experimental, extensible, typed array language in Rust that
runs math on CPUs, GPUs and FPGAs.

## The owner's questions, answered

- **When was APL\360 released?** Outside IBM, in August 1968, as a
  Type-III Library program. It had run inside IBM since November
  1966.
- **When did System/370 and APLSV come out?** System/370 was announced
  in June 1970, before domino reached APL\360. APLSV came in 1973.
  1972 was the year the virtual-memory System/370 models appeared, not
  APLSV.
- **When was University of Maryland APL?** APL 1100 was built by the
  Computer Science Center's systems group by 1973, for the UNIVAC
  1108; the history that describes it gives no closer year. The owner
  used it on a UNIVAC 1110 at SUNY Albany in 1976-77.

`apl-feature-history.md` gives the same APLs feature by feature.

## Sources

Listed in full, with links, in `citations.md`:

- The SIGAPL chronology of APL, and APL Wiki pages on time-sharing,
  APL\360, APL\1130, APL.SV, APL2, ISO 8485:1989, ISO/IEC
  13751:2001, Kap, TinyAPL and the timeline of influential array
  languages (J, K).
- The *APL\360 User's Manual*, August 1968 and March 1970, and the
  *APL\360-OS and APL\360-DOS User's Manual*, December 1970; the
  *APL\1130 Primer*, 1968; the APL80 manual; IBM's *APL Reference* for
  the Personal Computer, May 1983, and the IBM Systems Journal paper on
  it; *An Introduction to APL2*, December 1985.
- Jack Minker, *Beginning of Computing and Computer Science at the
  University of Maryland*, 2004, for APL 1100.
- ISO's pages for ISO 8485:1989 and ISO/IEC 13751:2001.
- Dyalog's history of the company; the GNU APL 1.0 announcement;
  Futhark's own history; Marshall Lochbaum's *BQN's development
  history*; uiua.org and the APL Wiki on Uiua; TinyAPL's repository,
  for its first release.
- The Datapro report on the IBM 5100 (October 1975), and Wikipedia on
  the IBM 5110, the IBM 5120 and System/370.
- The owner, for the years they used each APL: APL\360 from 1972,
  University of Maryland APL (APL 1100, on a UNIVAC 1110 at SUNY
  Albany) in 1976-77, APLSV from 1977, APL80 in 1980,
  IBM PC APL from 1983, APL2 from 1990, and the IBM 51xx computers from
  1978.
- The J Wiki's history of I-APL and Anthony Camacho's I-APL manual; the
  York University Computer Museum's APL collection, for TryAPL2.
- Not yet checked against a source here: q and kdb+ in 2003, the 1979
  Turing Award, and Kenneth Iverson's death in 2004. They are as
  generally recorded.
