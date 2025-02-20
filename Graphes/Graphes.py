"""
Backbone réseau (Tier1):
Dans le domaine des réseaux informatiques, le backbone (ou épine dorsale) désigne la partie centrale et principale d'un réseau,
qui relie ensemble différents réseaux locaux (LAN). C'est une infrastructure à haut débit qui assure la connectivité entre les
différents éléments du réseau.

Opérateurs de transit (Tier2):
Egalement appelés fournisseurs de transit Internet ou carriers IP (Internet Protocol), sont des entreprises qui fournissent
des services de connectivité réseau à d'autres fournisseurs de services Internet (ISP) ou à des réseaux d'entreprise. Ces
opérateurs jouent un rôle essentiel dans l'interconnexion des réseaux informatiques à l'échelle mondiale.

Les opérateurs de niveau 3 (Tier 3):
Ils représentent une couche importante de l'infrastructure Internet, interconnectant les utilisateurs finaux et les réseaux
d'entreprise avec les opérateurs de niveau supérieur. La configuration du réseau décrite assure une connectivité efficace et fiable
en utilisant des connexions hiérarchiques avec des valeurs de liens attribuées.
"""

import random as rd
import numpy as np
import tkinter as tk

class Noeud():
    def __init__(self,noeud,tier):
        self.noeud=noeud
        self.tier=tier
    
    def get_noeud(self):
        return self.noeud
    def get_tier(self):
        return self.tier

class Arete():
    def __init__(self, n1, n2, poids):
        self.poids = poids
        self.noeud = (n1,n2)

    def get_noeud(self):
        return self.noeud
    def get_poids(self):
        return self.poids

