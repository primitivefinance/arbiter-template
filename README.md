# arbiter-template

Minimal template for simulating contracts with arbiter. 

## Usage

1. Clone this repository

```
git clone https://github.com/primitivefinance/arbiter-template.git
cd arbiter-template
```

2. Install foundry

```
curl -L https://foundry.paradigm.xyz | bash
foundryup
```

3. Install forge libraries

```
forge install
```

4. Generate bindings

```
forge bind --revert-strings debug -b src/bindings/ --module --overwrite
```

5. Run the project

```
cargo run
```
