all: stamvis1.html stamvis2.html stamvis3.html

stamvis1.html:
	stam view --query "SELECT RESOURCE ?res" \
			--query "SELECT ANNOTATION ?noun WHERE RESOURCE ?res; DATA \"https://raw.githubusercontent.com/proycon/folia/master/setdefinitions/frog-mbpos-cgn\" \"head\" = \"N\";" \
			--query "SELECT ANNOTATION ?verb WHERE RESOURCE ?res; DATA \"https://raw.githubusercontent.com/proycon/folia/master/setdefinitions/frog-mbpos-cgn\" \"head\" = \"WW\";" \
			--query "SELECT ANNOTATION ?name WHERE RESOURCE ?res; DATA \"https://raw.githubusercontent.com/proycon/folia/master/setdefinitions/frog-ner-nl\" \"class\";" \
			frogdeep.store.stam.json > $@

stamvis2.html:
	stam view --query "SELECT RESOURCE ?res" \
			--query "@VALUE SELECT ANNOTATION ?pos WHERE RESOURCE ?res; KEY \"https://raw.githubusercontent.com/proycon/folia/master/setdefinitions/frog-mbpos-cgn\" \"head\";" \
			--query "@VALUE SELECT ANNOTATION ?lemma WHERE RESOURCE ?res; KEY \"https://raw.githubusercontent.com/proycon/folia/master/setdefinitions/frog-mblem-nl\" \"class\";" \
			--query "@VALUE SELECT ANNOTATION ?name WHERE RESOURCE ?res; KEY \"https://raw.githubusercontent.com/proycon/folia/master/setdefinitions/frog-ner-nl\" \"class\"" \
			frogdeep.store.stam.json > $@

stamvis3.html:
	stam view --query "SELECT ANNOTATION ?sentence WHERE DATA \"https://w3id.org/folia/\" \"annotationtype\" = \"sentence\";" \
        	--query "SELECT ANNOTATION ?noun WHERE RELATION ?sentence EMBEDS; DATA \"https://raw.githubusercontent.com/proycon/folia/master/setdefinitions/frog-mbpos-cgn\" \"head\" = \"N\";" \
			--query "SELECT ANNOTATION ?verb WHERE RELATION ?sentence EMBEDS; DATA \"https://raw.githubusercontent.com/proycon/folia/master/setdefinitions/frog-mbpos-cgn\" \"head\" = \"WW\";" \
			--query "SELECT ANNOTATION ?name WHERE RELATION ?sentence EMBEDS; DATA \"https://raw.githubusercontent.com/proycon/folia/master/setdefinitions/frog-ner-nl\" \"class\";" \
			frogdeep.store.stam.json > $@

clean:
	rm stamvis*.html
