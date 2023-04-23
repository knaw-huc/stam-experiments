all: exp3.store.stam.json exp3.store.stam.csv exp3.compact.store.stam.json

exp3.folia.xml:
	@curl https://raw.githubusercontent.com/proycon/folia/master/examples/frog-deep-upgraded.2.0.2.folia.xml > $@

exp3.store.stam.json: exp3.folia.xml 
	time folia2stam --id exp3 exp3.folia.xml

	@echo -e "\n\n-----------------------------------------">&2

	time stam validate exp3.store.stam.json

exp3.store.stam.csv: exp3.store.stam.json
	time stam save -o exp3.store.stam.csv exp3.store.stam.json

	@echo -e "\n\n-----------------------------------------">&2

	time stam info exp3.store.stam.csv

exp3.compact.store.stam.json: exp3.store.stam.json
	jq -c < $< > $@

clean:
	-rm exp3*json exp3*csv


	
