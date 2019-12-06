from math import log


def cantor_pair():
    pass


def cantor_unpair():
    pass


# https://hbfs.wordpress.com/2011/09/27/pairing-functions/
def goedel_pair(x, y):
    return (2 ** x) * (3 ** y)


def goedel_unpair(z):
    x, y = 0, 0

    lo_y = 0
    hi_y = 1

    while z % 3 ** hi_y == 0:
        lo_y = hi_y
        hi_y *= 2

    # ok, we know it's somewhere lo_y<=y<hi_y
    while lo_y < hi_y:
        test_y = (hi_y + lo_y + 1) / 2
        if z % 3 ** test_y:
            hi_y = test_y - 1
        else:
            lo_y = test_y

    z /= 3 ** lo_y
    x = int(log(z + 0.01, 2))  # numerical stability issue here
    return (x, lo_y)
