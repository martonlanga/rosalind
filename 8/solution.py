k, m, n = [int(x) for x in open("rosalind_hamm.txt").read().split()]
sum = k + m + n
total = sum * (sum - 1) / 2

abab = m + (m - 1) / 2 * 0.25
abbb = (m * n) * 0.5
bbbb = n * (n - 1) * 0.5

print(1 - (abab + abbb + bbbb) / total)
