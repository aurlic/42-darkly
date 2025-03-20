When trying to login, we find a `I forgot my password` button that leads us to http://localhost:8080/index.php?page=recover.

Looking around the HTML, some element appear to be hidden : 

`<input type="hidden" name="mail" value="webmaster@borntosec.com" maxlength="15">`

So we un-hide them, change the maxlength and set the password recovery adress as our own and we find our next flag !

// here screenshot