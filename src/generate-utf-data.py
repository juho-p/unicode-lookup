import unicodedata

AMOUNT = 0x110000

def gather():
    for value in range(AMOUNT):
        c = chr(value)
        try:
            name = unicodedata.name(c)
            yield (name, c)
        except ValueError:
            pass

for name, c in gather():
    print(c, name, sep=' ')
