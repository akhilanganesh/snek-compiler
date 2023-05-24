# Snek Specification

## Concrete Syntax

The concrete syntax of a Snek program is shown below. This includes but is not limited to integers, booleans,
variables, let bindings, unary operators, binary operators, conditional and control expressions, functions,
and function calls.

```
<prog> := <defn>* <expr>
<defn> := (fun (<fname> <identifier>*) <expr>)
<expr> :=
  | <integer>
  | <boolean>
  | <tuple>
  | input
  | <identifier>
  | (let (<binding>+) <expr>)
  | (<op1> <expr>)
  | (<op2> <expr> <expr>)
  | (if <expr> <expr> <expr>)
  | (loop <expr>)
  | (break <expr>)
  | (set! <identifier> <expr>)
  | (tinit <expr:integer> <expr>)
  | (tset <expr:tuple> <expr:integer> <expr>)
  | (tget <expr:tuple> <expr:integer>)
  | (block <expr>+)
  | (<fname> <expr>*)

<integer>    := (-)?[0-9]*
<boolean>    := true | false
<tuple>      := (tuple <expr>*)
<expr:[value]>       :=       <expr> that holds type [value]

<op1> := add1 | sub1 | isnum | isbool | print
<op2> := + | - | * | < | > | >= | <= | =

<fname>      := [a-zA-z][a-zA-Z0-9]*
<identifier> := [a-zA-z][a-zA-Z0-9]*
<binding>    := (<identifier> <expr>)
```

Note that integers must be within the bounds $-2^{62}$ to $2^{62} - 1$. `input` refers to an optional
argument provided at runtime of the Snek binary.

## Abstract Syntax

The abstract syntax of Snek, parsed out of a .snek file and then compiled into instructions, is shown below. 

```
enum Op1 { Add1, Sub1, IsNum, IsBool, Print, }

enum Op2 { Plus, Minus, Times, Equal, Gt, Gte, Lt, Lte, }

enum Expr {
    Number(i64),
    Boolean(bool),
    Tuple(Vec<Expr>),
    Id(String),
    Let(Vec<(String, Expr)>, Box<Expr>),
    UnOp(Op1, Box<Expr>),
    BinOp(Op2, Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Loop(Box<Expr>),
    Break(Box<Expr>),
    Set(String, Box<Expr>),
    TSet(Box<Expr>, Box<Expr>, Box<Expr>),
    TGet(Box<Expr>, Box<Expr>),
    Block(Vec<Expr>),
    Call(String, Vec<Expr>),
}

struct Function {
    name : String,
    args : Vec<String>,
    body : Expr,
}

struct Program {
    defns : Vec<Function>,
    main : Expr,
}
```

## Value Representations

Values, such as integers, booleans, etc., are represented in the Snek runtime environment with two parts: a code and a tag. The tag is on the less significant part of the byte (includes LSB). The value representations are as follows. Note that the code reflects a decimal representation of the actual binary code part.

Value        | Tag Size | Code | Tag
-------------|:--------:|:----:|------:
integer      | 1 bit    |  n   | 0
tuple        | 2 bits   | addr | 01
true         | 2 bits   |  1   | 11
false        | 2 bits   |  0   | 11