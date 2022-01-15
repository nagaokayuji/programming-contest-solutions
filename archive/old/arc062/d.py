s = input()
ans = 0
ans -= s[:(len(s)+1)//2].count('p')
ans += s[(len(s)+1)//2:].count('g')
print(ans)
