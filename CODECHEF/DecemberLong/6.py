T = int(input())
solSet = {}


def getClasses(set):


def getSubs(str, size, lgt):
    thresh = size
    rset = []
    strt = 0
    while thresh < lgt+1:
        rset.append(str[strt:thresh])
        thresh += 1
        strt += 1
    return rset


for _ in range(T):
    S = input()
    lgt = len(S)
    for i in range(1, lgt+1):
        set = getSubs(S, i, lgt)
        print(set)
