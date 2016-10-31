# pwds

[![Version](https://img.shields.io/crates/v/pwds.svg)](https://crates.io/crates/pwds)
[![Build Status](https://travis-ci.org/rahiel/pwds.svg?branch=master)](https://travis-ci.org/rahiel/pwds)
[![License](https://img.shields.io/crates/l/pwds.svg)](https://github.com/rahiel/pwds/blob/master/LICENSE.txt)
[![Downloads](https://img.shields.io/crates/d/pwds.svg)](https://crates.io/crates/pwds)

**P**rint the path of the current **w**orking **d**irectory, **s**hortly.

The current working directory in your prompt can get uncomfortably large,
leaving little space to type your own commands. With pwds paths like
`/home/user/Code/rust/src/doc/nomicon` are displayed as `~/C/r/s/d/nomicon`.

It shows the first two characters for (hidden) directories that start with a
".". So `/home/rahiel/.config/autostart` becomes `~/.c/autostart`.

# Installation

Install pwds with cargo:
``` shell
cargo install pwds
```

Then you need to customize your PS1 in your shell's initialization file, e.g.
`.bashrc`. Here is a standard prompt, the `\w` is an escape code for the current
working directory:
``` shell
PS1='\[\033[01;32m\]\u@\h\[\033[00m\]:\[\033[01;34m\]\w\[\033[00m\]\$ '
```
replace `\w` with `$(pwds)`:
``` shell
PS1='\[\033[01;32m\]\u@\h\[\033[00m\]:\[\033[01;34m\]$(pwds)\[\033[00m\]\$ '
```
and enjoy a smaller prompt!

# Customization

By default, paths with more than 16 characters are shortened by replacing
directory names with their first character. This can be configured by setting
the `PWDS_LENGTH` environment variable:
``` shell
export PWDS_LENGTH=10
```
The current (most right) directory is never shortened.

# References

I encountered the concept of `pwds` in [fish][] and more recently in [xonsh][],
two modern and user-friendly shells. I'd use xonsh but a few issues keep me from
switching. In the meantime I'm trying to improve the experience of the Bash
shell, with as one of the results this program.

* [Bash prompts](https://sanctum.geek.nz/arabesque/bash-prompts/)
* [Alternative prompt directory shortening for Bash](https://sanctum.geek.nz/arabesque/prompt-directory-shortening/)
* [pwds.py][]: Python version of `pwds`.

[fish]: https://fishshell.com
[pwds.py]: https://gist.github.com/rahiel/cdfda857534bc9dd4456e404a996e38f
[xonsh]: http://xon.sh
