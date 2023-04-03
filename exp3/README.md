# Experiment 3: FoLiA to STAM conversion

This experiment is centered around folia2stam, part of [foliatools](httsp://github.com/annotation) and converts a small but heavily annotated FoLiA XML text to STAM. The tool effectively 'untangles' FoLiA, producing a plain text file and a STAM JSON or STAM CSV file. It builds upon [stam-python](https://github.com/annotation/stam-python) as well as [foliapy](https://github.com/proycon/foliapy).

Please inspect the [Makefile](Makefile) for details.

## Usage

Just run ``make`` to run the experiment (``make clean`` to remove results again)

You may want to run this in the provided Docker container (one level up).
