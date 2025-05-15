#!/bin/bash

# Define target and wordlist location
TARGET="target.ine.local"
WORDLIST="/root/Desktop/wordlists/shares.txt"

# Check if the wordlist file exists; if not, exit with error message
if [ ! -f "$WORDLIST" ]; then 
  echo "Wordlist not found: $WORDLIST"
  exit 1
fi

# Loop through each share in the wordlist
# read line by line from the stdin and store the line in the variable SHARE, read is a build-in bash funciton that reads from stdin, and this case the stdin stores $WORDLIST from the last line
while read -r SHARE; do 
  echo "Testing share: $SHARE"
  smbclient //$TARGET/$SHARE -N -c "ls" &> /dev/null # run smbclient with anonymous login and run ls (-c ls) right after login, direct all output to /dev/null

# $? is a special shell variable that holds the exit status of the last command executed, 0 means success, non-zero value means an error or failure.
# If the command succeeds (e.g., anonymous login worked, ls ran without errors), then $? will be 0.
# If the command fails (e.g., anonymous login denied, share not found, network error), then $? will be a non-zero number (like 1, 2, etc.) indicating failure.

  if [ $? eq 0 ]; then # $? is a special shell variable that holds the exit status of the last command executed
    echo "[+] Anonymous access allowed for: $SHARE" # Write out the valid share
  else
    echo "[+] Access denied for: $SHARE"
  fi
done < "$WORDLIST" # directs $WORDLIST (in this case the location of the txt) into stdin
 
