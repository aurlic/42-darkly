When trying to login, we find a `I forgot my password` button that leads us to http://localhost:8080/index.php?page=recover.

Looking around the HTML, some element appear to be hidden : 

`<input type="hidden" name="mail" value="webmaster@borntosec.com" maxlength="15">`

So we un-hide them, change the maxlength and set the password recovery adress as our own and we find our next flag !

To remediate this vulnerability, the application should:
 - Implement proper server-side verification of user identity and access rights
 - Never rely on hidden form fields for access control or security purposes
 - Use session-based authentication to verify that users can only access their own accounts
 - Apply the principle of least privilege for all user accounts and functions
 - Implement robust authorization checks on every request that accesses sensitive data