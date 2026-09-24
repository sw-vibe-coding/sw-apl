# Library fixtures

`extended/` stands in for a library kept outside this repository --
the owner's sw-apl-workspaces, which is library 2 by convention. It
holds one workspace that runs in both modes and one for (B) only, so
the tests see a configured library listed, loaded and filtered by
mode as library 1 is. Configured with `--lib 2=tests/libs/extended,EXTENDED`.
