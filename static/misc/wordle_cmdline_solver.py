# wordle_cmdline_solver.py
# 
# Command line solver for WORDLE
#
# Version  : 1.0
# Author   : Jasper Ty
# Date     : 2022/01/10
# Language : python 3.8.2

# DESCRIPTION 
#       This program solves a single game of the popular word game WORDLE.
#
#       WORDLE is a game similar to Mastermind. The player is given six attempts to guess a five-letter word that is unknown to them, and wins when one of the guesses is successful.
#       After every guess, WORDLE gives the player information by coloring each letter of their current guess according to three rules:
#               BLACK - This letter is not present in the answer
#               YELLOW - This letter is present in the answer, but in the wrong position
#               GREEN - This letter is present in the answer, and is in the correct position.
#
#       This program solves WORDLE by utilizing a min-max strategy, wherein the guesses taken ensure that the minimum words eliminated is maximized.
#       Given the state of any WORDLE game, one may determine the set of all five-letter words that may be the possible answer. 
#       In the beginning, when one has taken zero guesses, this set is the set of ALL five-letter words.
#       Given a guess word, one can calculate the minimum number of possible said guess eliminates, regardless of the guesses' outcome.
#       This is done by looking at all possible outcomes of the guess (one outcome is, say, BYGGB, or GGBBY), and finding the outcome with the minimum eliminated possibilities.
#       Knowing this for every possible guess word, one can choose the guess word which maximizes the minimum eliminated.
#       Proceeding this way, one finds, given enough guesses, the answer word.
#
#       No initial guess is given, and must be provided by the user.

# DEFINITIONS
#       word    : a five letter string of lowercase alphabetic characters
#       fiveLWs : the set of all five-letter words (according to the 5757 word file sgb-words.txt)
#                 the file can be found at https://www-cs-faculty.stanford.edu/~knuth/sgb-words.txt
#       hidden  : a word that is tested against by input words. hidden represents the word that one must find in a game of WORDLE
#
#       ev      : a 5-long array of the values {BLACK, YELLOW, GREEN}, representing the result of testing an input word against a hidden
#       guess   : an ev equipped with an associated input word, which specifies complete info about both letters and colors. a guess represents the result of one complete guess in WORDLE
#
#    consistent : a guess is consistent with a given hidden when the guess arises from testing the guess word against the hidden



W_LEN = 5
GREEN = 2
YELLOW = 1
BLACK = 0

# Replace with appropriate path
f = open("E:\Wordlestats\sgb-words.txt", "r")
fiveLWs = [x[:-1] for x in f]



#############
# Functions #
#############

##########
# evaluate
#   Returns the ev resulting from entering an input word against the hidden
def evaluate(word, hidden):
    ev_w = [0] * W_LEN
    ev_h = [0] * W_LEN
    for i in range(W_LEN):
        if word[i] == hidden[i]:
            ev_w[i] = GREEN
            ev_h[i] = GREEN
    for i in range(W_LEN):
        for j in range(W_LEN):
            if ev_w[i] == BLACK and ev_h[j] == BLACK:
                if word[i] == hidden[j]:
                    ev_w[i] = YELLOW
                    ev_h[j] = YELLOW
    return ev_w

# prints an ev in a readable format 
def print_ev(ev):
    color = ['B', 'Y', 'G']
    temp = []
    for i in range(5):
        temp.append(color[ev[i]])
    print(''.join(temp))

#######
# guess
#   Returns a guess resulting from entering an input word against the hidden 
def guess(word, hidden):
    return (word, evaluate(word, hidden))

# prints a guess in a readable format
def print_guess(guess):
    print(guess[0])
    print_ev(guess[1])

############
# consistent
#   Determines whether a guess is consistent with a given hidden
def consistent(guess, test_hidden):
    test_ev = evaluate(guess[0], test_hidden)
    return guess[1] == test_ev

###################
# consistent_subset
#   Returns the subset of w_set of all hiddens that are consistent with a guess
def consistent_subset(w_set, guess):
    w_subset = []
    for word in w_set:
        if consistent(guess, word):
            w_subset.append(word)
    return w_subset

################
# min_eliminated
#   Calculates the minimum amount of hiddens in a set that a given input word eliminates (the minimum number of hiddens inconsistent with the input word)
def min_eliminated(w_set, word):
    min_elim = len(w_set)
    for test_hidden in w_set:
        g = guess(word, test_hidden)
        w_subset = consistent_subset(w_set, g)
        elim = len(w_set) - len(w_subset)
        min_elim = elim if elim < min_elim else min_elim
    return min_elim

##############
# min_max_word
#   Calculates the input word which maximizes the minimum amount of hiddens in w_set eliminated
def min_max_word(w_set):
    max_min_elim = 0
    min_max_word = []
    for word in fiveLWs:
        min_elim = min_eliminated(w_set, word)
        if min_elim > max_min_elim:
            max_min_elim = min_elim
            min_max_word = word
    return min_max_word




############################
# Solver command-line code #
############################

print("WORDL SOLVER")
def read_ev(ev_str):
    color = ['B', 'Y', 'G']
    ev = []
    for i in range(W_LEN):
        ev.append(color.index(ev_str[i]))
    return ev

# initial guess
word_i = input("Enter initial guess: ")
ev_i = read_ev(input("Enter colors (e.g, BBYGY): "))
g_i = (word_i, ev_i)
S = consistent_subset(fiveLWs, g_i)

# Runs the min-max algorithm until there is only one possible answer
while len(S) > 1:
    next_word = min_max_word(S)
    print("Try " + next_word + ".")
    ev = read_ev(input("Enter colors: "))
    g = (next_word, ev)
    S = consistent_subset(S, g)

print("Final answer is: " + S[0])