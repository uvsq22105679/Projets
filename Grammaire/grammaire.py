import string,re,sys

class Grammaire():
    def __init__(self,axiome : str,regles : dict):
        self.non_terminaux_generaux = [lettre+str(chiffre) for chiffre in [0,1,2,3,4,5,6,7,8,9] for lettre in string.ascii_uppercase if lettre != "E"]
        self.terminaux_generaux = list(string.ascii_lowercase)
        self.nos_terminaux = set(lettre for values in regles.values() for regle in values for lettre in regle if lettre in self.terminaux_generaux)
        self.nos_non_terminaux = (regles.keys())
        self.axiome = axiome
        self.regles = regles

    def decomposer_regle(self,regle):
        print(type(regle))
        pattern = r'[a-z]|[A-Z][0-9]'
        return re.findall(pattern, regle)
      
    def suppression_terminales(self):
        modification={}
        nouvels_regles={}
        for elem in self.nos_terminaux:
            for non_ter in self.non_terminaux_generaux :
                if non_ter not in self.nos_non_terminaux and non_ter not in list(modification.values()):
                    val=non_ter
                    break
            modification[elem]=val 
        for key,values in self.regles.items():
            for i,val in enumerate(values):
                valeur=self.decomposer_regle(val)
                temp=[]
                for elem in valeur:
                    for lettre in elem:
                        if lettre in self.nos_terminaux:
                            temp.append(lettre)
                if len(valeur)>1:
                    for l in set(temp):
                        values[i]=values[i].replace(l,modification[l])
                        nouvels_regles[modification[l]]=l
        for key,value in nouvels_regles.items():
            self.regles[key]=value     
                                                    
    def suppression_membres_droits(self):
        modification=[]
        non_term_selectionne=[]
        for key,values in self.regles.items():
            for i,val in enumerate(values):
                decomp_val=self.decomposer_regle(val)
                if len(decomp_val)>2 :
                    for non_ter in self.non_terminaux_generaux:
                        if non_ter not in self.nos_non_terminaux and non_ter not in non_term_selectionne:
                            new_key=non_ter
                            non_term_selectionne.append(non_ter)
                            break
                    new_val="".join(val[2::])
                    modification.append((key, i, new_key, new_val))
        for key, i, new_key, new_val in modification:
            self.regles[new_key] = [new_val]  
            self.regles[key][i] = self.regles[key][i][0:2] + new_key
  
    def suppression_unitaire(self):
        for key,values in self.regles.items():
            for val in values:
                if len(val)==2 and val in self.nos_non_terminaux: 
                    values.remove(val)
                    for elem in self.regles[val]:
                        if elem not in self.regles[key]:
                            self.regles[key].append(elem)

    def prochain_non_term_dispo(self):
        nb_nt = len(self.nos_non_terminaux)
        prc_nt = "S0"
        for _ in range(nb_nt):
            if prc_nt in self.nos_non_terminaux :
                if "9" in prc_nt :
                    asc = ord(prc_nt[0])+1
                    if asc > 90:
                        asc = 65
                    elif asc == 69:
                        asc = 70
                    prc_nt = chr(asc) + "0"
                else :
                    prc_nt = prc_nt[0] + str(int(prc_nt[1])+1)
        return prc_nt
    
    def liste_regle(self):
        liste = []
        for NT, production in self.regles.items():
            i = 0
            for regle in production:
                liste.append((NT,regle,i))
                i += 1
        return liste
        
    def retirer_axiome_de_droite(self):
        est_a_droite = False
        for regle in self.liste_regle():
            if self.axiome in regle[1]:
                est_a_droite = True
        if est_a_droite:
            prc_nt = self.prochain_non_term_dispo()
            self.regles[prc_nt] = [self.axiome]
            self.axiome = prc_nt
        self.regles = verif_doublon(self.regles)

    def supr_epsilon(self):
        def donne(regle, element : str):
            nb_elem = regle.replace(element, "E").count("E")
            liste = []
            i = 0
            while i < len(regle):
                if regle[i:i+len(element)] == element:
                    liste.append(element)
                    i += len(element)
                else:
                    liste.append(regle[i])
                    i += 1
            combinaisons = [['' if (i >> j) & 1 else element for j 
                            in range(nb_elem)] for i in range(2**nb_elem)]
            liste_final = [''.join(temp if temp != element 
                            else combinaison.pop(0) for temp in liste) 
                            for combinaison in combinaisons]
            for i in range(len(liste_final)):
                if liste_final[i] == '':
                    liste_final[i] = 'E'
            return liste_final

        liste_epsilon = [1]
        while len(liste_epsilon) != 0:
            liste_epsilon = []
            for regle in self.liste_regle():
                if regle[1] == "E":
                    if regle[0] != self.axiome:
                        liste_epsilon.append(regle[0])
            for elem in liste_epsilon:
                self.regles[elem].remove("E")
            for regle in self.liste_regle():
                for elem in liste_epsilon:
                    if elem in regle[1]:
                        self.regles[regle[0]] += (donne(regle[1],elem))
        self.regles = verif_doublon(self.regles)

    def supr_regle_1_1(self):
        for i in range(2):
            dico_des_modif = {}
            for NT, production in self.regles.items():
                for i in range(len(production)) :
                    if len(production[i]) == 2 and production[i] in self.nos_non_terminaux:
                        dico_des_modif[NT] = self.regles[NT][:i] + self.regles[production[i]] + self.regles[NT][i+1:]
            for elem in dico_des_modif.keys():
                self.regles[elem] = dico_des_modif[elem]
            self.regles = verif_doublon(self.regles)

    def supr_non_terminaux_tete(self):
        liste = []
        for NT, prod in self.regles.items():
            for i in range(len(prod)):
                if prod[i][0] not in self.terminaux_generaux and prod[i][0] != 'E':
                    if prod[i][:2] == NT:
                        liste.append((NT,prod[i][2:],prod[:i]+prod[i+1:]))
        for rec in liste:
            prc_nt = self.prochain_non_term_dispo()
            self.regles[rec[0]] = [regle + prc_nt for regle in rec[2]]
            self.regles[prc_nt] = [rec[1]+ rec[0],'E']
        self.supr_epsilon()
        for NT, prod in self.regles.items():
            for i in range(len(prod)):
                if prod[i][:2] in self.non_terminaux_generaux:
                    self.regles[NT] = prod[:i] + [regle + prod[i][2:] for regle in self.regles[prod[i][:2]]]+ prod[i+1:]

    def supr_terminaux_pas_tete(self):
        dico = self.regles.copy()
        terminal_to_non_term = {}
        for NT, productions in self.regles.items():
            for i in range(len(productions)):
                production = productions[i]        
                for j in range(1, len(production)):
                    elem = production[j]
                    if elem in self.nos_terminaux:
                        if elem not in terminal_to_non_term:
                            new_non_term = self.prochain_non_term_dispo()
                            dico[new_non_term] = [elem]
                            terminal_to_non_term[elem] = new_non_term
                            self.nos_non_terminaux = dico.keys()
                        else:
                            new_non_term = terminal_to_non_term[elem]
                        dico[NT][i] = production[:j] + new_non_term + production[j+1:]
        self.regles = verif_doublon(dico)

