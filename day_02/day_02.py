import numpy as np
import pandas as pd

pd.set_option('display.max_rows', 500)

base_scores = {
    'X': 1,
    'Y': 2,
    'Z': 3
}

# PART 1
df = pd.read_csv('d2_input.txt', names=['them', 'me'], delimiter=' ')
df['base_score'] = df['me'].map(base_scores)
df['them_ord'] = df['them'].apply(lambda x: ord(x))
df['me_ord'] = df['me'].apply(lambda x: ord(x))

# Get draws first
df['win_score'] = np.where(df['me_ord'] - df['them_ord'] == 23, 3, 0)

# Now wins
df['win_score'] = np.where(
    (df['me_ord'] - df['them_ord'] < 22) | (df['me_ord'] - df['them_ord'] == 24), 
    6, 
    df['win_score']
)

df['tot_score'] = df['base_score'] + df['win_score']
print(df['tot_score'].sum())