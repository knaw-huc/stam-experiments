all: stamvis1.html stamvis2.html stamvis3.html stamvis4.html stamvis5.html stamvis5.tsv stamvis6.html stamvis6.tsv

stamvis1.html: frogdeep.store.stam.json
    #Show selection of part-of-speech tags, lemmas en named entities in the resource.
	stam view --query "SELECT RESOURCE ?res" \
			--query "SELECT ANNOTATION ?noun WHERE RESOURCE ?res; DATA \"https://raw.githubusercontent.com/proycon/folia/master/setdefinitions/frog-mbpos-cgn\" \"head\" = \"N\";" \
			--query "SELECT ANNOTATION ?verb WHERE RESOURCE ?res; DATA \"https://raw.githubusercontent.com/proycon/folia/master/setdefinitions/frog-mbpos-cgn\" \"head\" = \"WW\";" \
			--query "SELECT ANNOTATION ?name WHERE RESOURCE ?res; DATA \"https://raw.githubusercontent.com/proycon/folia/master/setdefinitions/frog-ner-nl\" \"class\";" \
			$< > $@

stamvis2.html: frogdeep.store.stam.json
    #Show all PoS, lemma and named entities in the resource. Demonstrates *keys*.
	stam view --query "SELECT RESOURCE ?res" \
			--query "@VALUETAG SELECT ANNOTATION ?pos WHERE RESOURCE ?res; DATA \"https://raw.githubusercontent.com/proycon/folia/master/setdefinitions/frog-mbpos-cgn\" \"head\";" \
			--query "@VALUETAG SELECT ANNOTATION ?lemma WHERE RESOURCE ?res; DATA \"https://raw.githubusercontent.com/proycon/folia/master/setdefinitions/frog-mblem-nl\" \"class\";" \
			--query "@VALUETAG SELECT ANNOTATION ?name WHERE RESOURCE ?res; DATA \"https://raw.githubusercontent.com/proycon/folia/master/setdefinitions/frog-ner-nl\" \"class\"" \
			$< > $@

stamvis3.html: frogdeep.store.stam.json
	#The same annotations as 1, but per sentence rather than per resource.
	stam view --query "SELECT ANNOTATION ?sentence WHERE DATA \"https://w3id.org/folia/\" \"annotationtype\" = \"sentence\";" \
        	--query "SELECT ANNOTATION ?noun WHERE RELATION ?sentence EMBEDS; DATA \"https://raw.githubusercontent.com/proycon/folia/master/setdefinitions/frog-mbpos-cgn\" \"head\" = \"N\";" \
			--query "SELECT ANNOTATION ?verb WHERE RELATION ?sentence EMBEDS; DATA \"https://raw.githubusercontent.com/proycon/folia/master/setdefinitions/frog-mbpos-cgn\" \"head\" = \"WW\";" \
			--query "SELECT ANNOTATION ?name WHERE RELATION ?sentence EMBEDS; DATA \"https://raw.githubusercontent.com/proycon/folia/master/setdefinitions/frog-ner-nl\" \"class\";" \
			$< > $@

stamvis4.html: frogdeep.store.stam.json
	#Show some structural types.
	stam view --query "SELECT RESOURCE ?res" \
			--query "@IDTAG SELECT ANNOTATION ?paragraph WHERE RESOURCE ?res; DATA \"https://w3id.org/folia/\" \"annotationtype\" = \"paragraph\";" \
			--query "@IDTAG SELECT ANNOTATION ?sentence WHERE RESOURCE ?res; DATA \"https://w3id.org/folia/\" \"annotationtype\" = \"sentence\";" \
			--query "SELECT ANNOTATION ?word WHERE RESOURCE ?res; DATA \"https://w3id.org/folia/\" \"annotationtype\" = \"token\";" \
			$< > $@

stamvis5.html: frogdeep.store.stam.json
	#Highlight all occurrences of the word "van" (textual search)
	stam view --query "SELECT RESOURCE ?res" \
			--query "SELECT ANNOTATION ?match WHERE RESOURCE ?res; TEXT \"van\";" \
			$< > $@

stamvis5.tsv: frogdeep.store.stam.json
	#Get all occurrences of the word "van" (textual search)
	stam query --query "SELECT ANNOTATION ?word WHERE TEXT \"van\"; DATA \"https://w3id.org/folia/\" \"annotationtype\" = \"token\";" \
			$< > $@

stamvis6.html: frogdeep.store.stam.json
	#Show all and only named entities 
	stam view --auto --query "SELECT ANNOTATION ?name WHERE DATA \"https://raw.githubusercontent.com/proycon/folia/master/setdefinitions/frog-ner-nl\" \"class\";" \
			$< > $@

stamvis6.tsv: frogdeep.store.stam.json
	#Show all and only named entities 
	stam query --query "SELECT ANNOTATION ?name WHERE DATA \"https://raw.githubusercontent.com/proycon/folia/master/setdefinitions/frog-ner-nl\" \"class\";" \
			$< > $@
