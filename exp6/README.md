# STAM Visualisations

This experiment shows the `stam view` command line tool (part of [stam tools](https://github.com/annotation/stam-tools)) to view annotations in HTML.

1. [stamvis1.html](stamvis1.html) - Shows selection of part-of-speech tags, lemmas en named entities in the resource.
2. [stamvis2.html](stamvis2.html) - Shows all PoS, lemma and named entities in the resource. Demonstrates *keys*.
3. [stamvis3.html](stamvis3.html) - The same annotations as 1, but per sentence rather than per resource.
4. [stamvis4.html](stamvis4.html) - Shows some structural types.
5. [stamvis5.html](stamvis5.html) - Highlights all occurrences of the word "van" (textual search)
6. [stamvis6.html](stamvis6.html) - Shows all (and only) named entities

Queries are formulated in [STAMQL](https://github.com/annotation/stam/tree/master/extensions/stam-query), you can find them in the Makefile or the comments in the resulting HTML source code.

The example document is produced by Frog, and converted from FoLiA XML to STAM, using [folia2stam](https://github.com/proycon/foliatools).
