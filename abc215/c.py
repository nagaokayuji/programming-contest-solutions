import itertools
s, k = input().split()
s = list(s)
k = int(k)

s.sort()

perm = list(set(itertools.permutations(s)))
perm.sort()

print("".join(perm[k-1]))
