n, k = map(int, input().split())
a = list(map(int, input().split()))


base = k // n
kd = k % n


border = 10**20
ai = []
for i, x in enumerate(a):
    ai.append((x, i))
ai.sort()
border = ai[kd][0]
for x in a:
    # print(base if x < border else base+1)
    if x < border:
        print(base+1)
    else:
        print(base)
