## lps - Loris' parallel search

lps is a simple Rust command line tools that combines `find` and `grep` but is 2 to 10 times faster depending on the amount of files and size

#### How does it work?

lps first traverses the directory structure sequential, gathering filtered files (name, attributes, size, ...), then distributes the work over a defined amount of worker threads.

#### Usage

lps can be used in two modes, file search and content search which includes file search.

Content search is enabled by specifiying the `--content` parameter.

List of available parameters

| Short |          Long          |                          Description                         | Requires |         Default        |
|:-----:|:----------------------:|:------------------------------------------------------------:|:--------:|:----------------------:|
|   -h  |         --help         |              Shows all commands and explanations             |     -    |            -           |
|   -V  |        --version       |                 Displays version information                 |     -    |            -           |
|   -v  |        --verbose       |                    Enables verbose output                    |     -    |          false         |
|   -n  |         --name         |                  Filter files based on name                  |     -    |  No files are filtered |
|   -b  | --ignore-filename-case | Ignores the casing of file names when name filtering is used |    -n    |          false         |
|   -c  |        --content       |                    Search content of files                   |          | No content is searched |
|   -x  |  --ignore-content-case |     Ignores casing of content when content search is used    |    -c    |          false         |
|   -d  |          --dop         |  Sets the amount of worker threads to use for content search |    -c    |   Logical core count   |

The first positional argument is used to set the root search directory, defaults to current working directory.

#### Output format

##### File search

In file search mode, the output will be a plain list of paths to found files, printed to stdout:

Example call `lps -n lock C:\`

```
C:\ProgramData\data.lock
C:\ProgramData\db\file.lock
...
C:\Users\dev\lock.txt
...
```

Permission errors can occur, but they'll be printed to stderr.

##### Content search

In content search mode, lps will print a list of file paths and the occurrences of the search term:

```
<path>
  <line>:<column><whitespace><text>
  <line>:<column><whitespace><text>
<path>
  <line>:<column><whitespace><text>
  <line>:<column><whitespace><text>
```

###### Note the 2 space indentation before every occurrence in the file

Example call `lps -c result C:\`

```
...
C:\Users\dev\projects\cpp\main.cpp
  6:9     auto result = 1;
  10:8         result = 0;
C:\Users\dev\projects\rs\main.rs
  2:4     let result = 5;
...
```

Permission errors can occur, but they'll be printed to stderr.

##### Combining file and content search

If you combine these modes, lps will only scan filtered files.

Example call `lps -n .cpp -c result`

```
C:\Users\dev\projects\cpp\main.cpp
  6:9     auto result = 1;
  10:8         result = 0;
...
```

##### I'm a Rust learner; improvements and recommendations are very welcome