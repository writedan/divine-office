# Ensures that the text offered by each tone accords with the text of the psalm

import os
import difflib
import re

def lexicographic_analysis(directory):
    text_file = os.path.join(directory, 'text.lit')
    lit_files = [f for f in os.listdir(directory) if f.endswith('.lit') and f != 'text.lit']

    if not os.path.isfile(text_file) or len(lit_files) == 0:
        print("Error: Directory must contain 'text.lit' file and other '.lit' files.")
        return

    # Read lines from text.lit
    with open(text_file, 'r') as text_fp:
        text_lines = text_fp.readlines()

    # Define regex pattern to match characters to be removed
    char_filter_regex = re.compile(r'[`°~^\n]')

    # Iterate over other .lit files
    for lit_file in lit_files:
        lit_file_path = os.path.join(directory, lit_file)
        with open(lit_file_path, 'r') as lit_fp:
            lit_lines = lit_fp.readlines()

        # Apply character filtering and compare lines of text.lit with lines of current .lit file
        differ = difflib.Differ()
        text_lines_filtered = [char_filter_regex.sub('', line) for line in text_lines]
        lit_lines_filtered = [char_filter_regex.sub('', line) for line in lit_lines]

        lit_lines_filtered = lit_lines_filtered[:-2]
        diff = list(differ.compare(text_lines_filtered, lit_lines_filtered))

        if not any(line.startswith('+') or line.startswith('-') for line in diff):
            return

        print(f"Differences with {lit_file}:")

        # Print differences with line numbers
        text_line_number = 1
        lit_line_number = 1
        for line in diff:
            if line.startswith('+'):
                print(f"Line {text_line_number}: {line.strip()}")
                text_line_number += 1
            elif line.startswith('-'):
                print(f"Line {lit_line_number}: {line.strip()}")
                lit_line_number += 1
            elif line.startswith('?'):
                print(f"Line {lit_line_number}: {line.strip()}")
            else:
                text_line_number += 1
                lit_line_number += 1
        print()

# Example usage:
directory = 'your_directory_path_here'
lexicographic_analysis(directory)
