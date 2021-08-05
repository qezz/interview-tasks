# Price Match (Cache Lookup and Variations)

## Disclaimer

This task has been discussed during a coding interview/coding session.
My original solution is slightly changed, though the main idea remain
the same.

It's important to understand that this solution isn't the best, hence
I want to improve it. 

The description of the original task has been changed in order to be easier 
to understand. To be honest, understanding the task was the hardest part 
of the interview.

## Task Description

A tender. Or maybe an auction, doesn't matter. Companies suggest 
the price of a mystical goods. The seller has some baseline price in mind, 
but doesn't announce it. If a company suggests a price lower than the
baseline, the seller wants to choose a company that will pay more than others.

There's a third-party algorithm that chooses a company to sell the goods to. 
We don't know how it works, but it returns the following information:

1. All the suggestions analyzed
2. A company to sell to
3. Baseline set by the seller

Let's say, it comes as following:

```rust
struct {
    all_data: { "google": 100, "amazon": 200 },
    buyer: "amazon",
    baseline: 300,
}
```

For you, as a seller, it is crucial to sell to the company that will pay more 
than others. But firstly, you need to validate the output of the algorithm.

You set the following rules:

1. If a company (`buyer`), suggested bf the algorithm, pays exactly the price 
you want (`baseline`), return `Outcome::Exact`.
2. If a company, suggested by the algorithm, won't pay the baseline, make sure
that the suggested company pays more than others. Return `Outcome::MoreThanOthers`.
3. In all other cases, return `Outcome::WrongResults`

The task is to design the validator function.

When writing the tests, consider these:

Simple:

```rust
let all_data = [("amazon", 200), ("google", 100)] ... ;
let predicted = "amazon";
let baseline = 200;

Correct: Outcome::Exact
```

Two companies suggested the same price:

```rust
let all_data = [("amazon", 200), ("google", 200)] ... ;
let predicted = "amazon";
let baseline = 200;

Correct: Outcome::Exact
```

No match with the baseline, but Algorithm suggested the higher price:

```rust
let all_data = [("amazon", 100), ("google", 150)] ... ;
let predicted = "google";
let baseline = 200;

Correct: Outcome::MoreThanOthers
```

No match with the baseline, and Algorithm suggested the wrong company:

```rust
let all_data = [("amazon", 100), ("google", 150)] ... ;
let predicted = "amazon";
let baseline = 200;

Correct: Outcome::WrongResults
```

## Disclaimer #2

It seems logical to assume the following situation:

```rust
let all_data = [("amazon", 200), ("google", 400)] ... ;
let predicted = "amazon";
let baseline = 200;

Expected: Outcome::Exact
```

Although "google" suggested the higher price, the algorithm predicted 
the buyer for the exact baseline. As of now, **this case is not covered**.
It is possible that I'll take it into the consideration later.

## Solutions

- [My solution](src/v1_baseline.rs) - the solution provided by me during the interview