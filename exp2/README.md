# Experiment 2: STAM Tools on a book

This experiment invokes most of the [STAM Tools](https://github.com/annotation/stam-tools) on a book by Jules Verne (Twenty Thousand Leagues under the Sea). It also shows STAM CSV output.

It demonstrates:

1. `stam init`: initializing a stam annotation store from scratch, adding only a single small input text
3. `stam tag`: run a simple regular-expression based tagger on the resources, effectively tokenizing it and producing STAM data. The tagging is on on the basis of [these simple rules](../config/stam-tag/simpletagger.tsv).
5. `stam info`: info on the annotation store at this point
4. `stam save`: saving the annotation store to STAM CSV (this tools allows easy switching between STAM JSON and STAM CSV)

Please inspect the [Makefile](Makefile) for details.

## Usage

Just run ``make`` to run the experiment (``make clean`` to remove results again)

You may want to run this in the provided Docker container (one level up).

## File sizes

The book contains 107750 words (12807 lines).

A side-product of this experiment is that it provides some insight in the different serialized file sizes (STAM JSON vs STAM CSV):

```
 624K  verne.txt

  160  exp2.store.stam.csv
  11M  exp2.annotations.stam.csv
   93  simpletokens.annotationset.stam.csv

  62M  exp2.store.stam.json

  35M  exp2.compact.store.stam.json
```

The JSON comes in two variants, pretty printed (default) and compacted.

For rough comparison a tokenized FoLiA XML document was generated with ucto (outside this experiment):

```
  14M verne.folia.xml
```

If file size is a concern, a lot can be gained by simple gzip compression:

```
2.6M  exp2.annotations.stam.csv.gz

2.8M  exp2.store.stam.json.gz
```



