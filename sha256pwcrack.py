from pwn import *
import sys

#check if there is input
if len(sys.argv) != 3:
	print("Invalid arguments!")
	print(">> {} <sha256sum> <list_to_crack>".format(sys.argv[0]))
	exit()

#variables needed
wanted_hash = sys.argv[1]
password_file = sys.argv[2]
attempts = 0

#log lib.progress is a handy function to return a log object to track for progress, and return result
with log.progress("Attempting to crack: {}!\n".format(wanted_hash)) as p:
	with open(password_file, "r", encoding='latin-1') as password_list:
		for password in password_list:
			password = password.strip("\n").encode('latin-1')
			password_hash = sha256sumhex(password)
			p.status("[{}] {} == {}".format(attempts, password.decode('latin-1'), password_hash))
			if password_hash == wanted_hash:
				p.success("Password hash found after {} attempts! '{}' hashes to {}!".format(attempts, password.decode('latin-1'), password_hash))
				exit()
			attempts += 1
		p.failure("Password hash not found!")


