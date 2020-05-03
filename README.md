Nautilus
========
![GitHub Workflow Status](https://img.shields.io/github/workflow/status/gatewaynode/nautilus/Rust)

A simple headless, command line CMS for management of simple blog like data stores.
---------------------------------------------------------------------------
This has been a simple learning project for myself for both the Rust language
and crates like Diesel ORM, Clap, Serde and such.  Much of it is based on the
Diesel simple CLI tutorial not so much the advanced CLI example in the Diesel
source.  The project is pretty quickly moving away from that and not much will
resemble it soon.   

Differences from the Diesel Simple CLI tutorial:
* Argument parsing with Clap allows a more standard arrangement of parts between
``cli.yml``, ``lib.rs`` and ``main.rs``
* CRUD functions changed to a fuller workflow (for me at least, publish flagging
  dropped)
* Use of VIM as a content editor
* Post and Links content types, a managed System table also exists for internal
things
* Import an existing post from JSON
* Export a post to JSON

Installation
---------------
1. Clone the git repo and change into that directory
1. Install PostgreSQL DB and create an empty database with full acces for your
user
1. The ``example.env`` should be changed to a regular ``.env`` file with the
appropriate creds and such before this will work.
1. Use the [Diesel](https://diesel.rs/guides/getting-started/) directions for
setting up the database (``diesel setup`` --> ``diesel migration run``)
with those sorts of things.
1. ``cargo run post`` to build and create your first post
1. ``cargo run list`` to see the posts in the database
1. I recommend symlinking to the binary from your ``~/.local/bin/`` directory
for easier use

The Basics
----------

It's important to understand this is only a CLI interface to a database.  To
actually host the database content in a website you need to manually embed the
lib.rs, schema.rs and models.rs files in a web server framework (like I do with
[Rocket](https://rocket.rs)).  By manually I mean I symlink them right now, but
there will be a crate to make this easier in the near future.

The reasons this isn't directly integrated with a web framework circle around
two concepts: one, "compartmentalization of concerns" provides better security;
two, "modularity" let's me move the data around to several different web
frameworks easily without worrying about the CMS becoming web framework
dependent.  Also in the vein of the web framework work, there is no reason this
can't be used to power an app with a mobile or desktop UI since it's just
creating and structuring content in a PostgreSQL DB.

From the CLI there are sub-commands like ``post`` to create blog like posts,
with variants such as ``epost #`` to edit a post by number, ``dpost`` to delete
posts.  Also similar commands exist for ``link`` and ``system`` to manage those
pieces of content.

The Roadmap
-----------
* Finish web support basics so a fully functioning blog can be setup with this.
* Create an interactive mode (this is actually supposed to be the default but
  the REPL can wait until the basics work)
* There are some binaries compiles on an AMD64 in the potential binary
section, these are primarily for testing at this point (should work on the
right arch)
* Build and document different heads: web frameworks, mobile frameworks, desktop
UI, GraphQL API, REST API


```
USAGE:
    nautilus [FLAGS] [SUBCOMMAND]

FLAGS:
    -d, --debug      Output internal variables
    -h, --help       Prints help information
    -v, --verbose    Enable extra output
    -V, --version    Prints version information

SUBCOMMANDS:
    delink     delete a link by id (sub args)
    depost     delete a post by post_id number (sub args)
    dsystem    Delete a system value by key
    elink      edit a link by link_id and field arg
    epost      edit a post by post_id and field arg
    esystem    Edit a system value by key
    export     export a post by post_id to a file (sub args)
    help       Prints this message or the help of the given subcommand(s)
    import     import a post from a file (sub args)
    link       create a piece of link content
    list       list post contents
    post       create a piece of post content
    show       show a blog post by id (sub args)
    slink      Show all links
    ssystem    Show system entry data
    system     Enter a system value
    testing    Don't use this, it's for prototyping new functions


```

Actions on specific resources require the ID value which can be obtained through
the list function or directly from the database.

Examples:
---------
List blog posts.
```
nobody@computer:~/code/rust-projects/nautilus$ cargo run list
... compile stuffs...
+----+--------------------------------------------------------------+------------------+----------------------------+
| ID | TITLE                                                        | TAGS             | TIME                       |
+----+--------------------------------------------------------------+------------------+----------------------------+
| 8  | Rust default function parameters and Clap-rs argument YAML's | front, rust      | 2020-04-28 19:44:25.148125 |
+----+--------------------------------------------------------------+------------------+----------------------------+
| 7  | Publishing a Dynamic Website Statically                      | front, bash, aws | 2020-04-27 16:05:22.837377 |
+----+--------------------------------------------------------------+------------------+----------------------------+
| 6  | Hitting that coding stride in a new lang                     | front, rust      | 2020-04-27 03:20:35.770888 |
+----+--------------------------------------------------------------+------------------+----------------------------+
| 5  | Rustyline is pretty slick                                    | front, rust      | 2020-04-26 17:38:24.467518 |
+----+--------------------------------------------------------------+------------------+----------------------------+
| 4  | Diving into Rocket                                           | front, rust      | 2020-04-24 22:58:29.070613 |
+----+--------------------------------------------------------------+------------------+----------------------------+
| 3  | Adventures in Rust                                           | front, rust      | 2020-04-24 21:09:03.207312 |
+----+--------------------------------------------------------------+------------------+----------------------------+
| 2  | How I found the PinePhone and why this is awesome.           | front            | 2020-04-24 21:06:40.213583 |
+----+--------------------------------------------------------------+------------------+----------------------------+
| 1  | And now I have a CMS                                         | front, python    | 2020-04-24 20:53:32.588076 |
+----+--------------------------------------------------------------+------------------+----------------------------+
```

Show a particular blog post
```
nobody@computer:~/code/rust-projects/nautilus$ nautilus show 3
+----+--------------------+----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+-----------+
| ID | TITLE              | BODY                                                                                                                                                                                                                                                                                                                                                                                                                                   | PUBLISHED |
+----+--------------------+----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+-----------+
| 3  | Adventures in Rust | <p>So I recently checked back in on the programming language Rust just to see where it was going and boy was I surprised.  Not only has it really started to take off in terms of new users and projects but the nature of the projects is fantastic.  So switching gears, as I often do, I set myself the goal of learning Rust this year and what better way than hitting the books and mirating my Python powered blog to Rust?</p> | false     |
|    |                    | <p>I'll document on here as I go and include deeply annotated source code, which maybe will help others but is really just for me to study with.</p>                                                                                                                                                                                                                                                                                   |           |
+----+--------------------+----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+-----------+
```

Enter a new blog post
```
nobody@computer:~/code/rust-projects/nautilus$ nautilus -v post
Writing Post.
Title: My Awesome Title
Writing Body...
<vim get's opened here>
Saved My Awesome Title with id 1
```

Use at your own risk
====================
This is usable, but it is far from intuitive and I'm sure there are bugs.  I'm
using it to power [gatewaynode.com](https://gatewaynode.com) but that's a very
custom website build so it may or may not work for you.  Maybe there will be a
0.1 release in a month or so.
