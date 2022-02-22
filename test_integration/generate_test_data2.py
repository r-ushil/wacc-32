import subprocess

def main():
    output = subprocess \
        .check_output(['refCompile', '-a', \
                       'wacc_examples/valid/pairs/checkRefPair.wacc']) \
        .decode('utf-8') \
        .split('\n') \

    f = open("test.s", "w")

    found = False
    for line in output:

        # Only print the lines between the occurences of "===="
        if "====" in line:
            found = not found
            continue

        if found:
            print(line)
            line = line.lstrip("0123456789") # Drop leading digits
            line = line[1:]                  # Drop first leading tab
            f.write(line + "\n")             # Write the line


if __name__ == "__main__":
    main()
