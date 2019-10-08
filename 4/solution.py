months, k = open('rosalind_fib.txt').read().split(' ')
months, k = [int(months), int(k)]

currMonth = 1
young = 1
old = 0
while currMonth < months:
    temp = young
    young = old * k
    old += temp
    currMonth += 1

print(old + young)