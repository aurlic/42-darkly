When exploring the application, we check for a robots.txt file (standard file used by websites to communicate with web crawlers about which parts of the site should not be accessed or indexed).

We see that there are two disallowed files but let's focus on the first one `whatever`.
When going to `http://localhost:8080/whatever/` we find a downloadable htpasswd with a login:password.

Let's try decrypting the password with MD5:

`437394baff5aa33daa618be47b75cb49` -> `qwerty123@`

Now that we got a password let's go to `http://localhost:8080/admin/` and log in with the previously found informations.

And just like this we get our next flag !