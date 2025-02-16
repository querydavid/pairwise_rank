This is a simple tool to implement a pairwise ranking algorithm in Rust.

Usage:

```
$ pairwise_rank

What single, simple question are you trying to answer?

Enter items to rank separated by commas:
```

User is then presented with an exhaustive set of two-item combinations from the entered list and ranks them by pressing "k" or "l".

The top-ranked item can be considered "definitively" the most-preferred outcome. The ordering of the other items is not guaranteed to reflect "true" preferences (as not all information about the rankings is preserved, only the total counts).

Roadmap, if I update it:
* Implement a Condorcet method or Elo ranking
* Add a timeout
* Move the implementation to a prettier TUI
