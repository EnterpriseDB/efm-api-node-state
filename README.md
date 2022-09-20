# EDB Failover Manager (EFM) API Node State

`efm-api-node-state` is an HTTP server serving a REST API and exposing the
state of the current EFM node.

Through this API, loadbalancers and traffic routers will be able to know if the
current EFM node is the primary node of the cluster, a standby node, or,
something else (not a primary node and not a standby node) and make appropriate
changes on traffic routing.

# API

## GET /primary

Based on the presence of a specific EFM trigger file, this API call returns
an HTTP response code set to `200` if the current node is the primary node of
the EFM cluster.

Example:
```http
HTTP/1.1 200 OK
content-length: 16
content-type: application/json
date: Tue, 13 Sep 2022 11:20:05 GMT

{"message":"OK"}
```

If the current EFM node is not the primary one, or its state can't be checked,
it returns an HTTP error with a response code set to `500`:
```http
HTTP/1.1 500 Internal Server Error
content-length: 16
content-type: application/json
date: Tue, 13 Sep 2022 11:24:57 GMT

{"message":"KO"}
```

## GET /standby

Works like `GET /primary` but returns `200` if the current EFM node is a
standby node.

# Installation procedure on CentOS7/RHEL7/OracleLinux7

## Package installation

The RPM packages are available via the GitHub release page.

Package installation:
```shell
$ sudo rpm -ivh efm-api-node-state-0.2.0-1.el7.x86_64.rpm
```

## Configuration

The configuration file is located at `/etc/edb/efm-api-node-state/config.toml`.

Default configuration:
```toml
[config]
primary_command = "/usr/edb/efm-api-node-state/scripts/efm_check_primary.sh 4.5 main"
standby_command = "/usr/edb/efm-api-node-state/scripts/efm_check_standby.sh 4.5 main"
listen_addr = "0.0.0.0"
port = 9000
shell = "/bin/sh"
# DEBUG, INFO, WARN, ERROR
log_level = "DEBUG"
```

## EFM configuration

EFM configuration must be updated with the following properties:
```
script.load.balancer.attach=/bin/touch /var/run/efm-4.5/efm_trigger_%t
script.load.balancer.detach=/bin/rm -f /var/run/efm-4.5/efm_trigger_%t
script.custom.monitor=/usr/edb/efm-api-node-state/scripts/efm_monitor.sh 4.5
custom.monitor.interval=5
custom.monitor.timeout=10
custom.monitor.safe.mode=true
auto.resume.period=5
```

### Settings

| Setting             | Description                                                                      |
| ------------------- | -------------------------------------------------------------------------------- |
| **primary_command** | System command executed for checking if the current EFM is the **primary** node. |
| **standby_command** | System command executed for checking if the current EFM is a **standby** node.   |
| **listen_addr**     | Network interface the HTTP server is listening on.                               |
| **port**            | TCP port number the HTTP server is listening on.                                 |
| **shell**           | Shell used for executing the commands.                                           |
| **log_level**       | Log messages verbosity.                                                          |

## firewalld configuration

```shell
$ sudo firewall-cmd --add-port=9000/tcp --permanent
$ sudo firewall-cmd --reload
```

## Enabling and starting the service

```shell
$ sudo systemctl enable efm-api-node-state
$ sudo systemctl start efm-api-node-state
```

## Logs access

```shell
$ sudo journalctl -ru efm-api-node-state
```
