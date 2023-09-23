# Parsing

Working on https://www.udemy.com/course/essentials-of-parsing/

Currently creating an LALR(1) parser generator. It builds the parse table from a given 
LALR-parsable grammar and provides algorithm to parse a given string.


Demos shift reduce conflict

```
npx syntax-cli -g demo.bnf -m LR0 -t
```

