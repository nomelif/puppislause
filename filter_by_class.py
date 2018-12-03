# Quick script to filter words by class from the Kaino-dataset
# This discards suffixes and some classes of complex compounds
# GPL3 Copyright Théo Friberg 2018

from voikko import libvoikko
from bs4 import BeautifulSoup
import sys

# Read the Kaino dataset

f = open(sys.argv[1])
parsed = BeautifulSoup(f.read(), 'lxml')
f.close()


# Initialise voikko

v = libvoikko.Voikko('fi')

# Accumulate words into a seet

words = set()

for word in parsed.find_all('s'): # The s-tag in Kaino denotes a word

  s = word.string
  if s.lower() != s or "-" in s: # Discard suffixes and certain compounds
    continue

  analysis_ = v.analyze(s) # Analyse the word using Voikko; skip if Voikko gets confused
  if len(analysis_) == 0:
    continue
  analysis = analysis_[0]

  if v.analyze(s)[0]['CLASS'] == sys.argv[2]:
    words = words | set([analysis['BASEFORM'].lower()])

# Sort the output for predictability's sake

for word in sorted(list(words)):
  print(word)

