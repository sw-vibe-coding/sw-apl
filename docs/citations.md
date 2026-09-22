# Citations

Every source sw-apl's behaviour, documentation and workspaces refer
to. The code and the docs describe what these say in their own words;
none of them is copied into the repository. The one borrowed file the
repository does carry, the 2741 keyboard picture, is listed last with
the licence it is carried under.

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
  ⌹ that `data/glyphs.toml` quotes.
  <https://www.bitsavers.org/pdf/ibm/apl/SH20-0906-0_APL_360-OS_and_APL_360-DOS_Users_Manual_Dec70.pdf>
- **IBM APL\360-OS (5734-XM6)**, Computer History Museum catalog.
  <https://www.computerhistory.org/collections/catalog/102787847>

## The IBM 5100 family -- the (B) '75 mode

- **IBM 5110 APL Reference Manual**, SA21-9303-0, IBM, December
  1977. The reference for the (B) mode: its commands, system
  variables and functions, editor, keyboard, and its appendices on
  how the 5110 differs from the 5100 and from APLSV.
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
  photograph of a 5100 keyboard; not carried, licence unchecked.
  <https://commons.wikimedia.org/wiki/File:Ibm5100_(2297950254).jpg>

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

## Carried in the repository, under licence

- **APL-keybd2.svg**, Wikimedia Commons user Rursus, July 2007:
  the IBM 2741 APL keyboard. Carried under CC BY-SA 3.0, with an
  adaptation beside it; the licence, the credit and what was changed
  are in `images/redistributed/apl-keyboard/`.
  <https://commons.wikimedia.org/wiki/File:APL-keybd2.svg>
