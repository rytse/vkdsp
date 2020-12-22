import numpy as np
import array 
import struct 
from scipy.fftpack import fft

a = []
with open("tmp.bin", 'rb') as f:
	while True:
		try:
			(num,) = struct.unpack('f', f.read(4))
			a.append(num)
		except:
			break




for val in np.reshape(a,(len(a)//2,2)).tolist():
	i,j = val
	print("{0}+{1}".format(i,j))


a = np.fft.fft(np.exp(2j * np.pi * np.arange(len(a))))
for row in a:
	print("{:.6f}+{:.6f}j".format(row.real, row.imag))




