N = int(input())
P = float(input())
if N >= 200:
    print(1)
    exit()

print(1 - pow(1-P, N))
