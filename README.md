[![CI](https://github.com/MerlinDMC/noderole/actions/workflows/ci.yml/badge.svg)](https://github.com/MerlinDMC/noderole/actions/workflows/ci.yml)

# noderole

noderole is a small statically linked program that can be used to label Kubernetes nodes.
In addition to normal labels it will also allow attaching labels in the
`node-role.kubernetes.io` namespace which in return will show up as a role in
the `kubectl get nodes` output.

# Installation

Binaries are built using GitHub Actions and can be fetched directly from the
[releases](https://github.com/MerlinDMC/noderole/releases) page.
The binaries are all statically linked against [musl libc](https://www.musl-libc.org)
and will run without any additional library dependencies.

## Configuration

To configure roles and additional labels to assign to a node noderole will read from a
YAML file. By default it will read and assign the config defined in `/etc/noderole.yml`.

### Example

```yaml
---
# list(string) of roles to assign
roles:
- role1
- role2

# map(string) of additional labels to assign
labels:
  your.namespace/key1: value1
  your.namespace/key2: value2
```
