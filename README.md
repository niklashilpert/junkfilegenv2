# Junkfile Generator v2

## What is it?
With this tool, you can create files filled with random characters. This can help you fool some teachers in school: "Looks like the presentation is corrupted, I will have do do it another time." 

## How to use the tool?

To generate a file, compile the project and execute it with the following commandline arguments:

```
./junkfilegenv2 -p <path of the file> -s <size in bytes> [-o]
```

Explanation of the parameters:

- the `-p` parameter specifies the path of the file to be created.
- the `-s` parameter specifies the size of the file.
- the `-o` flag tells the program to always generate the file even if another file already exists at the given path. If this flag is not set, you will be prompted during execution.

## Why v2?
This is a rewrite of the Junkfile Generator.

It uses `clap` to parse the commandline arguments and the Linux file `/dev/random` as the source for random characters.
This means that is will only work on machines that run Linux.

The reason for the change to `/dev/random` is the incredible performance boost over the `rand` version.