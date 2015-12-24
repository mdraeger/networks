# Networks
This library represents a subset implementation of networks according to 
Ahuja, Magnati, Orlin: "Network Flows".

It is also work in progress, with very few algorithms working yet.

## Node numbering
Other than in the book, nodes in this implementation are numbered from `0`. 
Anything else would have been harder to implement and would also feel quite
unnatural.

## Test tool
For two algorithms, Dijkstra and PageRank(TM), there is a command line test
tool available. 

Note: PageRank(TM) is not in the book, I implemented it anyway because the 
problem came up in a different context.

One more note: Though tempting (i.e. you are confident that your graph is 
fully connected, has no dead ends and no spider traps), it is highly 
discouraged to use a teleport probability (parameter `beta`) of `0.0`. 
Due to floating point precision, the sum of the probability vector 
(a.k.a. the page ranks) can exceed `1.0` and the algorithm will panic.

### Test tool usage
Type `test_network -h` to see a list of available command line options. 
One note regarding the regular expression pattern for the parsing of the 
input file: The implemented default is `<from>.<to>   <cost> <something>`.
If your input file deviates from that pattern, you need to provide a 
matching pattern like 
```
"(?P<from>[[:alnum]]*)\\s+(?P<to>[[:alnum]]*)\\s+(?P<cost>\\d*)\\s+(?P<cap>\\d*)"
```
If your algorithm doesn't need costs and capacities (like in PageRank(TM)), 
the captures are optional.

## Input file formatting.
Your input file must obey the same pattern for every line that contains an 
arc. Empty lines are not allow. You can have an arbitrarily long header,
the length (in lines) will be supplied to the `--skip=<#header lines>`
parameter.

## TODO
- Implement more algorithms. I take suggestions with which I should start.
- Add more options to the test tool in order to provide more algorithms and
  more control over the output.
- Implement more alternative network representations. For now, only compact
  star is offered.
