
/**
Will obviously panic on the second call, but it's interesting to compare
`valgrind`'s output from panic = "abort" vs "unwind":

## Unwind

==53928== HEAP SUMMARY:
==53928==     in use at exit: 0 bytes in 0 blocks
==53928==   total heap usage: 12 allocs, 12 frees, 3,256 bytes allocated
==53928==
==53928== All heap blocks were freed -- no leaks are possible


## Abort

==53630== Process terminating with default action of signal 6 (SIGABRT)
...
==53630== HEAP SUMMARY:
==53630==     in use at exit: 1,112 bytes in 3 blocks
==53630==   total heap usage: 10 allocs, 7 frees, 3,184 bytes allocated
==53630==
==53630== LEAK SUMMARY:
==53630==    definitely lost: 0 bytes in 0 blocks
==53630==    indirectly lost: 0 bytes in 0 blocks
==53630==      possibly lost: 0 bytes in 0 blocks
==53630==    still reachable: 1,112 bytes in 3 blocks
==53630==         suppressed: 0 bytes in 0 blocks

*/
fn main() {
    drink("water");
    drink("lemonade");
    drink("still water");
}


/// Panic if asked for lemonade.
fn drink(beverage: &str) {
    if beverage == "lemonade" {
        panic!("AAAaaaaa!!!!");
    }
    println!("Some refreshing {} is all I need.", beverage);
}
