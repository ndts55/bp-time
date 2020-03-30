# The format
The format itself is pretty simple. Any time entry just has to include `<time-format> | <category>`.
Anything before or after that part is ignored.
If a line does not match that pattern it is ignored as well.
This tool only works with `md` files.

## Time formats
`\d+h \d+m` ex. `10h 43m`

`\d+h` ex. `4h`

`\d+m` ex. `56m`

## Categories
This is the current list of categories
- c
- b
- m

## Example Entries
Your entry could look like this
```
| 2019-11-08 | 13.40 | 15.20 | 1h 30m | b did something |
```

or 

```
| 2019-11-08 | 13.40 | 17.40 | 5h | c did another thing |
```

or

```
| 2019-11-08 | 17.40 | 18.20 | 40m | m did one more thing |
```
