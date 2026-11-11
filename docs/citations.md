# Citations

Every source sw-apl's behavior, documentation and workspaces refer
to. The code and the docs describe what these say in their own words;
none of them is copied into the repository. The one borrowed file the
repository does carry, the 2741 keyboard picture, is listed last with
the license it is carried under.

bitsavers refuses scripted downloads from its main host; its mirrors
carry the same files, for example
`bitsavers.informatik.uni-stuttgart.de` and
`bitsavers.trailing-edge.com`.

## APL\360 -- the (A) '70 mode

- **APL\360 User's Manual**, IBM, August 1968. The reference for
  everything the (A) mode does: the primitives, the del editor, the
  system commands, the trouble reports, and locked functions.
  <https://www.bitsavers.org/pdf/ibm/apl/APL_360_Users_Manual_Aug68.pdf>
- **APL\360 User's Manual**, GH20-0683-1, IBM, March 1970. The second
  edition. Its primitives are the 1968 edition's -- take and drop are
  in both, and domino is in neither -- and the rules checked against
  both editions so far read the same in each.
  <https://www.bitsavers.org/pdf/ibm/apl/GH20-0683-1_APL_360_Users_Manual_Mar70.pdf>
- **APL\360-OS and APL\360-DOS User's Manual**, SH20-0906-0, IBM,
  December 1970. The Program Product release (5734-XM6, 5736-XM6)
  that (A) models, and where domino comes from: it gives the keying of
  ⌹ that `data/glyphs.toml` quotes, and the line editing `[n⎕m]`
  follows ("Line editing", p. 3.18, and its example on p. A.5).
  <https://www.bitsavers.org/pdf/ibm/apl/SH20-0906-0_APL_360-OS_and_APL_360-DOS_Users_Manual_Dec70.pdf>
- **IBM APL\360-OS (5734-XM6)**, Computer History Museum catalog.
  <https://www.computerhistory.org/collections/catalog/102787847>

## The IBM 5100 family -- the (B) '75 mode

- **IBM 5110 APL Reference Manual**, SA21-9303-0, IBM, December
  1977. The reference for the (B) mode: its commands, system
  variables and functions, editor, keyboard, and its appendices on
  how the 5110 differs from the 5100 and from APLSV. The system
  variables (Chapter 5) and the order of the atomic vector
  (Appendix B) are built from it; `⎕AV`'s order is facts from the
  table, not a copy of it.
  <https://www.bitsavers.org/pdf/ibm/5110/SA21-9303-0_IBM_5110_APL_Reference_Manual_Dec1977.pdf>
- **IBM 5110 APL User's Guide**, SA21-9302-1, IBM, August 1978.
  <https://www.bitsavers.org/pdf/ibm/5110/SA21-9302-1_IBM_5110_APL_Users_Guide_Aug1978.pdf>
- **IBM 5100 APL Reference Manual**, SA21-9213-2, IBM, May 1976. The
  5100's APL, and its own appendix on how it differs from APLSV.
  <https://www.bitsavers.org/pdf/ibm/5100/SA21-9213-2_IBM_5100_APL_Reference_Manual_May1976.pdf>
- **IBM 5100**, Wikipedia: announced September 1975, the year in
  the (B) '75 mode's name. <https://en.wikipedia.org/wiki/IBM_5100>
- **IBM 5100 Portable Computer**, Datapro report M11-491-201,
  October 1975: announced 9 September 1975, for delivery beginning
  that month.
  <https://bitsavers.org/pdf/datapro/datapro_reports_70s-90s/IBM/M11-491-20_7510_IBM_5100.pdf>
- **IBM 5120**, Wikipedia: the 5120 as a 5110-class machine.
  <https://en.wikipedia.org/wiki/IBM_5120>
- **IBM Beam Spring Keyboards**, Deskthority wiki: the 5100 and 5120
  keyboards sharing a layout. <https://deskthority.net/wiki/IBM_Beam_Spring_Keyboards>
- **Ibm5100 (2297950254).jpg**, Marcin Wichary, Wikimedia Commons: a
  photograph of a 5100 keyboard; not carried, license unchecked.
  <https://commons.wikimedia.org/wiki/File:Ibm5100_(2297950254).jpg>

- **IBM 5100 APL Reference Manual**, SA21-9213-2, May 1976, as above:
  its section on the scan operator is what keeps scan in (B).

