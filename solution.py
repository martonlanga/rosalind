dna = open('rosalind_dna.txt', 'r').read()
index = 0
a = c = g = t = 0
while index < len(dna) - 1:
    cur = dna[index]
    if cur == 'A': a += 1
    elif cur == 'C': c += 1
    elif cur == 'G': g += 1
    else: t += 1
    index += 1

print(a, c, g, t)