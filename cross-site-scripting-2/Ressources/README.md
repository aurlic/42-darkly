When we check the main page, we see that clicking the NSA logo redirects us to http://localhost:8080/?page=media&src=nsa.

Upon inspection, we find that the image is being loaded through an `<object>` tag, as shown in the source code: `<object data="http://10.0.2.15/images/nsa_prism.jpg"></object>`.

We then try a URL with random text http://localhost:8080/?page=media&src=test and notice a 404 error, suggesting the application doesn't handle this source.

Let's try writing some script to see if we can exploit this breach.

`http://localhost:8080/?page=media&src=data:text/html;base64,PHNjcmlwdD5hbGVydCgpPC9zY3JpcHQ+`

By sending a base64-encoded HTML and JS payload in the src parameter, we find our next flag !

To remediate this vulnerability, the application should:
 - Sanitize all user inputs to prevent code injection
 - Enforce a strict Content Security Policy (CSP)
 - Validate and restrict allowed file types
 - Avoid using <object> for untrusted content