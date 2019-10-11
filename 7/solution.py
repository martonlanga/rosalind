count = 0
s, t = [line.rstrip("\n") for line in open("rosalind_hamm.txt")]
for i in range(len(s)):
    if s[i] != t[i]:
        count += 1

print(count)
