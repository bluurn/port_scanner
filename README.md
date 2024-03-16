# Port scanner

to run, use:

```shell
> port_scanner -h
Accepts IPv4 address and number of threads
If you want to print the processes list for localhost, run
lsof -i :$(port_scanner | tail -n +2 | tr '\n' ',')

Usage: [-i ARG] [-M ARG] [-m ARG]

Available options:
    -i, --ipaddr <ARG>    IPv4 address, defaults to 127.0.0.1
    -M, --max-port <ARG>  Maximal port, defaults to 65535
    -m, --min-port <ARG>  Minimal port, defaults to 1
    -h, --help            Prints help information
    -V, --version         Prints version information
```
