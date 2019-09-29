#!/bin/sh

rm -rv docs

cargo doc -p toitoi --target-dir docs

rm -rv docs/debug
mv -fv docs/doc/* docs

# Dummy index with redirect
cat > docs/index.html <<- EOM
<!-- Manual redirection to the crate index -->

<html>
    <head>
        <noscript><meta http-equiv="refresh" content="0; url=toitoi/index.html"></noscript>
    </head>
    <body onload="window.location = 'toitoi/index.html'">
        <a href="toitoi/index.html">Click me</a>
    </body>
</html>
EOM