## APLSV -- what the 5100 family's APL derives from

- **APL Language**, GC26-3847-0, IBM, March 1975. The language
  APLSV implements: execute, format, and the system variables and
  functions.
  <https://www.bitsavers.org/pdf/ibm/apl/GC26-3847-0_APL_Language_Mar75.pdf>
- **APL Shared Variables (APLSV) User's Guide**, SH20-1460-1, IBM,
  March 1975. What is particular to the APLSV system: its system
  variables' values in a clear workspace, its limits, its additional
  commands, and how its function editing differs.
  <https://www.bitsavers.org/pdf/ibm/apl/SH20-1460-1_APL_Shared_Variables_Users_Guide_Mar75.pdf>
- **APL/CMS User's Manual**, SC20-1846, IBM, July 1974. Covers
  APL\360 and APLSV workspaces together.
  <https://www.bitsavers.org/pdf/ibm/apl/SC20-1846_APL-CMS_Users_Manual_197407.pdf>

## APL generally

- **APL\360**, APL Wiki. <https://aplwiki.com/wiki/APL%5C360>
- **The APL collection**, Computer History Museum Software
  Preservation Group. <https://softwarepreservation.computerhistory.org/apl/>
- **APL language features**, Try MTS: APL on the Michigan Terminal
  System. <https://try-mts.com/apl-language-features/>
- **Running APL\360 on OS/360 MVT 21.8F**, hercules-390 mailing list:
  how the library workspaces were resurrected from tape.
  <https://hercules-390.yahoogroups.narkive.com/SakfTQxp/running-apl-360-on-os-360-mvt-21-8f-resurrecting-library-workspaces>
- **Roll**, Roger Hui, Jsoftware. The Lehmer generator behind roll
  and deal, which sw-apl's matches, and J's mapping of a link onto a
  range, which sw-apl's does not. <https://www.jsoftware.com/papers/roll.htm>

## The feature history

`apl-feature-history.md` draws on these, besides the manuals above.

- **APL\1130 Primer**, C20-1697-0, IBM, 1968.
  <https://www.bitsavers.org/pdf/ibm/apl/C20-1697-0_APL_1130_Primer_1968.pdf>
- **APL80**, Phelps Gates and Ramware, undated: the manual for APL on
  the TRS-80.
  <https://archive.org/details/APL80_19xx_Phelps_Gates-Ramware>
- **APL80**, Phelps Gates and Ramware, 1980, 26 pages: a second edition
  of the manual, listed in Ira Goldklang's TRS-80 catalog, which dates
  APL80 to 1980.
  <https://www.trs-80.com/sub-publications-manuals-software-1.htm#apl80-2>
- **APL Reference**, 1502219, IBM, May 1983: IBM Personal Computer APL.
  <https://bitsavers.trailing-edge.com/pdf/ibm/pc/languages/1502219_APL_Reference_May83.pdf>
- **An APL System for the IBM Personal Computer**, M. L. Tavera and
  M. Alfonseca, IBM Systems Journal 24, no. 1: the June 1983
  announcement.
  <https://bitsavers.trailing-edge.com/pdf/ibm/IBM_Systems_Journal/241/ibmsj2401H.pdf>
- **An Introduction to APL2**, SH20-9229-1, IBM, December 1985.
  <https://www.bitsavers.org/pdf/ibm/apl/SH20-9229-1_An_Introduction_to_APL_2_Dec85.pdf>
- **APL2 Migration Guide**, Version 2 Release 2, IBM, 1994.
  <https://public.dhe.ibm.com/ps/products/apl2/pubs/apl2mig.pdf>

## The timeline

`apl-timeline.md` draws on these, besides the manuals above.

- **Chronology of APL**, ACM SIGAPL.
  <https://www.sigapl.org/APLChronology.php>
- **Time-sharing**, **APL\1130**, **APL.SV** and **APL2**, APL Wiki.
  <https://aplwiki.com/wiki/Time-sharing>,
  <https://aplwiki.com/wiki/APL%5C1130>,
  <https://aplwiki.com/wiki/APL.SV>, <https://aplwiki.com/wiki/APL2>
- **IBM 5110** and **IBM System/370**, Wikipedia.
  <https://en.wikipedia.org/wiki/IBM_5110>,
  <https://en.wikipedia.org/wiki/IBM_System/370>

## The BIRDS workspace

