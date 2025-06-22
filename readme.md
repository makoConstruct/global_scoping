# global scoping

*Global replace scoping* or *override scoping* is a pattern that mitigates the problems of (thread-local) mutable global state. It's also arguably a better solution to context management than *implicit parameter* passing could be.

The main problem with mutable global state is that changes from one unit of code leak invisibly and unpredictably into others. *Override scoping* addresses this by confining changes to a scope, then reliably restoring the global to its previous state when the scope ends, meaning that changes imposed within a function will never escape that function. Changes go down, but not up.

Implicit parameters are a kind of nice feature that some languages have, where variables or globals that are marked as `context` can be slurped up into a function call without the programmer needing to pass them explicitly. This is a genuinely important feature in situations where there are parameters that are often inherited from their context, but where the user needs to override them rarely and shouldn't have to think about them otherwise. An example would be CSS. CSS isn't a programming language, and I contend that the *reason* CSS couldn't have been implemented as part of a real programming language is because your programming languages lack a way of cascading variables implicitly from context that also allows succinctly overriding them when needed.

So we can say in the worst cases the lack of implicits manifests as a severing of infeasible APIs into limited DSLs. In milder cases it manifests as a `Context ctx` parameter that you have to pass around everywhere all the time, which also results in inefficient movement or reconstruction of contextual configuration. In milder cases, it results in a perfusion of generally unimportant parameters.

Implicits parameter passing looks something like this:

```scala
implicit b:i32 = 2

function f(a: i32, implicit b: i32){
    print(a + b)
}

f(1)
// 3

b = 10
f(1)
// 11

f(1, 3)
// 4
```

A problem with implicits is that they're not implicit enough. Functions have to mention them in order for them to be propagated through those functions. Functions that don't use an implicit will essentially 'reset' that implicit to default, for any functions they call. To the programmer, not taking the implicit seems like a non-action, but to the functions that depend on that implicit, it's a destructive action that you basically never want.

We could introduce a new kind of implicit that propagates even when not mentioned.

Or we could just use reformed globals.

```scala
contextual b:i32 = 2

function f(a: i32){
    print(a + b)
}

f(1)
// 3

{
    b = 10
    f(1)
    // 11
}

f(1)
// back to 3 again
```

Instead of providing a nice syntax like that, this library provides an implementation of this pattern in Rust, because I was curious about how well it could be made to work in rust with the use of thread locals (scoped replacements doesn't make any sense under multithreading), it already seems like it doesn't work very well, and I haven't published it. Even as thread locals, the syntax is not very nice, requiring `contextual.with(|c| ...)` blocks whenever the contextual needs to be accessed. I also don't think this would work under concurrency (like, `await`), that would significantly complicate things. An await scope would have to save and revert all of its contextual overrides before yielding. To do that efficiently, you'd have to remember which ones haven't been changed, which would ideally use first class support from the compiler.

Maybe we *should* just have an implicit `ctx` immutable map...