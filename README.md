This rust project with replace all filenames in a directory (`dirToRun`) if their path contains a specified string (`strToMatch`).

Basic Usuage

```
ls -R  dirToRun/
  blah.txt   strToMatch

  dirToRun//strToMatch:
  other.txt
cargo run -- strToMatch strToReplace dirToRun
ls -R  dirToRun/
  blah.txt   strToMatch

  dirToRun//strToMatch:
  strToReplace.txt
```

More functionality to come!