- **To Mock a Mockingbird**, Raymond Smullyan, Knopf, 1985. The birds
  and their names.
- **The Sage Bird: Y Combinators in an Eager Array Language**,
  Software Wrighter, 2026. The core aviary BIRDS draws its list from,
  and the case for Z over Y in an applicative language.
  <https://blog.softwarewrighter.com/2026/09/21/rabbit-hole-sage-y-combinator/>
- **The combinator demos**, sw-MLPL. The same birds in another
  language. <https://mlpl.softwarewrighter.com/>

## The timeline's history, besides the sources above

- **Beginning of Computing and Computer Science at the University of
  Maryland**, Jack Minker, 2004: APL 1100 for the UNIVAC 1108.
  <http://www.cs.umd.edu/projects/photohistory/retirement/MinkerHistory.pdf>
- **ISO 8485:1989** and **ISO/IEC 13751:2001**, ISO and the APL Wiki.
  <https://www.iso.org/standard/15692.html>,
  <https://www.iso.org/standard/22817.html>,
  <https://aplwiki.com/wiki/ISO_8485:1989>,
  <https://aplwiki.com/wiki/ISO/IEC_13751:2001>
- **Our History**, Dyalog: Dyalog APL v1.0 at APL83.
  <https://dyalogdev.gos.dyalog.com/?p=14520>
- **GNU APL 1.0 Released**, Savannah, 27 September 2013.
  <https://savannah.gnu.org/news/?id=7702>
- **The past and present of Futhark**, Troels Henriksen.
  <https://futhark-lang.org/blog/2021-12-19-past-and-present.html>
- **BQN's development history**, Marshall Lochbaum.
  <https://mlochbaum.github.io/BQN/commentary/history.html>
- **Kap**, **Uiua**, **TinyAPL** and **Timeline of influential array
  languages**, APL Wiki. <https://aplwiki.com/wiki/Kap>,
  <https://aplwiki.com/wiki/Uiua>, <https://aplwiki.com/wiki/TinyAPL>,
  <https://aplwiki.com/wiki/History>
- **Fortran**, **Lisp**, **ALGOL**, **COBOL** and **Forth**, Wikipedia: the
  other languages of the timeline's early years, by name and year only.
  <https://en.wikipedia.org/wiki/Fortran>,
  <https://en.wikipedia.org/wiki/Lisp_(programming_language)>,
  <https://en.wikipedia.org/wiki/ALGOL>, <https://en.wikipedia.org/wiki/COBOL>,
  <https://en.wikipedia.org/wiki/Forth_(programming_language)>
- **STSC** and **SHARP APL**, APL Wiki: APL*PLUS from 1970, SHARP APL by name
  from about 1976. <https://aplwiki.com/wiki/STSC>,
  <https://aplwiki.com/wiki/SHARP_APL>
- **I. P. Sharp Associates** and **A+ (programming language)**,
  Wikipedia. <https://en.wikipedia.org/wiki/I._P._Sharp_Associates>,
  <https://en.wikipedia.org/wiki/A%2B_(programming_language)>
- **Chronology of APL**, ACM SIGAPL, for APL*PLUS in 1970 and the
  MCM/70 shown in May 1973.
  <https://sigapl.hosting.acm.org/APLChronology.php>
- **rubenverg/tinyapl**, GitHub: its first release, 11 March 2024.
  <https://github.com/rubenverg/tinyapl>

## The pictures on the landing page and the timeline

The pictures in `pages/machines/` are the owner's own: generated
from procedural 3D models the owner built, with X_eTaL and Blender,
and some edited afterward. They are not photographs and not borrowed
work. The badges they show are part of the historical machines they
depict.

## Carried in the repository, under license

- **APL-keybd2.svg**, Wikimedia Commons user Rursus, July 2007:
  the IBM 2741 APL keyboard. Carried under CC BY-SA 3.0, with an
  adaptation beside it; the license, the credit and what was changed
  are in `images/redistributed/apl-keyboard/`.
  <https://commons.wikimedia.org/wiki/File:APL-keybd2.svg>
- **APL385 Unicode**, Adrian Smith, 2016: the font the browser demo
  draws its glyphs in. Public domain, by the author's dedication;
  carried as WOFF2, converted and otherwise unchanged, with its terms
  and credit in `images/redistributed/apl385-font/`.
  <https://www.apl385.com/fonts/>
