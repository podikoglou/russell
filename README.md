# russell
A tiny little Propositional Logic engine, made while I was studying for my
Logic class. This was written for both educational and recreational purposes,
as I wanted to see how evaluation of logical expressions is typically done.

A few things:
- russell can "prove" whether a logical proposition is a tautology or not, by
  generating truth tables and evaluating the proposition for each row. It can't
  show that something is a contradiction *yet* (but it's trivial to implement).

- russell is *naive*. It doesn't currently do any term rewriting or
  simplifications of any sort.

## An Example
This is what De Morgan's laws look like encoded in russell's syntax:

```js
!(p && q) == !p || !q
```

```js
!(p || q) == !p && !q
```

When we run `echo '!(p && q) == !p || !q' | cargo r`, we get:
```language
[russell/src/main.rs:52:9] engine.check_tautology(expr)? = true
```

See more examples at the
[examples](https://github.com/podikoglou/russell/tree/main/examples) directory,
where I have implemented some foundational properties/laws.
