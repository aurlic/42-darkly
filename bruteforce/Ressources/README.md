Looking around the page we stumble upon a login page http://localhost:8080/index.php?page=signin that we are going to try to bruteforce.

We make a quick script in Rust to try a list of the 100 most common passwords and execute it like so:

`cargo run -- http://localhost:8080/index.php admin passwords.txt`

And find that the password is `shadow`.

We log in using `admin` `shadow` and get our next flag.

To remediate this vulnerability, the application should:
	- Implement a stronger password policy