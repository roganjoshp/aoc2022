import numpy as np
import pandas as pd

pd.set_option('display.max_rows', 500)

# PART 1
df = pd.read_csv('d1_input.txt', skip_blank_lines=False, names=['calories'])
df['elf'] = np.where(df['calories'].isna(), 1, 0).cumsum() + 1
df = df.dropna()

df = df.groupby(['elf'])['calories'].sum().reset_index()
print(df.tail())
print(df['calories'].max())

# PART 2
df = df.sort_values(by=['calories'], ascending=False)
top_3 = df.iloc[:3, :]['calories'].sum()
print(top_3)
