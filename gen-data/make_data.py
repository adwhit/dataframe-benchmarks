import os
import random
import string
import math
import pandas

try:
    os.mkdir("data")
except FileExistsError:
    pass

LETTER_CHOICES = string.ascii_letters + string.digits + " "
CATEGORIES = "RED BLUE YELLOW GREEN PINK PURPLE BLACK WHITE GREY".split()

def rand_string(l):
    return ''.join(random.choice(LETTER_CHOICES) for _ in range(l))

def generate_data(nrow):
    print("start")
    ints = [random.randint(-10000, 10000) for _ in range(nrow)]
    print("gen ints")
    floats = [random.random() for _ in range(nrow)]
    print("gen floats")
    floats_nan = [random.random() if random.random() > 0.8 else math.nan for _ in range(nrow)]
    print("gen floats_nan")
    word = [rand_string(20) for _ in range(nrow)]
    print("gen strings")
    categorical = [random.choice(CATEGORIES) for _ in range(nrow)]
    print("gen categorical")
    data = {
        "ints": ints,
        "floats": floats,
        "floats_nan": floats_nan,
        "words": word,
        "categorical": categorical
    }
    df = pandas.DataFrame(data=data)
    return df

def dump(df):
    l = len(df.values)
    path = "data/data_{}row.csv".format(l)
    df.to_csv(path)

if __name__ == "__main__":
    nrows = 20_000
    frame = generate_data(nrows)
    dump(frame)



