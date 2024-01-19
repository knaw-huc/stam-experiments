all: exp1 exp2 exp3 exp6

.PHONY: clean
clean:
	cd exp1; make clean; cd ..

.PHONY: exp1
exp1:
	cd exp1; make; cd ..

.PHONY: exp2
exp2:
	cd exp2; make; cd ..

.PHONY: exp3
exp3:
	cd exp3; make; cd ..

.PHONY: exp6
exp6:
	cd exp6; make; cd ..

docker:
	docker build -t knaw-huc/stam-experiments . 
