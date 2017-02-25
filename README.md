# Y CHALLENGE GO!

Real simple - can we get a pure rust implementation of saxpy to match performance
w/ the fortran implementation? Because Y not?

#### Install

Make sure you've got [rust installed](https://www.rust-lang.org/en-US/install.html).

```bash
$ git clone git@github.com:MurphyMarkW/y.git
$ cd y
# Optionally, if you're using rustup for managing which version of rustc to use...
$ rustup override set nightly
$ cargo bench
```
