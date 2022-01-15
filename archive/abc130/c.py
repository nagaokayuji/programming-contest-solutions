W, H, x, y = map(int, input().split())
print(W*H/2)

if x*2 == W and y*2 == H:
    print(1)
else:
    print(0)
