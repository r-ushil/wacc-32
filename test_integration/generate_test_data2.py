import subprocess
import sys
from pathlib import Path
import os

def main():
    output = subprocess \
        .check_output(['refCompile', '-a', str(sys.argv[1])]) \
        .decode('utf-8') \
        .split('\n') \

    outputFile = Path(str(sys.argv[2]))
    outputPath = outputFile.parent

    if not os.path.exists(outputPath):
        os.makedirs(outputPath)

    f = open(outputFile, "w")
    lines = []

    found = False
    for line in output:

        # Only add the lines between the occurences of "===="
        if "====" in line:
            found = not found
            continue

        # Add the line if between equals signs, drop leading numbers and tab
        if found:
            lines.append(line.lstrip("0123456789")[1:])

    # Drop the last empty line
    lines = lines[:-1]

    # Write the collected lines
    for line in lines:
        f.write(line + "\n")             

if __name__ == "__main__":
    main()
