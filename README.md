# sstr

stupid simple task runner

sstr is a language agnostic and simple task runner.

## Setup

+ download the source and compile with `go build`
+ create an `sstr` file (no extensions) wherever
+ paste the following:

```json
{
    "commands": {
        "echo": "echo echo",
    }
}
```

+ edit key/value pairs under the `commands` object to edit available tasks
+ the key is the name passed to `sstr` cli, the value is the command actually run by `sstr`

For instance, a completed configuration for a `node.js` project could look like:

```json
{
    "commands": {
        "build": "node ./build.js",
        "test": "npx ava"
    }
}
```

## Notes

+ for now, all binaries must be in `PATH`
