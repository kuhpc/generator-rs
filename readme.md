# Generator

At KUHPC we frequently need to create account. Most of the infomation is available in excel file.

First we need a excel file with Sheet1 having username and password.

Grab the binary and run it like this

```
$ ./generator --excel-path="data.xlsx" --accounting="four_core" --max-jobs="2" --max-cpu="10"
```

### Future plan

1. Use templating engine

### Why Rust?

Why not.
