s = list(map(int, list(input())))
f1 = True
f2 = True
for i in range(3):
    if s[i] != s[i+1]:
        f1 = False
    if (s[i]+1) % 10 != s[i+1] % 10:
        f2 = False
if f1 or f2:
    print("Weak")
else:
    print("Strong")