def verif_doublon(dico : dict):
    for NT in dico.keys():
        dico[NT] = list(set(dico[NT]))
    return dico
        
def lire(file : str):
    dico={}
    with open(file,'r') as f:
        lignes=f.readlines()
        for ligne in lignes:
            ligne = ligne.replace(' ','').replace('\n','').split(":")
            if ligne[0] not in dico.keys():
                dico[ligne[0]] = []
            dico[ligne[0]].append(ligne[1])
    return dico

def chomsky():
    donnees=lire(sys.argv[1])
    axiome = list(donnees.keys())[0]
    gram = Grammaire(axiome,donnees)
    gram.retirer_axiome_de_droite()
    gram.suppression_terminales()
    gram.suppression_membres_droits()
    gram.supr_epsilon()
    gram.suppression_unitaire()
    return gram

def greibach():
    donnees=lire(sys.argv[1])
    axiome = list(donnees.keys())[0]
    gram=Grammaire(axiome,donnees)
    gram.retirer_axiome_de_droite()
    gram.supr_epsilon()
    gram.supr_regle_1_1()
    gram.supr_non_terminaux_tete()
    gram.supr_terminaux_pas_tete()
    return gram

def ecrire(gram : Grammaire,forme : str):
    with open(str('Grammaire.'+forme),'w') as file:
        liste_deja_parcouru = []
        liste_NT = [gram.axiome]
        while liste_NT != []:
            file.write(liste_NT[0] + ' -> ')
            i = 0
            for val in gram.regles[liste_NT[0]]:
                if i< len(gram.regles[liste_NT[0]])-1:
                    file.write(val + ' | ')
                else:
                    file.write(val+'\n')
                for elem in gram.nos_non_terminaux:
                    if elem in val and elem not in liste_deja_parcouru and elem not in liste_NT:
                        liste_NT.append(elem)
                i += 1
            liste_deja_parcouru.append(liste_NT.pop(0))

if __name__ == "__main__":
    gramm_chomsky = chomsky()
    gramm_greibach = greibach()
    ecrire(gramm_chomsky, 'chomsky')
    ecrire(gramm_greibach, 'greibach')