import random

def generate_words(grammar, start_symbol, max_length):
    def expand(sequence):
        if len(sequence) > max_length:
            return set()
        if all(symbol.islower() or symbol == "E" for symbol in sequence):
            return {"".join(symbol for symbol in sequence if symbol != "E")}

        words = set()
        for i, symbol in enumerate(sequence):
            if symbol.isupper() and symbol[-1].isdigit() and symbol in grammar:
                for production in grammar[symbol]:
                    new_sequence = sequence[:i] + production + sequence[i+1:]
                    words.update(expand(new_sequence))
                break
        return list(words)

    return expand([start_symbol])

def lire(file):
    dico={}
    with open(file,'r') as f:
        lignes=f.readlines()
        for ligne in lignes:
            ligne = ligne.replace(' ','').replace('\n','').split("->")
            if ligne[0] not in dico.keys():
                dico[ligne[0]] = []
            ligne[1] = ligne[1].split('|')
            for elem in ligne[1]:
                dico[ligne[0]].append(elem)     
        for NT, prod in dico.items():
            for i in range(len(prod)):
                dico[NT][i] = separe_elem(prod[i])
    return dico

def separe_elem(mot):
    if len(mot) == 1:
        return [mot]
    if len(mot) == 2 and mot[0].isupper():
        return [mot]
    else:
        if mot[0].islower():
            return [mot[:1]] + separe_elem(mot[1:])
        else:
            return [mot[:2]] + separe_elem(mot[2:])


grammar = lire("grammaire.chomsky")
start_symbol = next(iter(grammar))
max_length = 4
words = generate_words(grammar, start_symbol, max_length)
words.sort()
for elem in words:
    print(elem)