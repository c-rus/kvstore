# `kvstore`

# A command-line key-value database tool.

Inspired by the Microsoft Reactor Rust programming talk __[Rust Programming: Moving Beyond “Hello World”](https://www.youtube.com/watch?v=5dRT_v3hrZ0)__ by Ryan Levick.

```
kvstore is a key-value keeper.

Usage:
    kvstore [<key>] [<value>] [flags]

Args:
    <key>       label to identify data
    <value>     data to store behind a label

Flags:
    --init      list pairs in exporting environment format to stdout 
    --append    add additional <value> to existing value behind given <key>
    --version   print the current version
    --help      print help information

More:
    Enter only a <key> to view its value. To view all values, pass '.'.

    kvstore's database is a 'kv.db' file located where the program is ran
    unless the environment variable KVSTORE_HOME is set to an existing 
    directory.
```

## Editing

Adding/updating a key will require you to pass the new value for it as `value`. Values will be overridden if one already exists for the given `key`.

``` 
$ kvstore hello world
kv-info: Save successful
```

## Viewing

Viewing a key's value only requires the `key` argument to be passed. A key that does not exist will return a blank line.

``` 
$ kvstore hello
world
```

## Initializing

You can initialize the database as environment variables for the current terminal session. For zsh/bash, this can look like the following:

```
$ export `kvstore --init`
```

> __Note:__ You can include that line in a terminal profile file to run everytime a terminal session is created.

To preview what variables would be set, run without command substitution.

```
$ kvstore --init
hello=world
```

> __Note:__ If no output is visible, that may mean either the environment variables are already set from the key-values, or you have have zero key-values in your database.

## Ideas/Extensions

- [ ] `--preview` show the before/after state of key before asking user if it's okay to save when editing. 

- [x] `--init` could set the key-value pair as an env variable in the current working terminal session. Having no key & value arg will default to initialize all key-values as environment variables. 

- [ ] `--home=<dir>` to override a particular location of `kv.db` file for the given program call. Has precedence over `KVSTORE_HOME`.

- [x] Pass '.' as key to view all key-value pairs (skips keys with blank values).

- [x] allow user to define where to place `kv.db` file using an env variable `KVSTORE_HOME`.

- [ ] allow special syntax to prepend/append to an existing key-value pair.

    `$ kvstore rpath @:/usr/local/bin`
    
    Here the `@` symbol copies the existing value for key `rpath` to be restored with `:/usr/local/bin` appended. A current workaround is to use shell like: 

    `$ kvstore rpath $(kvstore rpath):/usr/local/bin`
