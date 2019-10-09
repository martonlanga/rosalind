months, k = open('rosalind_fibd.txt').read().split(' ')
months, k = [int(months), int(k)]

rabbits = [0] * k
rabbits[0] = 1
currMonth = 1
while currMonth < months:
    new = sum(rabbits[1:])
    temp = rabbits.copy()
    for i in range(k - 1):
        rabbits[i + 1] = temp[i]
    rabbits[0] = new
    currMonth += 1

print(sum(rabbits))