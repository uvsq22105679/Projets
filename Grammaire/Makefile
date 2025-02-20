all :
	python3 grammaire.py Grammaire1.general
	python3 generer.py 4 Grammaire.chomsky > Grammaire1_4_chomsky.res
	python3 generer.py 4 Grammaire.greibach > Grammaire1_4_greibach.res
	diff Grammaire1_4_chomsky.res Grammaire1_4_greibach.res

	python3 grammaire.py Grammaire2.general
	python3 generer.py 4 Grammaire.chomsky > Grammaire2_4_chomsky.res
	python3 generer.py 4 Grammaire.greibach > Grammaire2_4_greibach.res
	diff Grammaire2_4_chomsky.res Grammaire2_4_greibach.res

	python3 grammaire.py Grammaire3.general
	python3 generer.py 4 Grammaire.chomsky > Grammaire3_4_chomsky.res
	python3 generer.py 4 Grammaire.greibach > Grammaire3_4_greibach.res
	diff Grammaire3_4_chomsky.res Grammaire3_4_greibach.res

	python3 grammaire.py Grammaire4.general
	python3 generer.py 4 Grammaire.chomsky > Grammaire4_4_chomsky.res
	python3 generer.py 4 Grammaire.greibach > Grammaire4_4_greibach.res
	diff Grammaire4_4_chomsky.res Grammaire4_4_greibach.res

clean :
	rm -f Grammaire.chomsky
	rm -f Grammaire.greibach
	rm -f Grammaire1_4_chomsky.res
	rm -f Grammaire1_4_greibach.res
	rm -f Grammaire2_4_chomsky.res
	rm -f Grammaire2_4_greibach.res
	rm -f Grammaire3_4_chomsky.res
	rm -f Grammaire3_4_greibach.res
	rm -f Grammaire4_4_chomsky.res
	rm -f Grammaire4_4_greibach.res