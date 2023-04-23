all: exp2.store.stam.json exp2.store.stam.csv exp2.compact.store.stam.json exp2.compact.store.stam.json.gz exp2.tsv

simpletagger.tsv: 
	@ln -s ../config/stam-tag/simpletagger.tsv

verne.txt: 
	@curl https://www.gutenberg.org/cache/epub/164/pg164.txt > verne.txt

exp2.store.stam.json: verne.txt simpletagger.tsv
	@echo -e "STAGE 1/3 ---------------------">&2
	stam init --resource verne.txt exp2.store.stam.json

	@echo -e "\n\nSTAGE 2/3 -------------------------------">&2
	time stam tag --rules simpletagger.tsv exp2.store.stam.json

	@echo -e "\n\nSTAGE 3/3 -------------------------------">&2
	time stam info exp2.store.stam.json
	@echo -e "\n\n-----------------------------------------">&2


exp2.store.stam.csv: exp2.store.stam.json
	@echo -e "\n\nSTAGE 1/2 -------------------------------">&2
	time stam save -o exp2.store.stam.csv exp2.store.stam.json
	@echo -e "\n\n-----------------------------------------">&2

	@echo -e "\n\nSTAGE 2/2 -------------------------------">&2
	time stam info exp2.store.stam.csv
	@echo -e "\n\n-----------------------------------------">&2

exp2.tsv: exp2.store.stam.csv
	time stam to-tsv -C Id,Text,TextSelection,simpletokens/type $< > $@

exp2.compact.store.stam.json: exp2.store.stam.json
	jq -c < $< > $@

%.gz: %
	gzip -fk $< > $@

clean:
	-rm exp2*json exp2*csv


	
