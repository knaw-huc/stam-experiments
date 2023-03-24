all: exp1.store.stam.json

simpletagger.tsv: 
	@ln -s ../config/stam-tsv/simpletagger.tsv

smallquote.txt: 
	@ln -s ../texts/smallquote.txt

exp1.store.stam.json: smallquote.txt simpletagger.tsv
	@echo -e "STAGE 1/5 ---------------------">&2
	stam init --resource smallquote.txt exp1.store.stam.json

	@echo -e "\n\nSTAGE 2/5-------------------------------">&2
	stam info exp1.store.stam.json

	@echo -e "\n\nSTAGE 3/5 -------------------------------">&2
	stam validate exp1.store.stam.json

	@echo -e "\n\nSTAGE 4/5 -------------------------------">&2
	stam tag --rules simpletagger.tsv exp1.store.stam.json

	@echo -e "\n\nSTAGE 5/5 -------------------------------">&2
	stam info exp1.store.stam.json

clean:
	-rm exp1.store.stam.json


	
