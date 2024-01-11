# Rusty Pomodoro Timer

Pomodoro timer and logger made with rust and [Ratatui](https://ratatui.rs/).
All your activities are saved in ledger format, [learn more about ledger here](https://ledger-cli.org/doc/ledger3.html), this makes it easy to analyse all the data of the time you spent in what activity. You can use all the analysis tools compatible with ledger to get information and statistics about how you spent your time.

![example](./assets/screenshot.png)

This is an example of a ledger entry generated by the program

```
2024/01/11 ProgrammingProjects
 Expenses:ProgrammingProjects  30
 Assets:MyTime

2024/01/11 ProgrammingProjects:Web
 Expenses:ProgrammingProjects:Web  5
 Assets:MyTime
```

You can later analyse your data, here I'm using ledger-cli:
```bash
ledger -f log/ledger.log register ProgrammingProjects 

>24-Jan-11 ProgrammingProjects      Expenses:ProgrammingProjects       30   30
>24-Jan-11 ProgrammingProjects:Web  Expenses:ProgrammingProjects:Web   5    35
```

You can add more activities by adding new entries to the vector in `src/app.rs`

## Keys

- `S` start timer

- `P` pause timer 

- `SPACE` start / pause

- `+` make a session 5 minutes longer

- `-` make a session 5 minutes shorter

- `left_arrow` change to previous activity

- `right_arrow` change to next activity


## TODO
- [x] Change session duration
- [x] Logs
- [x] Advanced statistics
