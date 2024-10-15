# Calium: REPL Lexer

## [!] Disclaimer
This project is currently unstable and may break
in the next version, so is up to you.

## Core types
Here are list of types, used in this project

1. `Scanner<'a>`
  This type is built on top of the `Chars<'a>` type.
  it also implements the trait `Deref<Target = Chars<'a>`
  and `DerefMut` as well.

2. `TokenKind`
  This type is an `enum` that represent a token for
  a single symbols.
  The token maybe a little bit messy when it comes to
  scan an integer, represented by `Number(u128)`.

  ### Variants
  1. `Number(u128)`, represent decimal number
  2. `Add`, represent _addition_ `(+)`
  3. `Sub`, represent _substraction_ `(-)`
  4. `Mul`, represent _multiplication_ `(*)`
  5. `Div`, represent _division_ `(/)`
  6. `Rem`, represent _remainder_ `(%)`
  7. `Pow`, represent _power_ `(^)`
  8. `EOL`, end of line `(;)`

3. `SyntaxError`
  This type act as an error reporter for the user.
  This type encapsulate crucial data like where the error
  occures like the line of code, or the column.
  This type implement `Display`, just like the `Error` from the
  standard library.
