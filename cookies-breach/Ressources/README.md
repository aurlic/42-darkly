To find this new breach we first start by going to the storage of the app to check the cookies.

We notice that there's an already existing `I_am_admin` cookie with a value `68934a3e9455fa72420237eb05902327` that appears to be encrypted.

We try to decode it using MD5 as in the rest of the project, and it works, giving us the keywor `false`. We then encode the keyword `true` with MD5, change the value and this gives us our next flag.

To remediate this vulnerability, the application should:
 - Implement secure session management by using strong, server-side generated session tokens
 - Avoid storing sensitive information directly in cookies
 - Properly validate and sanitize all cookie values on the server side before processing them
 - Use signed or encrypted cookies to prevent tampering