# todo
- mechanism to get state ref
- function to print state chart

- self transition optional reentry
- no_std support seems feasible

# attributes
- StateEnum name
- internal module

# errors / warnings
- init not implemented for branch state
    * optional
- dead states

# features
- no dynamic memory allocation

# shortcomings
- states must be Sized
- does not check for transition cycles
