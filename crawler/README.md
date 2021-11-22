# (Some kind of) Crawler

This task was defined on a recent interview. Unfortunately, it looks like during the dialog some miscommunication happened.
I think there were some doubts regarding my algorithm of recursively traversing the graph.

## Task

Each Node has an address, and returns a list of known addresses once an API call made.
Collect the most comprehensive list of observable nodes.

## Idea

Core idea of my solutions is following:

While recursively traversing a graph, putting all addresses one-by-one into a registry will eventually lead to all reachable addresses being discovered.

As a side effect, while traversing, the repeated references will be cutted-off, hence we'll be getting a tree-like representation of the graph. 
A starting address acting as a root node.

An execution branch will return (i.e. finished its recursion) once all the nodes from the list are processed:

* Either a Node was added to the list of the known list (registry) AND the recursive call happened on this new Node.
* Or it was simply ignored due to already being in the registry

> TODO: Add pictures

## Solutions

### Kick start

[simple.rs](src/simple.rs) - Not an actual solution at the moment, go next

### Better, with threads and cache/registry

[better.rs](src/better.rs) - Here we talk. 

* Pros
   * multithreaded
   * recursive (stops once there is nothing to do)
* Cons
   * Mutex locks could be very long
   * Hard to interrupt the process

### Better (v3)

[v3_better.rs](src/v3_better.rs) - It is possible to interrupt the crawling process. More tests.

### Tokio (Bad, v4)

[v4_tokio_wrong.rs](src/v4_tokio_wrong.rs) - Into the tokio field.

Recursive async functions are not allowed in rust (yet?). Anyway, the compiler cannot derive the type.

This version utilizes `deadqueue::unlimited::Queue` as an event queue. External `oneshot` receiver is required to manually stop the processing task.

* Pros:
   * It is possible to manually stop the process with a stop token ('poisoning token')
   * It is possible to interrupt the task using tokio, e.g. with `tokio::time::timeout`
* Cons:
   * `do_crawl_and_return()` does not return the result; only able to print it to console
   * External `oneshot` receiver is required to manually stop the processing task.

### Tokio (Better, v5)

[v5_tokio_wrong.rs](src/v5_tokio_wrong.rs)

`do_crawl_and_return()` finally returns a value!

* Pros
   * Mutexes are locked for a really short times, only for updates
   * Returns a value
