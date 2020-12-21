from array import array

x = array('d', range(0, 1024))
with open("../tmp.bin", "wb") as fi:
    x.tofile(fi)
