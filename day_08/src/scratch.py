a = [
        [1, 2, 3, 4, 98], 
        [5, 6, 7, 8, 97], 
        [9, 10, 11, 12, 96]
    ]

def backwards(row, column, num_cols=5):
    a = [
        [1, 2, 3, 4, 98], 
        [5, 6, 7, 8, 97], 
        [9, 10, 11, 12, 96]
    ]
    a = [item for sublist in a for item in sublist]
    a = a[::-1]
    new_col = num_cols - column - 1
    print(a[new_col - num_cols])
    return a
    
print(backwards(2, 1))
