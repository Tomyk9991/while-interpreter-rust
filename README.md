## Grammar rules
Where S is the starting non terminal, regular expressions are terminals

|Non terminals| Non terminals + Terminals                                |
|-------------|----------------------------------------------------------|
| S           | (INNERSCOPE* or METHOD*)+                                |
| VARIABLE    | NAME = ASSIGNMENT;                                       |
| INCREMENT   | NAME += ASSIGNMENT;                                      |
| DECREMENT   | NAME -= ASSIGNMENT;                                      |
| NAME        | [a-zA-Z][a-zA-Z0-9]*                                     |
| DIGIT       | [0-9]+                                                   |
| ASSIGNMENT  | NAME or DIGIT or METHOD-CALL                             |
| WHILE       | while NAME != 0: INNERSCOPE#                             |
| METHOD      | METHOD-HEAD INNERSCOPE return (ASSIGNTMENT or ε);        |
| METHOD-HEAD | TYPE NAME(ASSIGNTMENT or (ASSIGNMENT,)+ ASSIGNMENT or ε):|
| METHOD-CALL | NAME(ASSIGNTMENT or (ASSIGNMENT,)+ ASSIGNMENT or ε);     |
| INNERSCOPE  | (VARIABLE* or METHOD-CALL* or WHILE*)+                   |

---
## Example tokenizer:
This piece of code returns this "program stack" which is a tree of stackables
```py
x = 5;

num add(x, y):
    while a != 0:
        while b != 0:
            c = 5;
        #
        d = 5;
    #
    return z;
```

```
Program:
├── Methods:
│  ├── Method token: ADD
│  │  ├── Header
│  │  │  ├── name: add, return: Num, parameters: [{Value: x}, {Value: y}]
│  │  ├── Scope:
│  │     └─ Return token: {Value: z}
│  │     ├── While Token:
│  │        ├── Header: {while target: {Value: a}}
│  │        └── Scope:
│  │           └─ Variable token: {name: d, Assignment: {Value: 5}}
│  │           ├── While Token:
│  │              ├── Header: {while target: {Value: b}}
│  │              └── Scope:
│  │                 └─ Variable token: {name: c, Assignment: {Value: 5}}
├── Scope:
│  ├── Variable token: {name: x, Assignment: {Value: 5}}
```

