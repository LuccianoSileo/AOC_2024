import sys
import re

if len(sys.argv) != 2:
  sys.exit(f"Uso: python3 {sys.argv[0]} puzzle.txt")
file = sys.argv[1]

# Lectura de fichero
with open(file, 'r') as f:
  lines = f.readlines()

# Parte 1
def find_match(regex, lines):
  match = []
  for l in lines:
    match += re.findall(regex, l)
  return match

def mul(mul):
  d = re.findall(r'\d{1,3}', mul)
  return int(d[0]) * int(d[1]) 

multiplicacion = find_match(r'mul\(\d{1,3},\d{1,3}\)', lines)
r1 = 0
for m in multiplicacion:
  r1 += mul(m)
print(f"Parte 1: {r1}")

# Parte 2
new_mult = find_match(r'don\'t\(\)|do\(\)|mul\(\d{1,3},\d{1,3}\)', lines)
r2 = 0
operate = True
for m2 in new_mult:
  if m2 == 'don\'t()': operate = False
  elif m2 == 'do()': operate = True
  else:
    if operate == True: r2 += mul(m2)
print(f"Parte 2: {r2}")