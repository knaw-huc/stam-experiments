# stam-experiments

Experiments with [STAM](https://github.com/annotation/stam) and its implementations

* `exp1` -- [Experiment 1: STAM Tools on a very small text](exp1/README.md)

## Installation

First ensure you have the necessary dependencies:

* [cargo & rust](https://www.rust-lang.org/):``apt install cargo rustc`` (debian/ubuntu)
* [STAM Tools](https://github.com/annotation/stam-tools): ``cargo install stam-tools``
* [jq](https://stedolan.github.io/jq/):``apt install jq`` (debian/ubuntu)
* [make](https://www.gnu.org/software/make/):``apt install make`` (debian/ubuntu)

## Usage

Run `make` with the experiment you want to run (or leave it out to run all):

```
$ make exp1
```

Optionally, run with docker. First build a container as follows: 

```
$ make docker
```

Then run with the experiment you want to run (or leaving it out to run all):

```
$ docker run --rm -t knaw-huc/stam-experiments exp1 
```

