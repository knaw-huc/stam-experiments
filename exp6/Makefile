all: stamvis1.html stamvis2.html stamvis3.html stamvis4.html stamvis5.html stamvis5.tsv stamvis6.html stamvis6.tsv

stamvis1.html:
    #Show selection of part-of-speech tags, lemmas en named entities in the resource.
	stam view --query "SELECT RESOURCE ?res" \
			--query "SELECT ANNOTATION ?noun WHERE RESOURCE ?res; DATA \"https://raw.githubusercontent.com/proycon/folia/master/setdefinitions/frog-mbpos-cgn\" \"head\" = \"N\";" \
			--query "SELECT ANNOTATION ?verb WHERE RESOURCE ?res; DATA \"https://raw.githubusercontent.com/proycon/folia/master/setdefinitions/frog-mbpos-cgn\" \"head\" = \"WW\";" \
			--query "SELECT ANNOTATION ?name WHERE RESOURCE ?res; DATA \"https://raw.githubusercontent.com/proycon/folia/master/setdefinitions/frog-ner-nl\" \"class\";" \
			frogdeep.store.stam.json > $@

stamvis2.html:
    #Show all PoS, lemma and named entities in the resource. Demonstrates *keys*.
	stam view --query "SELECT RESOURCE ?res" \
			--query "@VALUE SELECT ANNOTATION ?pos WHERE RESOURCE ?res; DATA \"https://raw.githubusercontent.com/proycon/folia/master/setdefinitions/frog-mbpos-cgn\" \"head\";" \
			--query "@VALUE SELECT ANNOTATION ?lemma WHERE RESOURCE ?res; DATA \"https://raw.githubusercontent.com/proycon/folia/master/setdefinitions/frog-mblem-nl\" \"class\";" \
			--query "@VALUE SELECT ANNOTATION ?name WHERE RESOURCE ?res; DATA \"https://raw.githubusercontent.com/proycon/folia/master/setdefinitions/frog-ner-nl\" \"class\"" \
			frogdeep.store.stam.json > $@

stamvis3.html:
	#The same annotations as 1, but per sentence rather than per resource.
	stam view --query "SELECT ANNOTATION ?sentence WHERE DATA \"https://w3id.org/folia/\" \"annotationtype\" = \"sentence\";" \
        	--query "SELECT ANNOTATION ?noun WHERE RELATION ?sentence EMBEDS; DATA \"https://raw.githubusercontent.com/proycon/folia/master/setdefinitions/frog-mbpos-cgn\" \"head\" = \"N\";" \
			--query "SELECT ANNOTATION ?verb WHERE RELATION ?sentence EMBEDS; DATA \"https://raw.githubusercontent.com/proycon/folia/master/setdefinitions/frog-mbpos-cgn\" \"head\" = \"WW\";" \
			--query "SELECT ANNOTATION ?name WHERE RELATION ?sentence EMBEDS; DATA \"https://raw.githubusercontent.com/proycon/folia/master/setdefinitions/frog-ner-nl\" \"class\";" \
			frogdeep.store.stam.json > $@

stamvis4.html:
	#Show some structural types.
	stam view --query "SELECT RESOURCE ?res" \
			--query "@VALUE SELECT ANNOTATION ?paragraph WHERE RESOURCE ?res; DATA \"https://w3id.org/folia/\" \"annotationtype\" = \"paragraph\";" \
			--query "@VALUE SELECT ANNOTATION ?sentence WHERE RESOURCE ?res; DATA \"https://w3id.org/folia/\" \"annotationtype\" = \"sentence\";" \
			--query "SELECT ANNOTATION ?word WHERE RESOURCE ?res; DATA \"https://w3id.org/folia/\" \"annotationtype\" = \"token\";" \
			frogdeep.store.stam.json > $@

stamvis5.html:
	#Highlight all occurrences of the word "van" (textual search)
	stam view --query "SELECT RESOURCE ?res" \
			--query "SELECT ANNOTATION ?match WHERE RESOURCE ?res; TEXT \"van\";" \
			frogdeep.store.stam.json > $@

stamvis5.tsv:
	#Get all occurrences of the word "van" (textual search)
	stam query --query "SELECT ANNOTATION ?word WHERE TEXT \"van\"; DATA \"https://w3id.org/folia/\" \"annotationtype\" = \"token\";" \
			frogdeep.store.stam.json > $@

stamvis6.html:
	#Show all and only named entities 
	stam view --auto --query "SELECT ANNOTATION ?name WHERE DATA \"https://raw.githubusercontent.com/proycon/folia/master/setdefinitions/frog-ner-nl\" \"class\";" \
			frogdeep.store.stam.json > $@

stamvis6.tsv:
	#Show all and only named entities 
	stam query --query "SELECT ANNOTATION ?name WHERE DATA \"https://raw.githubusercontent.com/proycon/folia/master/setdefinitions/frog-ner-nl\" \"class\";" \
			frogdeep.store.stam.json > $@
