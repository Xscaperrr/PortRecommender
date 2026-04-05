---
name: port-recommender
description: Use this skill when the user wants a deterministic, currently usable port recommendation for a service or application name, or wants to map one or more names to ports while avoiding common ports and ports already occupied on the local machine.
---

# Port Recommender

Use this skill to get a recommended port from a locally available `port-recommender` binary.

## When to use

Trigger this skill when the user asks for any of the following:

- recommend a port for a service name
- assign a stable local port for an app, worker, agent, API, database shadow, or dev server
- avoid common ports and already-listening ports
- map one or more names to deterministic ports

## What the CLI does

The CLI:

- hashes the input name with `SHA-256`
- maps it into the `1024-65535` range
- excludes built-in common ports
- excludes ports currently occupied on the machine
- supports `--protocol tcp|udp|both`
- prints only the final port number on success

The result is stable for the same name only when the local exclusion set is unchanged.

## How to use it

Default to a local existing binary. Do not use `cargo run`, `cargo build`, or other build commands as the normal execution path.

Use this resolution order:

1. `port-recommender` from `PATH`
2. a known local binary path supplied by the user
3. an already-existing repository binary such as `target\release\port-recommender.exe` on Windows or `target/release/port-recommender` on Unix, if it is already present

Default to `both` unless the user clearly wants only TCP or only UDP.

Preferred commands:

```powershell
port-recommender <name> --protocol both
port-recommender <name> --protocol tcp
port-recommender <name> --protocol udp
```

If the binary is not on `PATH`, use its existing absolute or workspace-local path directly.

```powershell
target\release\port-recommender.exe <name> --protocol both
```

If no local binary is available, stop and report that the `port-recommender` executable is missing. Do not silently fall back to Cargo-based execution.

## Response guidance

- Return the chosen port as a number plus a short explanation of which protocol scope was used.
- If the user gave multiple names, run the CLI separately for each name and present a compact mapping.
- If the command fails, report the stderr message directly and state that no port was recommended.
- If the binary cannot be found, say that this skill expects a prebuilt local `port-recommender` executable.
- Do not claim the port is globally reserved; describe it as a current recommendation based on local machine state.

## Examples

Single name:

```text
User: Recommend a port for "api-gateway"
Action: port-recommender api-gateway --protocol both
```

UDP-oriented service:

```text
User: Give me a UDP port for telemetry-agent
Action: port-recommender telemetry-agent --protocol udp
```

Multiple names:

```text
User: Recommend ports for api-gateway, worker, and scheduler
Action: run the CLI once per name and return a name -> port mapping
```
