At the bottom of the page we spot 3 icons linking to 42's different social medias.

Looking through the code we try and modify the href of one of those:

`<a href="index.php?page=redirect&amp;site=hehehe" class="icon fa-instagram"></a>`

And find our next flag !

To remediate this vulnerability, the application should:
 - Verify the links it goes to
 - Not let the user change them