# file-server
Static HTTP file server for web developers.

## Installing

Make sure you first [install Cargo](https://doc.rust-lang.org/cargo/), then run the following command in your terminal:
```
cargo install file-server
```

## Basic Usage

To start the server, run the following command in your terminal at the root directory of your project (your `index.html` file will likely be there):

```
file-server -p <PORT>
```

For example:
```
file-server -p 8080
```
