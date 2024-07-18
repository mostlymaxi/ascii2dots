import sys

def main(char_length=30):
    test_str = '''
    |DD|____T_    
    |_ |_____|<
      @-@-@-oo\\
    '''
    test_str = '''
   .----.
   |C>_ |
 __|____|__
|  ______--|
`-/.::::.\-'a
 `--------'
    '''

    special_boy = '⠄'
    padded_test_str = ' ' * (char_length) + '\n'
    for line in test_str.split('\n'):
        padded_test_str += line
        padded_test_str += ' ' * (char_length - len(line))
        padded_test_str += '\n'


    char_to_braille = {
        ' ': '⡀',
        '_': '⣀',
        '-': '⠤',
        '|': '⢸',
        '@': '⣾',
        '<': '⢎',
        '>': '⡱',
        '\\': '⢣',
        '=': '⠭',
        '+': '⡦',
        'l': '⢸',
        ')': '⡱',
        '(': '⢎',
        '*': '⡦',
        '^': '⠋',
        '#': '⡶',
        '[': '⣏',
        ']': '⣹',
        'D': '⣱',
        'E': '⣯',
        'o': '⣦',
    }

    for char, braille in char_to_braille.items():
        padded_test_str = padded_test_str.replace(char, braille)
    

    print(padded_test_str)

    # every character has a braille replacement?
    # D => D made out of dots?

    




if __name__ == "__main__":
    main()
