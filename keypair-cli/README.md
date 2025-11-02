# mini-keypair-cli
This is my first time making a CLI with clap so it's super simple and probably not following any best practices.

<p>&nbsp;</p>

## Usage

### Compile and run or just run from cargo
```bash
cargo build -p mini-keypair-cli --release && cp ./target/release/mini-keypair-cli .
```
or
```bash
cargo run -p mini-keypair-cli -- [args]
```

<p>&nbsp;</p>

### Command list
```bash
cargo run -p mini-keypair-cli -- -help
```
or with the compiled binary
```bash
./mini-keypair-cli --help
```

<p>&nbsp;</p>

### Save a keypair
```bash
cargo run -p mini-keypair-cli -- -s "./keypair.json"
```
or with the compiled binary

```bash
./mini-keypair-cli -s "./keypair.json"
```

### Load a keypair
```bash
cargo run -p mini-keypair-cli -- -l "./keypair.json"
```
or with the compiled binary

```bash
./mini-keypair-cli -l "./keypair.json"
```

<p>&nbsp;</p>

# Signing messages

### Signing a message with a generated keypair
```bash
cargo run -p mini-keypair-cli -- -m "message"
```
or with the compiled binary

```bash
./mini-keypair-cli -m "message"
```

> Output example: `message Signature: 341503995ba87f016dcbe7a304f05a3ba75481255cfabfcd198ba4aaaa484545`

<p>&nbsp;</p>

### Signing a message with a loaded keypair
```bash
cargo run -p mini-keypair-cli -- -l "./keypair.json" -m "message"
```
or with the compiled binary

```bash
./mini-keypair-cli -l "./keypair.json" -m "message"
```

> Output example: `message Signature: 66ccf3e9105697a832ea79098566c12572a01e46983b12fba211ddf9e4131c03`

<p>&nbsp;</p>

# Message and signature verification.

### Verifying a message with a generated keypair
```bash
cargo run -p mini-keypair-cli -- -m "message" --verify-signature "341503995ba87f016dcbe7a304f05a3ba75481255cfabfcd198ba4aaaa484545"
```
or with the compiled binary

```bash
./mini-keypair-cli -m "message" --verify-signature "341503995ba87f016dcbe7a304f05a3ba75481255cfabfcd198ba4aaaa484545"
```

> Output example: `message Signature: 341503995ba87f016dcbe7a304f05a3ba75481255cfabfcd198ba4aaaa484545, signed by the keypair? false`

<p>&nbsp;</p>

### Verifying a message with a loaded keypair
```bash
cargo run -p mini-keypair-cli -- -l "./keypair.json" -m "message" --verify-signature "66ccf3e9105697a832ea79098566c12572a01e46983b12fba211ddf9e4131c03"
```
or with the compiled binary

```bash
./mini-keypair-cli -l "./keypair.json" -m "message" --verify-signature "66ccf3e9105697a832ea79098566c12572a01e46983b12fba211ddf9e4131c03"
```

> Output example: `message Signature: 66ccf3e9105697a832ea79098566c12572a01e46983b12fba211ddf9e4131c03, signed by the keypair? true`
