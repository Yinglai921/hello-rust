### Crate and modules
How the rust project folder structure looks like
- hello-package

crate: box, container

### Read input from cmd and files
- input-and-output
- check-name

### Enum
- hello-enum
- display-location

### Struct (class?)
struct is like tuple but it's a group of multiple items of mixed data types

elements are named

associated function: (static function?)
    
    function associated with a struct data type
    
    does not have a &self parameter

### Generic types
Abstract stand-ins for concrete data types or other properties

Can be used with structs, functions, methods and more

Defined with `<T>`

#### `Box<T>` 
- sum-boxes
store data on the heap instead of on the stack 
smart pointer: provide additional functions beyond references
`Box<T>` has ownership of the heap
use case for `Box<T>`:

- store a type whose size cannot be known at compile time, example: recursive types
- transfer ownership of data rather than copy it on the stack
  - avoid copying large amount of data 

### Trait (interface)
(feature, character)
Similar to interfaces in other programming languages

A collection of methods

Data types can implement a trait

Generics use traits to specify the capabilities of unknown data types

`impl {trait} for {struct}`



