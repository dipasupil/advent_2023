import re

def check_inbounds(array, row, column):
    return 0 <= row < len(array) and 0 <= column < len(array[0])
    
    
def check_for_adjacent_symbol(input_array, row, col):
    
    # need to check 9 surrounding squares and return True if any hold a symbol.
    for i in range(row - 1, row + 2):
        for j in range(col - 1, col + 2):
            if check_inbounds(input_array, i, j):
                if input_array[i][j] != "." and not input_array[i][j].isdigit():
                    return True
    
    return False


def check_for_adjacent_asterisk(input_array, asterisk_array, number, row, col):
    
    changed = False
    for i in range(row - 1, row + 2):
        for j in range(col - 1, col + 2):
            if check_inbounds(input_array, i, j):
                
                # if adjacent square has an asterisk, add the number to that coordinates' list in the asterisk array.
                if input_array[i][j] == "*":
                    asterisk_array[i][j].append(number)
                    changed = True
    
    return changed, asterisk_array


def get_number_to_column_idx_sets(row, numbers):
    
    # for each number found, find its starting and ending column idx and store.
    number_to_column_idxs_sets = {}
    skipped = 0
    for number in numbers:
        
        # find idx of start and end of number in row.
        # skipped necessary because .find() finds first occurrence and same number can show up more than once per row.
        index = row[skipped:].find(str(number))
        start = index + skipped 
        end = start + len(str(number))
        skipped = end + 1
        
        # numbers can show up more than once per row, so append each "occurrence" of columns separately.
        # ex: {
        #      123: [[3,4,5], [22,23,24]],
        #      42:  [[12, 13]]
        #      }
        
        if number in number_to_column_idxs_sets:
            number_to_column_idxs_sets[number].append([num for num in range(start, end)])
        else:
            number_to_column_idxs_sets[number] = [[num for num in range(start, end)]]
    
    return number_to_column_idxs_sets
  
              
def part2():
    with open('../../data/day_3_input.txt', 'r') as file:
        input_array = [line.strip() for line in file]
        asterisk_array = [[[] for _ in line] for line in input_array] # for spaces with asterisks, will fill arrays w/ adjacent numbers.
        
    for row_idx, row in enumerate(input_array):
        
        # find all numbers in each row of the input and record the column_idxs they appear in.
        number_to_column_idxs_sets = {}
        numbers = [int(num) for num in re.findall(r'\d+', row)]         
        number_to_column_idxs_sets = get_number_to_column_idx_sets(row, numbers)

        # for each number "occurrence" (every time it shows up in row), if number adjacent to "*", add that number to the "*"'s coordinates in asterisk_array.
        for number, column_idxs_sets in number_to_column_idxs_sets.items():
            for column_idx_set in column_idxs_sets:
                for col_idx in column_idx_set:
                    changed, asterisk_array = check_for_adjacent_asterisk(input_array, asterisk_array, number, row_idx, col_idx)
                    if changed:  
                        break # break out of column set if asterisk_array is altered (so we don't double add the number)
    
    # go through asterisk array and for all entries of length 2, add their products to total_sum
    total_sum = 0
    for row in asterisk_array:
        for array in row:
            if len(array) == 2:
                total_sum += array[0] * array[1]
        
    return total_sum
       

def part1():
    with open('../../data/day_3_input.txt', 'r') as file:
        input_array = [line.strip() for line in file]

    total_sum = 0
    

    for row_idx, row in enumerate(input_array):
        
        # find all numbers in each row of the input and record the column_idxs they appear in.
        number_to_column_idxs_sets = {}
        numbers = [int(num) for num in re.findall(r'\d+', row)]         
        number_to_column_idxs_sets = get_number_to_column_idx_sets(row, numbers)
        
        # for each number "occurrence" (every time it shows up in row), if number adjacent to symbol, add that number to total_sum.      
        for number, column_idxs_sets in number_to_column_idxs_sets.items():
            adjacent = False
            for column_idx_set in column_idxs_sets:
                for col_idx in column_idx_set:
                    adjacent = check_for_adjacent_symbol(input_array, row_idx, col_idx)
                    if adjacent:
                        total_sum += number
                        break # if number is adjacent, break from column set so we don't add number more than once
        
    return total_sum


if __name__=="__main__":
    print(part1())
    print(part2())