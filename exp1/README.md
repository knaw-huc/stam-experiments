# Experiment 1: STAM Tools on a very small text

This experiment invokes most of the [STAM Tools](https://github.com/annotation/stam-tools) as available at the time of writing.

It demonstrates:

1. `stam init`: initializing a stam annotation store from scratch, adding only a single small input text
2. `stam info`: info on the annotation store (containing nothing but the text resource at this point)
3. `stam tag`: run a simple regular-expression based tagger on the resources, effectively tokenizing it and producing STAM data
4. `stam validate`: validate all of the resulting STAM JSON
5. `stam info`: info on the annotation store at this point
6. `stam to-tsv`: output the annotations in the store to tsv 

Please inspect the [Makefile](Makefile) for details.

## Usage

Just run ``make`` to run the experiment (``make clean`` to remove results again)

You may want to run this in the provided Docker container (one level up).


