import os 

dna = open('rosalind_revc.txt', 'r').read()[::-1]

reversedDna = ''
for i in dna:
    if i == 'T': reversedDna += 'A'
    if i == 'A': reversedDna += 'T'
    if i == 'C': reversedDna += 'G'
    if i == 'G': reversedDna += 'C'

print(reversedDna)
os.system("echo '%s' | pbcopy" % reversedDna)