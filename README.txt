
        Play Me Some ... Sudoku!


About
-----

    A simple Sudoku game written in Rust.

    Features:
    - undo and redo
    - playable at almost all resolutions
    - sexy graphics!


Compiling
---------

    Requires SDL2 from your distribution.
    Other dependencies will be handled by cargo.

    To build, run:

        cargo build


Starting up
-----------

    To play, run:

        cargo run <optional puzzle file>

    Alternatively, run the executable found inside the 'target' directory.

    The data file, sudoku.png, should be in either:
    1. current_directory/resources/sudoku.png, or
    2. the same directory as the executable.

    An example puzzle file:

        .........
        .....3.85
        ..1.2....
        ...5.7...
        ..4...1..
        .9.......
        5......73
        ..2.1....
        ....4...9

    You can also drag-and-drop a puzzle file to load it.


Controls
--------

    z - undo
    x - redo
    c - pencil tool
    v - cross out tool
    1-9 - select number

    lmb - assign number, or cross out a possibility
    rmb - unassign number
    wheel - cycle through numbers
    mouse thumb buttons - undo, redo

    F11, f - toggle fullscreen


Author
------

David Wang <millimillenary@gmail.com>
