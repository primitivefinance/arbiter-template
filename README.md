# arbiter-template
Minimal template for simulating contracts with arbiter. 

## Usage

1. Clone this repository

```
git clone https://github.com/primitivefinance/arbiter-template.git
cd arbiter-template
```

2. Install foundry and forge
```
curl -L https://foundry.paradigm.xyz | bash
foundryup
forge install
```
3. Generate bindings

```
forge bind --revert-strings debug -b src/bindings/ --module --overwrite
```

4. Run the project

```
cargo run
```
