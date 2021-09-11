facts = [1] * 10
for i in range(1, 10):
    facts[i] = facts[i-1] * (i+1)


facts.reverse()
ans = 0
p = int(input())
for x in facts:
    if p >= x:
        ans += p//x
        p %= x
print(ans)
