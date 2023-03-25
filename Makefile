all: exp1

.PHONY: clean
clean:
	cd exp1; make clean; cd ..

.PHONY: exp1
exp1:
	cd exp1; make; cd ..