class Schema():
    def __init__(self,mat,aretes):
        self.aretes=aretes
        self.root=tk.Tk()
        self.root.title('Algorithmique des graphes')
        self.mat=mat
        self.taille_sommets=[]
        self.sw,self.sh = self.root.winfo_screenwidth(),self.root.winfo_screenheight()
        self.root.geometry(f"{self.sw}x{self.sh}")
        self;self.frame=tk.Canvas(self.root,width=self.sw-200,height=self.sh)
        self.canva=tk.Canvas(self.root,width=200,height=700)
        self.sw -= 200
        self.sh -= 20
    def main(self):
        '''
        Fonction main: création des noeuds et des aretes les valeurs de leur valeurs sont inserés dans une liste que l'ont utilisera
        pour afficher les noeuds sur l'interface graphique
        '''
        for i in range(100):

            place = [(self.sh//2-70,self.sw//2),(self.sh//2+70,self.sw//2),(self.sh//2+56,self.sw//2-42),(self.sh//2+56,self.sw//2+42),(self.sh//2-56,self.sw//2+42),(self.sh//2-56,self.sw//2-42),(self.sh//2+21,self.sw//2-63),(self.sh//2-21,self.sw//2-63),(self.sh//2-21,self.sw//2+63),(self.sh//2+21,self.sw//2+63),
                    (self.sh//2-120,self.sw//2),(self.sh//2+120,self.sw//2),(self.sh//2+96,self.sw//2-72),(self.sh//2+96,self.sw//2+72),(self.sh//2-96,self.sw//2+72),(self.sh//2-96,self.sw//2-72),(self.sh//2+36,self.sw//2-108),(self.sh//2-36,self.sw//2-108),(self.sh//2-36,self.sw//2+108),(self.sh//2+36,self.sw//2+108),
                    (self.sh//2,self.sw//2-120),(self.sh//2,self.sw//2+120),(self.sh//2-72,self.sw//2+96),(self.sh//2+72,self.sw//2+96),(self.sh//2+72,self.sw//2-96),(self.sh//2-72,self.sw//2-92),(self.sh//2-108,self.sw//2+36),(self.sh//2-108,self.sw//2-36),(self.sh//2+108,self.sw//2-36),(self.sh//2+108,self.sw//2+36)]
            if 0<=i<10:
                y,x= place[i][0],place[i][1]
                self.taille_sommets.append((x,y))
            elif 10<=i<30:
                y,x=place[i][0],place[i][1]
                self.taille_sommets.append((x,y))
            else:
                endroit = rd.randint(0,5)
                if endroit == 0 or endroit == 1:
                    x,y=rd.randint(20,self.sw-40),rd.randint(20,self.sh//3-20)
                    self.taille_sommets.append((x,y))
                elif endroit == 2:
                    x,y = rd.randint(20,self.sw//2-150),rd.randint(self.sh//3,2*self.sh//3)
                    self.taille_sommets.append((x,y))
                elif endroit == 3:
                    x,y = rd.randint(self.sw//2+150,self.sw-40),rd.randint(self.sh//3,2*self.sh//3)
                    self.taille_sommets.append((x,y))
                else:
                    x,y=rd.randint(20,self.sw-40),rd.randint((2*self.sh//3)+20,self.sh-40)
                    self.taille_sommets.append((x,y))
        verif_posistion(self.taille_sommets)

    def reset(self):
        '''
        Fonction qui permet de reaffiche le schema original
        Fonction utilisé apres avoir afficher le chemin d'un noeud a un autre et que l'on souhaite affiche un nouveau chemin
        '''
        self.frame.delete('all')
        self.label.config(text=' ')
        self.affichage()
        self.canva.update()
        self.frame.update()

    def affichage(self):
        '''
        Fonction qui affiche les noeuds et les aretes ,crées plus tot, sur l'interface graphique
        '''
        self.label = tk.Label(self.canva, text=' ')
        self.label.place(x=10,y=200) 
        for l in range(100):
            if 0<=l<10:
                x,y = self.taille_sommets[l][0],self.taille_sommets[l][1]
                self.frame.create_oval(x-10,y-10,x+10,y+10,fill='red')
            elif 10<=l<30:
                x,y = self.taille_sommets[l][0],self.taille_sommets[l][1]
                self.frame.create_oval(x-10,y-10,x+10,y+10,fill='#4081e3')
            else:
                x,y = self.taille_sommets[l][0],self.taille_sommets[l][1]
                self.frame.create_oval(x-10,y-10,x+10,y+10,fill='light green')
        for ar in v_liste_arete:
            i,j=ar.get_noeud()[0],ar.get_noeud()[1]
            tag = f"{i},{j}"
            x1,y1=self.taille_sommets[i][0],self.taille_sommets[i][1]
            x2,y2=self.taille_sommets[j][0],self.taille_sommets[j][1]
            if 0<=ar.get_noeud()[0]<10:
                self.frame.create_line(x1,y1,x2,y2,fill='red',tags=tag)
            elif 10 <=ar.get_noeud()[0]<30 :
                self.frame.create_line(x1,y1,x2,y2,fill='purple',tags=tag)
            else:
                self.frame.create_line(x1,y1,x2,y2,fill='cyan',tags=tag)
        for k in range(100):
            x,y = self.taille_sommets[k][0],self.taille_sommets[k][1]
            self.frame.create_text(x,y, text = k, font=('Helvetica',10,'bold'))
    
    def affiche_chemin(self):
        '''
        Fonction qui affiche un texte qui énonce le chemin d'un noeud a un autre.
        Ainsi que les aretes concernés qui sont representés en noir
        '''
        self.noeud1 = int(self.entrer1.get())
        self.noeud2 = int(self.entrer2.get())
        texte=f"Pour aller du noeud {self.noeud1} au noeud {self.noeud2}:"
        l_chemin= chemin(self.noeud1,self.noeud2,self.mat)
        for i in range(len(l_chemin)):
            if l_chemin[i]==self.noeud2:
                break 
            else:
                if i ==0:
                    texte+=f"\n Commencé par {l_chemin[i]}"
                else:
                    texte+=f"\n puis  {l_chemin[i]}"
        for i in range(len(l_chemin)-1):
            if (l_chemin[i],l_chemin[i+1]) in self.aretes:
                tag = f"{l_chemin[i]},{l_chemin[i+1]}"
                target_line = self.frame.find_withtag(tag)
                if target_line:
                    self.frame.itemconfig(target_line, fill="black",width=5)
            elif(l_chemin[i+1],l_chemin[i]) in self.aretes :
                tag = (f"{l_chemin[i+1]},{l_chemin[i]}")
                target_line = self.frame.find_withtag(tag)
                self.frame.itemconfig(target_line, fill="black",width=5)
        self.label.config(text=texte)

    def entree(self):
        '''
        Fonction qui crée les entrées pour que l'utilisateur puisse inseré la valeur des noeuds dont il souhaite connaitre le chemin
        '''
        self.canva.create_text(30,50,text='Noeud 1:')
        self.canva.create_text(30,100,text='Noeud 2:')
        self.entrer1=tk.Entry(self.root)
        self.entrer2=tk.Entry(self.root)
        self.entrer1.place(x=60, y=45)
        self.entrer2.place(x=60, y=95)
        self.afficher=tk.Button(text='Afficher',command=self.affiche_chemin)
        self.afficher.place(x=50, y=150)
        self.reset=tk.Button(text='Reset',command=self.reset)
        self.reset.place(x=110,y=150)
        self.canva.grid(column=0,row=0)
        self.frame.grid(column=1, row=0, rowspan=5)  
        
    def run(self):
        '''
        Fonction qui permet le lancement et l'affichage géneral de l'interface graphique
        '''
        self.main()
        self.affichage()
        self.entree()
        self.root.mainloop()

def arete_t1():
    '''
    Fonction qui permet de crée les aretes des noeuds de tier 1
    Liste_inter: liste contenant les couples de noeud pour eviter d'avoir des doublons du type (a,b)(b,a)
    Les boucles for: permettent de sélectionner 2 noeuds
    rd.randint(1,100)<75: pour avoir une probabilité de 75%
    '''
    liste_inter = []
    for elem in liste_noeud:
        for elem2 in liste_noeud:
            if elem != elem2:
                if elem.get_tier() == elem2.get_tier():
                    if elem.get_tier() == 1:
                        if (elem2.get_noeud(), elem.get_noeud()) not in liste_inter and (elem.get_noeud(), elem2.get_noeud()) not in liste_inter:    
                            if rd.randint(1,100)<75:
                                v_liste_arete.append(Arete(elem.get_noeud(), elem2.get_noeud(), rd.randint(5,10)))
                                liste_inter.append((elem2.get_noeud(), elem.get_noeud()))
                            else:
                                liste_inter.append((elem2.get_noeud(), elem.get_noeud()))

def arete_t2():
    '''
    Fonction qui permet de crée les aretes des noeuds de tier 2

    Fonction compte(): compte le nombre de liens avec d'autres noeuds de tier 2

    1er boucle: création des liens avec les autres noeuds de tier 2 en vérifiant que le compte est inferieur à 3 ou a 2 car chaque noeud à 2 ou 3 liens
    On verifie également qu'ils sont du même tier et que le deuxieme noeud ne va pas depasser 3 liens

    2éme boucle: création des liens avec les noeuds de tier 1

    '''
    def compte(noeud):
        compteur = 0
        for elem in v_liste_arete:
            if noeud.get_noeud() in elem.get_noeud():
                compteur += 1
        return (compteur)
    
    liste_inter = []
    for elem in liste_noeud[len(liste_noeud)//10: (len(liste_noeud)//5 + len(liste_noeud)//10)-1]:
        while compte(elem) < rd.randint(2,3):
            elem2 = liste_noeud[rd.randint((len(liste_noeud)//10)+1, (len(liste_noeud)//5 + len(liste_noeud)//10))-1]
            if elem.get_noeud() != elem2.get_noeud() and compte(elem2) < 3:
                if (elem2.get_noeud(), elem.get_noeud()) not in liste_inter and (elem.get_noeud(), elem2.get_noeud()) not in liste_inter:
                    v_liste_arete.append(Arete(elem.get_noeud(), elem2.get_noeud(), rd.randint(10,20)))
                    liste_inter.append((elem.get_noeud(), elem2.get_noeud()))
    
    for elem in liste_noeud[len(liste_noeud)//10: (len(liste_noeud)//5 + len(liste_noeud)//10)]:
        nb = rd.randint(1,2)
        if nb == 1:
            v_liste_arete.append(Arete(elem.get_noeud(),liste_noeud[rd.randint(0, (len(liste_noeud)//10)-1)].get_noeud(),rd.randint(10,20)))
        else:
            i,j = 0,0
            while i == j:
                i = rd.randint(0, (len(liste_noeud)//10)-1)
                j = rd.randint(0, (len(liste_noeud)//10)-1)
            v_liste_arete.append(Arete(elem.get_noeud(),liste_noeud[i].get_noeud(),rd.randint(10,20)))
            v_liste_arete.append(Arete(elem.get_noeud(),liste_noeud[j].get_noeud(),rd.randint(10,20)))

def arete_t3():
    '''
    Fonction qui crée les aretes des noeuds de tier 3
    '''
    for elem in liste_noeud[nb_noeud//5+nb_noeud//10:]:
        i,j = 0,0
        while i == j:
            i = rd.randint((len(liste_noeud)//10), (len(liste_noeud)//10+len(liste_noeud)//5)-1)
            j = rd.randint((len(liste_noeud)//10), (len(liste_noeud)//10+len(liste_noeud)//5)-1)
        v_liste_arete.append(Arete(elem.get_noeud(),liste_noeud[i].get_noeud(),rd.randint(20,50)))
        v_liste_arete.append(Arete(elem.get_noeud(),liste_noeud[j].get_noeud(),rd.randint(20,50)))

def connexe():
    '''
    Fonction qui permet de vérifié la connexité du graphe à l'aide d'un parcours en largeur
    '''
    etat = ["Non vue" for _ in range(len(liste_noeud))]
    successeur = [0]
    i = 0
    while successeur != []:
        etat[i] = 'Vue'
        for elem in successeur:
            if elem == i:
                successeur.remove(i)
        for elem in liste_arete:
            if elem[0] == i: 
                if etat[elem[1]] == "Non vue":
                    successeur.append(elem[1]) 
            elif elem[1] == i:
                if etat[elem[0]] == "Non vue":
                    successeur.append(elem[0])
        if successeur == []:
            i = 0
        else:
            i = successeur[0]
    return etat == ["Vue" for _ in range(len(liste_noeud))]

def poids(aretes):
    '''
    Initialisation de la matrice qui contient le poids des aretes
    '''
    matrice=np.zeros((100,100))
    for elem in aretes:
        matrice[elem.get_noeud()[0],elem.get_noeud()[1]]=elem.get_poids()
        matrice[elem.get_noeud()[1],elem.get_noeud()[0]]=elem.get_poids()
    return matrice

def init_routage(poids):
    '''
    Fonction qui crée une table de routage
    mat_p: est la matrice d'adjacence qui contient la valeur du poids pour les aretes existantes et la valeur infini pour les aretes non existante.
    mat_c: est la matrice qui contient le chemin d'un noeud a un autre
    '''
    mat_p=np.zeros((100,100),)
    mat_chem=np.zeros((100,100))
    for i in range(100):
        for j in range(100):
            if i!=j:
                if poids[i,j]!=0:
                    mat_p[i,j]=poids[i,j]
                    mat_chem[i,j]=j
                else:
                    mat_p[i,j]=np.inf
                    mat_chem[i,j]=np.inf
            else:
                mat_chem[i,i]=i
                mat_p[i,i]=i      
    return mat_p,mat_chem

def routage(mat1,mat2):
    '''
    Fonction qui calcule la table de routage en fonction des matrice initialisé dans la fonction init_routage
    mat_val: est une copie de la matrice d'adjacence initialisé dans la fonction init_routage, pour garder l'original intacte
    mat_chemin: est une copie de la matrice du chemin des noeuds initialisé dans la fonction init_routage
    '''
    mat_val=mat1.copy()
    mat_chemin=mat2.copy()
    for i in range(len(mat_val)):
        for z in range(len(mat_val)):
            for y in range(len(mat_val)):
                if z!= y:
                    if mat_val[y,i]!=float('inf') and mat_val[i,z]!=float('inf') :       
                        if mat_val[y,z]>mat_val[y,i]+mat_val[i,z] or (mat_val[z][y]==np.inf):
                            # if (i,y) in liste_arete or (z,i) in liste_arete:
                                mat_val[y,z]=mat_val[y,i]+mat_val[i,z]
                                mat_chemin[y,z]=mat_chemin[y,i]
    return mat_val,mat_chemin

def chemin(noeud1,noeud2,mat):
    '''
    Fonction qui permet de retrouvé le chemin entre 2 noeuds à partir d'une matrice chemin
    '''
    liste=[]
    if noeud1!=noeud2:
        if mat[int(noeud1)][int(noeud2)]==noeud1:
            return [noeud1, noeud2]
        z=mat[int(noeud1)][int(noeud2)]
        liste.append(int(noeud1))
        print(f"mat de {noeud1,noeud2} = {z}")

        x=noeud1
        while z != x:
            x=z
            liste.append(int(z))
            z=mat[int(z)][int(noeud2)]
            print(f"mat de {z,noeud2} = {int(z)}")
        liste.append(noeud2)
        return liste
    return -1

def verif_posistion(liste):
    '''
    Fonction qui permet d'éviter la superposition d'aretes sur l'interface graphique
    '''
    for _ in range(3):
        for m in range (30,len(liste)):
            for n in range(30,len(liste)):
                if 0<abs(liste[m][0] - liste[n][0])<20 and 0< abs(liste[m][1]-liste[n][1]) <20:
                    moy_x, moy_y = (liste[m][0] + liste[n][0])//2, (liste[m][1] + liste[n][1])//2
                    liste[m] = (moy_x+15,moy_y+15)
                    liste[n] = (moy_x-15,moy_y-15)
                elif 0<abs(liste[m][0] - liste[n][0])<20 and abs(liste[m][1] - liste[n][1]) ==0:
                    moy_x = (liste[m][0] + liste[n][0])//2
                    liste[m] = (moy_x+15,liste[m][1])
                    liste[n] = (moy_x-15,liste[n][1])
                elif 0<abs(liste[m][1] - liste[n][1])<20 and abs(liste[m][0] - liste[n][0])==0:
                    moy_y = (liste[m][1] + liste[n][1])//2
                    liste[m] = (liste[m][0],moy_y+15)
                    liste[n] = (liste[n][0],moy_y-15)

if __name__ =='__main__':
    nb_noeud = 100
    liste_noeud = []
    nom = 0
    for _ in range(nb_noeud//10):
        liste_noeud.append(Noeud(nom, 1))
        nom += 1
    for _ in range(nb_noeud//5):
        liste_noeud.append(Noeud(nom, 2))
        nom += 1
    for _ in range(nb_noeud - (nb_noeud//5+nb_noeud//10)):
        liste_noeud.append(Noeud(nom, 3))
        nom += 1
    conexite = False
    while not conexite:
        v_liste_arete = []
        arete_t1()
        arete_t2()
        arete_t3()
        liste_arete = []
        for elem in v_liste_arete:
            liste_arete.append(elem.get_noeud())
        conexite = connexe()
    mat=init_routage(poids(v_liste_arete))
    mat_f=routage(mat[0],mat[1])
    schema=Schema(mat_f[1],liste_arete)
    schema.run()