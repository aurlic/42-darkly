Navigating through the application we land on a survey page http://localhost:8080/index.php?page=survey# and start looking through the HTML.

We notice that we can modify the value sent to the BackEnd for each of the option, so we do just that and modify it to a huge number:

`<option value="10000000">10</option>`

We then proceed to click our newly modified option in the drop down menu, and get our flag !

To remediate this vulnerability, the application should:
 - Not let the user modify the values sent to the BackEnd