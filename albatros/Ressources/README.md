When visiting http://localhost:8080/index.php?page=b7e44c7a40c5f80139f0a50f3650fb2bd8d00b0d24667c4c2ca32c88e13b758f we can see some weird comments in the HTML : 


`You must come from : https://www.nsa.gov/.`

`Let's use this browser : "ft_bornToSec". It will help you a lot.`

Let's try reaching this page as "https://www.nsa.gov/" using curl, then do the same without a referer and check for a diff:

```
$> curl "http://localhost:8080/index.php?page=b7e44c7a40c5f80139f0a50f3650fb2bd8d00b0d24667c4c2ca32c88e13b758f" > base_access
$> curl --header "Referer: https://www.nsa.gov/" "http://localhost:8080/index.php?page=b7e44c7a40c5f80139f0a50f3650fb2bd8d00b0d24667c4c2ca32c88e13b758f" > nsa_access
$> diff base_access nsa_access
37c37
< <audio id="best_music_ever" src="audio/music.mp3"preload="true" loop="loop" autoplay="autoplay">
---
> FIRST STEP DONE<audio id="best_music_ever" src="audio/music.mp3"preload="true" loop="loop" autoplay="autoplay">
```

We can see that a `FIRST STEP DONE` text appears, let's keep going and check if we can do something with the other information previously found : `Let's use this browser : "ft_bornToSec".`

Let's try reaching again with a different user-agent : 

```
$> curl --header "Referer: https://www.nsa.gov/" -A "ft_bornToSec" "http://localhost:8080/index.php?page=b7e44c7a40c5f80139f0a50f3650fb2bd8d00b0d24667c4c2ca32c88e13b758f" > nsa_access2
```

And now, running a diff between our base html and the one we just got, we manage to find our first flag:

```
$> diff base_access nsa_access2
37c37
< <audio id="best_music_ever" src="audio/music.mp3"preload="true" loop="loop" autoplay="autoplay">
---
> <center><h2 style="margin-top:50px;"> The flag is : f2a29020ef3132e01dd61df97fd33ec8d7fcd1388cc9601e7db691d17d4d6188</h2><br/><img src="images/win.png" alt="" width=200px height=200px></center> <audio id="best_music_ever" src="audio/music.mp3"preload="true" loop="loop" autoplay="autoplay">

```

If we wanted to fix this breach, we would probably just remove the comments indicating how to exploit it.