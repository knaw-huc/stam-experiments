all: exp1.store.stam.json

simpletagger.tsv: 
	@ln -s ../config/stam-tsv/simpletagger.tsv

smallquote.txt: 
	@ln -s ../texts/smallquote.txt

exp1.store.stam.json: smallquote.txt simpletagger.tsv
	@echo -e "STAGE 1/6 ---------------------">&2
	stam init --resource smallquote.txt exp1.store.stam.json

	@echo -e "\n\nSTAGE 2/6-------------------------------">&2
	stam info exp1.store.stam.json

	@echo -e "\n\nSTAGE 3/6 -------------------------------">&2
	stam tag --rules simpletagger.tsv exp1.store.stam.json

	@echo -e "\n\nSTAGE 4/6 -------------------------------">&2
	stam validate --verbose exp1.store.stam.json | jq

	@echo -e "\n\nSTAGE 5/6 -------------------------------">&2
	stam info --verbose exp1.store.stam.json

	@echo -e "\n\nSTAGE 6/6 -------------------------------">&2
	stam to-tsv --verbose exp1.store.stam.json

clean:
	-rm exp1.store.stam.json


	
