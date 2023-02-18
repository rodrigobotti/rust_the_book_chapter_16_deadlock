# Application that may deadlock

**DISCLAIMER**

Deadlocks are non deterministic. 
In order to observe one the only choice we have is to run the program multiple times.
If we run it enough times we should be able to reach a deadlock state.

```sh
cargo build --release
while true; do ./target/release/chapter_sixteen; done
```

In this example it took 3 runs of the program to observe a deadlock -- 
it may take more or less runs for you because of the non deterministic nature of the problem -- 
generating the following output.

```sh
$ while true; do ./target/release/chapter_sixteen; done
-----------------
-- May deadlock --
Transfering from B to A
Transfering from A to B
-- May not be called --
Balance A = 13
Balance B = 2
------------------
-- May deadlock --
Transfering from A to B
Transfering from B to A
-- May not be called --
Balance A = 13
Balance B = 2
------------------
-- May deadlock --
Transfering from B to A
Transfering from A to B

```

The program got stuck because there are two threads waiting on each other to release locks (check the source code for further explanation on why this happens).