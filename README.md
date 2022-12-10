# sway-workspace-extras

This application contains a set of additional [Sway](https://swaywm.org/)
commands. Use

```
sway-workspace-extras --help
```

to see the supported commands. Currently those are:

* next
* prev
* move-next
* move-prev
* shift

The next section contains more detailed information about them and how they
are different than the built-in Sway commands.

The commands are highly influenced by the way I use Sway workspaces. I want
to keep the workspaces of the same output next to each other and avoid any
intermediate workspaces on a different output. I also don't like the wrap
around to the first or last workspace of the built-in commands. Instead I
want the possibility to easily create new empty intermediate and trailing
workspaces to open windows on them.

The commands might not be useful for you if you have a different workflow
than me.

# Command Description

# next

This command is similar to the `workspace next` command. It moves the focus
to the next workspace on the current output. But there are key differences
regarding output handling, wrap around and gaps between workspaces.

The command will never move to a workspace on a different output. It only
targets workspaces that are on the same output.

Furthermore it doesn't move to the first workspace if you are currently on
the last workspace of the current output and execute the command. Instead
it will move to a new empty trailing workspace. This makes opening a
window on a fresh workspace fairly simple.

It also move to missing (w.r.t the number) workspaces that are between the
first and last workspace on the current output.

# prev

This command is similar to the `workspace prev` command. It moves the focus
to the previous workspace on the current output. But it will not move to the
last workspaces if you are currently on the first workspace. It will never
move to a workspace on a different output but will move the focus to missing
workspaces between the first and last one on the current output.

# move-next

This command moves the focused container to the next workspace and then
also moves the focus to the next workspace. So it is similar to a
combination of `move container to workspace <num>` and `workspace next`.

Most of the described behavior of the `sway-workspace-extras next` command
also applies to this command. One notable difference is the behavior on the
last workspace. This command will not do anything if the last workspace
contains only the focused window.

# move-prev

This command moves the focused container to the previous workspace and then
also moves the focus to the previous workspace. So it is similar to a
combination of `move container to workspace <num>` and `workspace prev`.

The same described behavior of the `sway-workspace-extras prev` command
also applies to this command.

# shift

This command creates a new empty workspace after the current one. It does
this by incrementing the workspaces number of the successors. If the next
workspace is already empty it does nothing.
