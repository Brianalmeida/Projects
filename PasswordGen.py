#!/usr/bin/env python3

import random 
import string
import colorama
import sys

from termcolor import colored
colors = ['red', 'yellow', 'green', 'cyan', 'blue', 'magenta', 'grey', 'white']

for color in colors:

    print(colored("********", color), end="")
    
print()


text = colored("*****/    *****   *****       ***      \*****\n         *   *** *   *       ***           ", 'cyan')

print(text)

text1 = colored("***/    *   *****   *      *****        \***\n       *   *** *   *      *   *                          ", 'red')

text10 = colored("*/    *****   *****      *****           \*", 'green')

text20 = colored("                                                  Created by @BAlms", 'red')



print(text1)
print(text10)
print(text20)

text2 = colored("****** Lets Start Creating A New Password For You Today! *******", 'yellow')

print(text2)


#This is where the user will input the length of the password that they want to use

length = int(input("\nEnter The Length Of The Password You Want To Use: "))

#We're going to define our data here

lower = string.ascii_lowercase
upper = string.ascii_uppercase
num = string.digits
symbols = string.punctuation

#This is where we'll combine the data

all = lower + upper + num + symbols + num + lower + upper + num + symbols + lower

#We will need to use random here

temp = random.sample(all,length)

#This is where we'll create the password

password = "".join(temp)

#Now we can print the password

print(password)

text3 = colored("\n**** Thanks For Using This Program, I'm Always Happy To Help ***\n", 'yellow')

print(text3)

for color in colors:

    print(colored("********", color), end="")
    
print()
