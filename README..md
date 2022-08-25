# TRANSACTIONS

A toy payments engine

## Development decisions

### Errors handling

The only errors that could crash the application are related with incorrect input data. It means that all the records should be well-formed according the description. The logical error causing records are considered as non existed. As there is a demand on interface, all the errors are considered to be expressed in the error output. Only valid records are processed and cause the final result. So using the demanding call ```cargo run -- transactions.csv > accounts.csv``` will result a correct output, but the errors would be visible in error output.

### Numeric precision

To avoid a cumulative error it was decided to use fixed precision floats. So as it needs to use only additions and subtractions, integer is a good candidate to emulate fixed precision floats without cumulative error.

### Testing

For the testing it was decided to use unit tests for the most critical structure, ```Account```. As parsing issues and issues related with transaction id and client id are easily detected in runtime. Depending on the quality demands it would make sense to achieve 100% coverage as well.

### Efficiency

As there is no info on the size of input data it was decided to process the data record by record. So it will not require a lot of memory to process the whole data. Also for the purposes of efficiency it was decided to use the ```Vec``` instead of ```HashMap``` to work with clients as their id is ```u16```, it would be more efficient to reserve the space for ```65536``` entities instead of using the ```HashMap```.

### Maintainability
All the main entities are located in separate files. The only entity that is widely spread is ```TransactionError```, but it makes sense as the errors are processed at the most higher level.

## Possible improvements

  1. Write more tests to achieve ~100% coverage.
  2. Use 3rd party library for the fixed precision floats.
  3. Separate the tasks for reading from input data and processing them using async or multithreaded solution, but it would definitely crush the simplicity.