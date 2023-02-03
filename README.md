# Sniffer

![sniffer](./sniffer.gif)
A simple ip port scanner.

## Usage

- Show help info.:

```
sn -h // show help information
```

- Scan the specified ip address's ports.

`sn [ip_address]`:

Example:

```
sn 192.168.1.1
```

- Scan the specified ip address's ports using the specified number of threads

`sn -n [number_of_threads] [ip_address]`

Example:

```
sn -n 100 192.168.1.1 // -n: specify the number of threads
```
