# Code

Code examples.

## Code

Just an example file.

<center>
<img src="https://placekitten.com/200/300">
</center>

### Example code

Here is some example `JavaScript` code:

```js
const express = require('express')
const app = express()
const port = 3000

app.get('/', (req, res) => res.send('Hello World!'))

app.listen(port, () => console.log(`Example app listening on port ${port}!`))
```

Just some go code:

```go
package main

import "fmt"

// This `person` struct type has `name` and `age` fields.
type person struct {
    name string
    age  int
}

// NewPerson constructs a new person struct with the given name
func NewPerson(name string) *person {
    p := person{name: name}
    p.age = 42
    return &p
}

func main() {

    // This syntax creates a new struct.
    fmt.Println(person{"Bob", 20})

    // You can name the fields when initializing a struct.
    fmt.Println(person{name: "Alice", age: 30})

    // Omitted fields will be zero-valued.
    fmt.Println(person{name: "Fred"})

    // An `&` prefix yields a pointer to the struct.
    fmt.Println(&person{name: "Ann", age: 40})

    // It's idiomatic to encapsulate new struct creation in constructor functions
    fmt.Println(NewPerson("Jon"))

    // Access struct fields with a dot.
    s := person{name: "Sean", age: 50}
    fmt.Println(s.name)

    // You can also use dots with struct pointers - the
    // pointers are automatically dereferenced.
    sp := &s
    fmt.Println(sp.age)

    // Structs are mutable.
    sp.age = 51
    fmt.Println(sp.age)
}
```

And some html code `html` here it is:

```
html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <title>Example HTML5 Document</title>
</head>
<body>
  <p>Test</p>
</body>
</html>