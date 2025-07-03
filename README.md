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

Belowe are more detailed information about them and how they are different
than the built-in Sway commands.

The commands are highly influenced by the way I use Sway workspaces. I want
to keep the workspaces of the same output next to each other and avoid any
intermediate workspaces on a different output. I also don't like the wrap
around to the first or last workspace of the built-in commands. Instead I
want the possibility to easily create new empty intermediate and trailing
workspaces to open windows on them.

The commands might not be useful for you if you have a different workflow
than me.

## Current limitations

This tool currently has some limitations:

* Workspaces must have different numbers
* Workspaces without a number are not supported. There might be some subtle
  bugs if there are workspaces without a number.

I think they can be fixed. I just didn't have a reason to fix them, because
I do not use those features of Sway. If you consider using this tool and
suffer from one of the limitaions feel free to open an issue.

# Command Description

## next

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

## prev

This command is similar to the `workspace prev` command. It moves the focus
to the previous workspace on the current output. But it will not move to the
last workspaces if you are currently on the first workspace. It will never
move to a workspace on a different output but will move the focus to missing
workspaces between the first and last one on the current output.

## move-next

This command moves the focused container to the next workspace and then
also moves the focus to the next workspace. So it is similar to a
combination of `move container to workspace <num>` and `workspace next`.

Most of the described behavior of the `sway-workspace-extras next` command
also applies to this command. One notable difference is the behavior on the
last workspace. This command will not do anything if the last workspace
contains only the focused window.

## move-prev

This command moves the focused container to the previous workspace and then
also moves the focus to the previous workspace. So it is similar to a
combination of `move container to workspace <num>` and `workspace prev`.

The same described behavior of the `sway-workspace-extras prev` command
also applies to this command.

## shift

This command creates a new empty workspace after the current one. It does
this by incrementing the workspaces number of the successors. If the next
workspace is already empty it does nothing.

# Development

## Sway workspace naming details

* Leading or trailing whitespace in a workspace name is removed. If you
  navigate to such a workspace and then look at the tree output

  ```
  swaymsg workspace "  2:name  "
  swaymsg -t get_tree -r
  ```

  you see that the workspace is named `2:name`

* The documentation mentions a `:` character to separate the number and the
  display name. This seems to be completely optional. The workspace gets a
  number as long as it starts with one or more digits. E.g.

  ```
  swaymsg workspace "2 name"
  swaymsg -t get_tree -r
  ```

  results in a workspace with a number of 2.

* The same number can be used for different workspaces as long as the whole
  workspace name is different. The ordering of workspaces with the same number
  seems to be a lexigraphical order. E.g. `1:a` comes before `1:b`.

* Equality comparison of workspace names is case insensitive. So, if you already
  have a workspace `1:name` and execute the command `swaymsg workspace "2:NAME"`
  you will end up at the existing workspace.

* Numbering of workspaces is optional. E.g. `swaymsg workspace without_name` is
  valid. Such a workspace has -1 as num property in the output of
  `swaymsg -t get_tree -r`.

* Workspaces without a number seem to come always last in the workspace list.
  Within the group of not numbered workspaces the individual workspaces are
  ordered by creation order.

