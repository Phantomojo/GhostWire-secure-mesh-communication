Crate crossterm Copy item path

<span class="sub-heading"><a href="../src/crossterm/lib.rs.html#1-260" class="src">Source</a>
</span>

Expand description

# <a href="#cross-platform-terminal-manipulation-library"
class="doc-anchor">§</a>Cross-platform Terminal Manipulation Library

Crossterm is a pure-rust, terminal manipulation library that makes it
possible to write cross-platform text-based interfaces.

This crate supports all UNIX and Windows terminals down to Windows 7
(not all terminals are tested see [Tested
Terminals](https://github.com/crossterm-rs/crossterm#tested-terminals)
for more info).

## <a href="#command-api" class="doc-anchor">§</a>Command API

The command API makes the use of `crossterm` much easier and offers more
control over when and how a command is executed. A command is just an
action you can perform on the terminal e.g. cursor movement.

The command API offers:

-   Better Performance.
-   Complete control over when to flush.
-   Complete control over where the ANSI escape commands are executed
    to.
-   Way easier and nicer API.

There are two ways to use the API command:

-   Functions can execute commands on types that implement Write.
    Functions are easier to use and debug. There is a disadvantage, and
    that is that there is a boilerplate code involved.
-   Macros are generally seen as more difficult and aren’t always well
    supported by editors but offer an API with less boilerplate code. If
    you are not afraid of macros, this is a recommendation.

Linux and Windows 10 systems support ANSI escape codes. Those ANSI
escape codes are strings or rather a byte sequence. When we `write` and
`flush` those to the terminal we can perform some action. For older
windows systems a WinAPI call is made.

### <a href="#supported-commands" class="doc-anchor">§</a>Supported Commands

-   Module [`cursor`](cursor/index.html)
    -   Visibility - [`Show`](cursor/struct.Show.html),
        [`Hide`](cursor/struct.Hide.html)
    -   Appearance -
        [`EnableBlinking`](cursor/struct.EnableBlinking.html),
        [`DisableBlinking`](cursor/struct.DisableBlinking.html),
        [`SetCursorStyle`](cursor/enum.SetCursorStyle.html)
    -   Position - [`SavePosition`](cursor/struct.SavePosition.html),
        [`RestorePosition`](cursor/struct.RestorePosition.html),
        [`MoveUp`](cursor/struct.MoveUp.html),
        [`MoveDown`](cursor/struct.MoveDown.html),
        [`MoveLeft`](cursor/struct.MoveLeft.html),
        [`MoveRight`](cursor/struct.MoveRight.html),
        [`MoveTo`](cursor/struct.MoveTo.html),
        [`MoveToColumn`](cursor/struct.MoveToColumn.html),[`MoveToRow`](cursor/struct.MoveToRow.html),
        [`MoveToNextLine`](cursor/struct.MoveToNextLine.html),
        [`MoveToPreviousLine`](cursor/struct.MoveToPreviousLine.html)
-   Module [`event`](event/index.html)
    -   Keyboard events -
        [`PushKeyboardEnhancementFlags`](event/struct.PushKeyboardEnhancementFlags.html),
        [`PopKeyboardEnhancementFlags`](event/struct.PopKeyboardEnhancementFlags.html)
    -   Mouse events -
        [`EnableMouseCapture`](event/struct.EnableMouseCapture.html),
        [`DisableMouseCapture`](event/struct.DisableMouseCapture.html)
-   Module [`style`](style/index.html)
    -   Colors -
        [`SetForegroundColor`](style/struct.SetForegroundColor.html),
        [`SetBackgroundColor`](style/struct.SetBackgroundColor.html),
        [`ResetColor`](style/struct.ResetColor.html),
        [`SetColors`](style/struct.SetColors.html)
    -   Attributes - [`SetAttribute`](style/struct.SetAttribute.html),
        [`SetAttributes`](style/struct.SetAttributes.html),
        [`PrintStyledContent`](style/struct.PrintStyledContent.html)
-   Module [`terminal`](terminal/index.html)
    -   Scrolling - [`ScrollUp`](terminal/struct.ScrollUp.html),
        [`ScrollDown`](terminal/struct.ScrollDown.html)
    -   Miscellaneous - [`Clear`](terminal/struct.Clear.html),
        [`SetSize`](terminal/struct.SetSize.html),
        [`SetTitle`](terminal/struct.SetTitle.html),
        [`DisableLineWrap`](terminal/struct.DisableLineWrap.html),
        [`EnableLineWrap`](terminal/struct.EnableLineWrap.html)
    -   Alternate screen -
        [`EnterAlternateScreen`](terminal/struct.EnterAlternateScreen.html),
        [`LeaveAlternateScreen`](terminal/struct.LeaveAlternateScreen.html)

### <a href="#command-execution" class="doc-anchor">§</a>Command Execution

There are two different ways to execute commands:

-   [Lazy Execution](#lazy-execution)
-   [Direct Execution](#direct-execution)

#### <a href="#lazy-execution" class="doc-anchor">§</a>Lazy Execution

Flushing bytes to the terminal buffer is a heavy system call. If we
perform a lot of actions with the terminal, we want to do this
periodically - like with a TUI editor - so that we can flush more data
to the terminal buffer at the same time.

Crossterm offers the possibility to do this with `queue`. With `queue`
you can queue commands, and when you call
[Write::flush](https://doc.rust-lang.org/std/io/trait.Write.html#tymethod.flush)
these commands will be executed.

You can pass a custom buffer implementing
[std::io::Write](https://doc.rust-lang.org/std/io/trait.Write.html) to
this `queue` operation. The commands will be executed on that buffer.
The most common buffer is
[std::io::stdout](https://doc.rust-lang.org/std/io/fn.stdout.html)
however,
[std::io::stderr](https://doc.rust-lang.org/std/io/fn.stderr.html) is
used sometimes as well.

##### <a href="#examples" class="doc-anchor">§</a>Examples

A simple demonstration that shows the command API in action with cursor
commands.

Functions:

``` {.rust .rust-example-rendered}
use std::io::{Write, stdout};
use crossterm::{QueueableCommand, cursor};

let mut stdout = stdout();
stdout.queue(cursor::MoveTo(5,5));

// some other code ...

stdout.flush();
```

The [queue](./trait.QueueableCommand.html) function returns itself,
therefore you can use this to queue another command. Like
`stdout.queue(Goto(5,5)).queue(Clear(ClearType::All))`.

Macros:

``` {.rust .rust-example-rendered}
use std::io::{Write, stdout};
use crossterm::{queue, QueueableCommand, cursor};

let mut stdout = stdout();
queue!(stdout,  cursor::MoveTo(5, 5));

// some other code ...

// move operation is performed only if we flush the buffer.
stdout.flush();
```

You can pass more than one command into the [queue](./macro.queue.html)
macro like `queue!(stdout, MoveTo(5, 5), Clear(ClearType::All))` and
they will be executed in the given order from left to right.

#### <a href="#direct-execution" class="doc-anchor">§</a>Direct Execution

For many applications it is not at all important to be efficient with
‘flush’ operations. For this use case there is the `execute` operation.
This operation executes the command immediately, and calls the `flush`
under water.

You can pass a custom buffer implementing
[std::io::Write](https://doc.rust-lang.org/std/io/trait.Write.html) to
this `execute` operation. The commands will be executed on that buffer.
The most common buffer is
[std::io::stdout](https://doc.rust-lang.org/std/io/fn.stdout.html)
however,
[std::io::stderr](https://doc.rust-lang.org/std/io/fn.stderr.html) is
used sometimes as well.

##### <a href="#examples-1" class="doc-anchor">§</a>Examples

Functions:

``` {.rust .rust-example-rendered}
use std::io::{Write, stdout};
use crossterm::{ExecutableCommand, cursor};

let mut stdout = stdout();
stdout.execute(cursor::MoveTo(5,5));
```

The [execute](./trait.ExecutableCommand.html) function returns itself,
therefore you can use this to queue another command. Like
`stdout.execute(Goto(5,5))?.execute(Clear(ClearType::All))`.

Macros:

``` {.rust .rust-example-rendered}
use std::io::{stdout, Write};
use crossterm::{execute, ExecutableCommand, cursor};

let mut stdout = stdout();
execute!(stdout, cursor::MoveTo(5, 5));
```

You can pass more than one command into the
[execute](./macro.execute.html) macro like
`execute!(stdout, MoveTo(5, 5), Clear(ClearType::All))` and they will be
executed in the given order from left to right.

## <a href="#examples-2" class="doc-anchor">§</a>Examples

Print a rectangle colored with magenta and use both direct execution and
lazy execution.

Functions:

``` {.rust .rust-example-rendered}
use std::io::{self, Write};
use crossterm::{
    ExecutableCommand, QueueableCommand,
    terminal, cursor, style::{self, Stylize}
};

fn main() -> io::Result<()> {
  let mut stdout = io::stdout();

  stdout.execute(terminal::Clear(terminal::ClearType::All))?;

  for y in 0..40 {
    for x in 0..150 {
      if (y == 0 || y == 40 - 1) || (x == 0 || x == 150 - 1) {
        // in this loop we are more efficient by not flushing the buffer.
        stdout
          .queue(cursor::MoveTo(x,y))?
          .queue(style::PrintStyledContent( "█".magenta()))?;
      }
    }
  }
  stdout.flush()?;
  Ok(())
}
```

Macros:

``` {.rust .rust-example-rendered}
use std::io::{self, Write};
use crossterm::{
    execute, queue,
    style::{self, Stylize}, cursor, terminal
};

fn main() -> io::Result<()> {
  let mut stdout = io::stdout();

  execute!(stdout, terminal::Clear(terminal::ClearType::All))?;

  for y in 0..40 {
    for x in 0..150 {
      if (y == 0 || y == 40 - 1) || (x == 0 || x == 150 - 1) {
        // in this loop we are more efficient by not flushing the buffer.
        queue!(stdout, cursor::MoveTo(x,y), style::PrintStyledContent( "█".magenta()))?;
      }
    }
  }
  stdout.flush()?;
  Ok(())
}
```

# Modules<a href="#modules" class="anchor">§</a>

<a href="cursor/index.html" class="mod"
title="mod crossterm::cursor">cursor</a>  
A module to work with the terminal cursor

<a href="event/index.html" class="mod"
title="mod crossterm::event">event</a>  
A module to read events.

<a href="style/index.html" class="mod"
title="mod crossterm::style">style</a>  
A module to apply attributes and colors on your text.

<a href="terminal/index.html" class="mod"
title="mod crossterm::terminal">terminal</a>  
A module to work with the terminal.

<a href="tty/index.html" class="mod" title="mod crossterm::tty">tty</a>  
A module to query if the current instance is a tty. Making it a little
more convenient and safe to query whether something is a terminal
teletype or not. This module defines the IsTty trait and the is\_tty
method to return true if the item represents a terminal.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.execute.html" class="macro"
title="macro crossterm::execute">execute</a>  
Executes one or more command(s).

<a href="macro.queue.html" class="macro"
title="macro crossterm::queue">queue</a>  
Queues one or more command(s) for further execution.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Command.html" class="trait"
title="trait crossterm::Command">Command</a>  
An interface for a command that performs an action on the terminal.

<a href="trait.ExecutableCommand.html" class="trait"
title="trait crossterm::ExecutableCommand">ExecutableCommand</a>  
An interface for types that can directly execute commands.

<a href="trait.QueueableCommand.html" class="trait"
title="trait crossterm::QueueableCommand">QueueableCommand</a>  
An interface for types that can queue commands for further execution.

<a href="trait.SynchronizedUpdate.html" class="trait"
title="trait crossterm::SynchronizedUpdate">SynchronizedUpdate</a>  
An interface for types that support synchronized updates.
