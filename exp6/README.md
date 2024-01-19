# STAM Visualisations

This experiment shows the `stam view` command line tool (part of [stam tools](https://github.com/annotation/stam-tools)) to view annotations in HTML.

1. [stamvis1.html](stamvis1.html) - Shows selection of part-of-speech tags, lemmas en named entities in the resource.
2. [stamvis2.html](stamvis2.html) - Shows all PoS, lemma and named entities in the resource. Demonstrates *keys*.
3. [stamvis3.html](stamvis3.html) - The same annotations as 1, but per sentence rather than per resource.

Queries are formulated in [STAMQL](https://github.com/annotation/stam/tree/master/extensions/stam-query), you can find them in the Makefile or the comments in the resulting HTML source code.

The example document is produced by Frog, and converted from FoLiA XML to STAM, using [folia2stam](https://github.com/proycon/foliatools).
