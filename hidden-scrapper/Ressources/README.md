When following the same process as the whatever breach, we find a 2nd file mentionned as disallowed.

We get to the adress `http://localhost:8080/.hidden/` and find many files with unreadable name, with the same inside each of those files and multiple times.

Let's run a scrapper through these files to see if we can find anything useful:

`cargo run`

Our scrapper gives us the link of the readme containing the flag: `[+] Flag found in: http://localhost:8080/.hidden/whtccjokayshttvxycsvykxcfm/igeemtxnvexvxezqwntmzjltkt/lmpanswobhwcozdqixbowvbrhw/README`

To remediate this vulnerability, the application should:
 - Avoid storing sensitive information in hidden directories that can be discovered through simple enumeration
 - Implement proper authentication and authorization controls for all resources
 - Do not rely on "security through obscurity" by hiding files in obscure locations