# rust marco polo game
A Marco Polo game.

Accepts a string with a name.

If the name is "Marco", returns "Polo".

If the name is "any other value", it returns "not Polo".

An example of a tiny Rust command-line tool in a Docker container.
About 83.8MB in size.

## Usage

To run locally
```bash
cargo run play --name "Marco"
OR
./target/debug/marco_polo play --name "Marco"
```

To build the container and run the application:
```bash
docker build -t marco-polo .
docker run --rm -it marco-polo --help 
```

You can see full invocation here:

```bash
➜  marco_polo git:(main) ✗ docker run --rm -it marco-polo play --name Marco
Polo
```

To find the size of the application:

```bash
docker image ls | grep marco-polo
```

```bash
marco-polo                                                                   latest                                                                       7da8a3444176   15 minutes ago   83.8MB
```

## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
* [Simplify Rust Deploy](https://www.docker.com/blog/simplify-your-deployments-using-the-rust-official-image/)