import random
import numpy as np
from time import *

N = 4096

A = np.array([[random.random() for _ in range(N)] for _ in range(N)])
B = np.array([[random.random() for _ in range(N)] for _ in range(N)])

start = time()
C = np.matmul(A, B)
end = time()

print(f"Time taken: {end - start}")
