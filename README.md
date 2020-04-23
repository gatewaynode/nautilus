Nautilus
========
A simple command line CMS for management of simple blog like data stores.
------------------------------------------------------------------
This has been a simple learning project for myself for both the Rust language
and crates like Diesel ORM, Clap, Serde and such.  Much of it is based on the Diesel
simple CLI tutorial not so much the advanced CLI example in Diesel.  

Differences from the Diesel Simple CLI tutorial:
* Argument parsing with Clap allows a more standard arrangement of parts between ``lib.rs`` and ``main.rs``
* CRUD functions changed to a fuller workflow (for me at least, publish flagging dropped)
* Use of VIM as a content editor
* Import an existing post from JSON
* Export a post to JSON

Usage
-----
There are no release binaries yet, this will need to be compiled with Cargo.

```
nautilus [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -v <verbose>...        Set's the level of verbosity

SUBCOMMANDS:
    delete    delete a post by post_id number (sub args)
        <post_id>    The post id number (position 1)
    edit      edit a post by post_id (sub args)
        <post_id>    The post id number (position 1)
    export    export a post by post_id to a file (sub args)
        <post_id>            The post id number (position 1)
        <export_filename>    The filename to save to (position 2)
    help      Prints this message or the help of the given subcommand(s)
    import    import a post from a file (sub args)
        <import_filename>    The filename to open and import (position 1)
    list      list post contents
    show      show a blog post by id (sub args)
        <post_id>    The post id number (position 1)
    write     create a piece of post content
```

Bits and Pieces
---------------
* Command line options are defined in ``src/cli.yaml``, which happens to be my new
favorite way of managing command line stuff thanks to Clap.
* The ``example.env`` should be changed to a regular ``.env`` file with the
appropriate creds and such before this will work.
* Use the Diesel directions for setting up the database or don't if your handy
with those sorts of things.
* Only tested with local PostgreSQL DB's just so you know.

Use at your own risk
====================
This is just barely working.  I really haven't gotten around to writing tests or throwing this in a pipeline yet, but you know merge requests and Github issues are welcome.
