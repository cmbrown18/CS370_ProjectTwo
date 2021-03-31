#! /bin/bash

# Testing all the cases of redirect


# Case one
# Redirect std in

cat < file1.txt

# Case Two 
# Redirect stdout without appending

echo "hello all my people" >> file2.txt

# Case Three
# Redirect stdout with appending

echo "hello everyone else" > file2.txt

# Case Four 
# Redrect stdout and stderr

echo "welp that didnt work" &> file3.txt


# Case 5 
# Redirect stderr

printf "oops we made a fucky wucky" 2> file4.txt



# Case 6
# Piping

echo "Hello There" | grep "Hello"

