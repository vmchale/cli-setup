# cli-setup

Documentation [here](https://docs.rs/cli-setup/0.1.0/cli_setup/), example use
[here](https://github.com/vmchale/project-init/blob/master/build.rs).

I recommend writing manpages using [pandoc](http://pandoc.org/). You can write
a markdown document and generate a manpage using

```bash
 $ pandoc EXECUTABLE.md -s -t man -o man/executable.1
```

For an example of how to write a manpage using markdown, see
[here](https://raw.githubusercontent.com/vmchale/project-init/master/man/MANPAGE.md).
