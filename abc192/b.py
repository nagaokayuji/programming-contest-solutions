S = input()
for i, c in enumerate(S):
    ev = i % 2 == 0
    up = c.isupper()
    if ev ^ (not up):
        print("No")
        exit()
print("Yes")
