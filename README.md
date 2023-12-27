# lagging_server

A simple web server use to benchmark sozu. Originally written by @Keksoj

## Build

```bash
cargo install --path .
```

## Run

```
lagging_server --port 1054
```

You can run several ones, in several terminals, for instance:

```bash
lagging_server --port 1052
```

_Et cætera_

## Request it with curl

| curl command                                                    | outcome                                         |
| :-------------------------------------------------------------- | :---------------------------------------------- |
| `curl http://0.0.0.0:1054/api`                                  | replies "Hey there!"                            |
| `curl http://0.0.0.0:1054/latency`                              | replies after a latency period, tells about it  |
| `curl http://0.0.0.0:1054`                                      | should hang for 10 seconds                      |
| `curl -X POST -d "Do you here me?"  http://0.0.0.0:1054/echo`   | echoes back to you                              |

Try it!
