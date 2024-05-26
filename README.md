# Junkfile Generator v2

## What is it?
With this tool, you can create files filled with random characters. This can help you fool some teachers in school: "Looks like the presentation is corrupted, I will have do do it another time." 

## How to use the tool?

To generate a file, compile the project and execute it with the following commandline arguments:

```
./junkfilegenv2 -p <path of the file> -s <size in bytes> [-o] [-l] [-d]
```

Explanation of the parameters:

- the `-p` parameter specifies the path of the file to be created.
- the `-s` parameter specifies the size of the file.
- the `-o` flag tells the program to always generate the file even if another file already exists at the given path. If this flag is not set, you will be prompted during execution.
- the `-l` flag changes the rng generator to only output readable characters. See `PRINTABLE_CHARS` string in `content.rs`.
- the `-d` flag tells the program to always use `rand` as the option to retrieve random bytes.


## Adding the tool as a terminal command

By executing the add_command.sh script, the Junkfile generator can be added to the `/bin` folder easily. The script is a wrapper for the `cp` command.

Example:
```
sudo ./add_command.sh -s target/release/junkfilegenv2 -n gen-junkfile
```

Explanation of the parameters:
- `-s` specifies the source file.
- `-n` specifies the name that the command should receive (i.e. the filename in the `/bin` folder).


## Why v2?
This is a rewrite of the Junkfile Generator.

It uses `clap` to parse the commandline arguments and the Linux file `/dev/random` as the source for random characters.
If `/dev/random` is not available, `rand` is used to retrieve random bytes.

The reason for the change to `/dev/random` is the incredible performance boost over `rand`.
