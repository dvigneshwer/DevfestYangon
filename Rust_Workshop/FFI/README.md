
# Integrating Rust with other languages

## Steps to run the different FFI examples:

rust to c
=========

Calling C operations from Rust

~~~~
cd project_dir

sudo apt-get install gcc

// cargo.toml changes

[package]
name = "rust-to-c"
version = "0.1.0"
authors = ["Vigneshwer.D <dvigneshwer@gmail.com>"]
build = "build.rs"

[dependencies]
libc = "0.1"

[build-dependencies]
gcc = "0.3"

touch double.c
~~~~

C to Rust
=========

Calling Rust operations from C

~~~~
cd project_dir

sudo apt-get update && apt-get install build-essential

make -f Makefile
~~~~

Node to Rust
============

Calling Rust operations from Node.js apps

~~~~
cd project_dir

sudo apt-get update 
curl -sL https://deb.nodesource.com/setup_6.x | sudo -E bash -
sudo apt-get install npm
sudo apt-get install nodejs

make -f Makefile
~~~~

python to Rust
==============

Calling Rust operations from Python

~~~~
cd project_dir

sudo apt-get update
sudo apt-get -y upgrade

make -f Makefile
~~~~

python-rust-library
===================

Writing a Python module in Rust

~~~~
cd project_dir

sudo apt-get update
sudo apt-get -y upgrade
sudo apt-get install python-pip python-dev build-essential

make -f Makefile
~~~~

## Read More

Rust-Python FFi 
===============

* Regex
	* https://regexone.com/lesson/letters_and_digits
	* https://regexone.com/references/python

* Rust-Python FFI 
	* https://developers.redhat.com/blog/2017/11/16/speed-python-using-rust/

* map zip lambda
	* https://bradmontgomery.net/blog/pythons-zip-map-and-lambda/

* GIL
	* https://stackoverflow.com/questions/1294382/what-is-a-global-interpreter-lock-gil
	* https://wiki.python.org/moin/GlobalInterpreterLock

More links:
===========

* https://github.com/mitsuhiko/snaek
* https://github.com/PyO3/pyo3
* https://pypi.python.org/pypi/setuptools-rust
* https://github.com/mckaymatt/cookiecutter-pypackage-rust-cross-platform-publish
* http://jakegoulding.com/rust-ffi-omnibus/
* https://github.com/urschrei/polylabel-rs/blob/master/src/ffi.rs
* https://bheisler.github.io/post/calling-rust-in-python/
* https://github.com/saethlin/rust-lather