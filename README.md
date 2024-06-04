# Banking System with Rust 
- A simple cli Banking system where you can perform banking operations like check balance, send money etc.
for you yo run this project first build it.

```sh {"id":"01HZH7ZTSGR3MP1PE331SZ4M3H"}
cargo build

```

# Various operations

```sh {"id":"01HZH84R6EK9S88YK01AHRC58M"}
// To create an account-Provides Account number and Pin
cargo run create
```

```sh {"id":"01HZH883VZFH0XVYVYP6Z9ESVV"}
// To login you need your account number and pin- if you run this an error will be thrown cause you need the account number and pin
cargo run login AccountNumber Pin
```

When you login you can do various operations there, it would be shown and you need your pin to complete each operation

```sh {"id":"01HZH8EDHHPX8XRVSTF5PF6WZ6"}
// Don't need the account again? you can delete the account by writing
cargo run delete AccountNumber
```