Phase 10 step 4 (docs/plan.md). Libraries kept outside the repository, and )LIBS, at the CLI and the service.

Configuration is outside the language, at session start: a TOML file (--config FILE, else ./sw-apl.toml, else the user's config directory, sw-apl/config.toml) mapping library numbers to directories with a name, and --lib N=DIR[,NAME] to add or override one. Library 0 stays yours and writable; 1 stays what sw-apl ships (ws/lib1); 2 and up come from the configuration and are read-only. A configured library keeps library 1's file naming (NAME.apl.ws, NAME.a-70.apl.ws, NAME.b-75.apl.ws) and modes lines, so )LIB N and )LOAD N NAME see only what runs in the mode. sw-apl-server takes the same flags and file.

)LIBS, an sw-apl extension: one line per library -- number, name, where it comes from, read-only or not. A library whose directory is missing says so rather than failing a session. The owner's separate workspaces repository is the case to test against; a fixture directory under tests/ stands in for it.

TDD: store and session tests for config parsing, precedence (flag over file), read-only refusal, missing directories, mode filtering in a configured library. Docs: session.md, workspaces.md, commands-reference.md, README (how to point sw-apl at another repository), parity.md.
