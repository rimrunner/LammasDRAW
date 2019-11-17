# LammasDRAW

This is a Rust language practice project which is an ANSI Editor
- a program to make drawings by using ANSI characters 
(code page 437, the character set of the original IBM PC), 256 characters 
(minus few control characters) with 16 foreground and 8 background colors.
Images made of ANSI characters  were commonly seen in the 1980s and 1990s 
in bulletin board systems (BBS) which preceeded the Internet as a means of
a multi party digital communication. ANSI art is still being made in the 
demoscene as a form of underground art.

This project is currently under work.
Right now it is possible to draw characters to fixed sized screen and change 
their colors but it is not yet possible to choose characters itself. This will
be implemented next.

The project uses SDL Rust bindings and a code page 437 font file created by Zeh Fernando 
