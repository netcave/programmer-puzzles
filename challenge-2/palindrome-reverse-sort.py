import sys

def main(args):
    path = args[1]
    file = open(path, 'r')
    lines = file.read().split('\n')
    strings = map(format_string, lines)
    for string in strings:
        print(string)

def format_string(string):
    palindrome = is_palindrome(string)
    status = get_status(palindrome)
    stripped_string = strip_spaces(string)
    sorted_string = sort_string(stripped_string)
    reversed_sorted = reverse(sorted_string)
    return "{0} | {1}".format(status, reversed_sorted)

def get_status(boolean):
    responses = {True: "YES", False: "NO"}
    return responses[boolean]

def strip_spaces(string):
    return string.replace(' ', '')

def is_palindrome(string):
    lower_string = string.lower()
    reversed_string = reverse(lower_string)
    return lower_string == lower_string[::-1] 

def reverse(string):
    return string[::-1]

def sort_string(string):
    return "".join(sorted(string))

if __name__ == '__main__':
    main(sys.argv)
