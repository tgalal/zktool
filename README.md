## ZKTool

A tool to verify zkmail proofs from command line

### Build

```
cargo build
```

### Test

Run from project root, this will use test data under `fixtures/`, obtained from [email-as-ens](https://github.com/zkemail/email-as-ens/)

```
cargo test
```

### Usage

Basic Proof Verification

```
Usage: zktool verify --verification-key <VERIFICATION_KEY> --proof <PROOF> --inp <INP>
```

Verify Claim ENS Proof

```
zktool claim --verification-key <VERIFICATION_KEY> --proof <PROOF> --inp <INP> --dkim-pk <DKIM_PK> --address <ADDRESS> --resolver <RESOLVER> <EMAIL>
```

Verify Raw Command

```
Usage: zktool command --verification-key <VERIFICATION_KEY> --proof <PROOF> --inp <INP> --dkim-pk <DKIM_PK> --email <EMAIL> <COMMAND>
```

### Examples

Basic Proof Verification

```
zktool verify -v fixtures/vkey.json -i fixtures/public.json -p fixtures/proof.json
```

Verify Claim ENS Proof

```
zktool claim -v fixtures/vkey.json \
             -i fixtures/public.json \
             -p fixtures/proof.json \
             -d 0ea9c777dc7110e5a9e89b13f0cfc540e3845ba120b2b6dc24024d61488d4788 \
             --address 0xafBD210c60dD651892a61804A989eEF7bD63CBA0 \
             --resolver resolver.eth \
             thezdev1@gmail.com 
```


Verify Proof for a Raw Command

```
zktool command -v fixtures/vkey.json \
             -i fixtures/public.json \
             -p fixtures/proof.json \
             -d 0ea9c777dc7110e5a9e89b13f0cfc540e3845ba120b2b6dc24024d61488d4788 \
             -e thezdev1@gmail.com \
             "Claim ENS name for address 0xafBD210c60dD651892a61804A989eEF7bD63CBA0 with resolver resolver.eth"
```
