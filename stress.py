import os
import sys
import time
import pandas
import psutil

try:
    INPUT = sys.argv[1]
except:
    print("No file supplied")
    sys.exit(1)

def time_me(f):
    start = time.time()
    res = f()
    t = time.time() - start
    print("Time: {:.5f}s".format(t))
    return t, res

MB = 1024 * 1024

print("Loading CSV")
t, df = time_me(lambda: pandas.read_csv(INPUT, dtype={"categoricals": "category"}))

mem = int(df.memory_usage(index=True, deep=True).sum()/MB)
print(f"Dataframe memory usage: {mem}M")

process = psutil.Process(os.getpid())
procmem = int(process.memory_info().rss/MB)
print(f"Process memory usage: {procmem}M")

print("Summing ints")
time_me(lambda: df.ints.sum())

print("Summing floats")
time_me(lambda: df.floats.sum())

print("Summing floats_nan")
time_me(lambda: df.floats_nan.sum())

def lower(df):
    df["strings"] = df["strings"].str.lower()
    return df

print("Lowercasing strings")
time_me(lambda: lower(df))

print("Group by category")
time_me(lambda: df.groupby("categoricals").sum())

print("Joining to self on ints")
time_me(lambda: df.join(df.set_index("ints"), on="ints", rsuffix="l"))
