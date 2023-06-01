# encode-solana

## Public Keys
Mine - `CcPDbb1KyuTMiCWqSS4HENL81pqQjTYvnfgjnbgfZHX`
Austin - `626hG9Hh8YGJ6R7AtFSj2bdiY4ZNErYjvjPGDpvKzzQM`
Bogo - `CZvMwuRbcmRJasFxpXAyqFDN42RHZzu8smfJHFM2zrCH`
Eduardo - `HF9ijP2BHb6JBW6j67qs2iQwiaeZV3QgWtJ25jUJPw1L`

## transfering tokens
pubkey: CcPDbb1KyuTMiCWqSS4HENL81pqQjTYvnfgjnbgfZHX

solana airdrop 1 CcPDbb1KyuTMiCWqSS4HENL81pqQjTYvnfgjnbgfZHX  --url https://api.devnet.solana.com

solana balance CcPDbb1KyuTMiCWqSS4HENL81pqQjTYvnfgjnbgfZHX --url https://api.devnet.solana.com

solana transfer --from /home/codespace/.config/solana/id.json HF9ijP2BHb6JBW6j67qs2iQwiaeZV3QgWtJ25jUJPw1L 0.5 --allow-unfunded-recipient --url https://api.devnet.solana.com --fee-payer /home/codespace/.config/solana/id.json

solana transfer --from /home/codespace/.config/solana/id.json HF9ijP2BHb6JBW6j67qs2iQwiaeZV3QgWtJ25jUJPw1L 0.5 --allow-unfunded-recipient --url https://api.devnet.solana.com --fee-payer /home/codespace/.config/solana/id.json

## creating and sending fungible token
`spl-token create-token`
```
Creating token 4LQLkdCjNBfrtdG7Uhf8kk1vBK8et1KY34bgt1vaiAER under program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA

Address:  4LQLkdCjNBfrtdG7Uhf8kk1vBK8et1KY34bgt1vaiAER
Decimals:  9

Signature: 5mg5Pnh8zHR4t9soZkoUzAV4qtaBJZHNBW94B1L7yQ8NWziRN4Gw93kqRPNtiEXW4kTwiKYe1QiKdX9SWJsqjhuN
```

`spl-token create-account 4LQLkdCjNBfrtdG7Uhf8kk1vBK8et1KY34bgt1vaiAER`
```
Creating account 6jPPKM7ACdFiuPkNtkW7MQUeZWEjRBVdMup9Pq9nN8SL

Signature: 5orReFWU3YwvebacJAzTe3FnLN5wX3kPuFBfxjXi3b4NuXFwV6XdRFj8WA8PJC58bVef8KuKHVF4AVDEVoxhjVvj
```

`spl-token supply 4LQLkdCjNBfrtdG7Uhf8kk1vBK8et1KY34bgt1vaiAER`
```
0
```

`spl-token mint 4LQLkdCjNBfrtdG7Uhf8kk1vBK8et1KY34bgt1vaiAER 10000`
```
Minting 10000 tokens
  Token: 4LQLkdCjNBfrtdG7Uhf8kk1vBK8et1KY34bgt1vaiAER
  Recipient: 6jPPKM7ACdFiuPkNtkW7MQUeZWEjRBVdMup9Pq9nN8SL

Signature: 3vMKNrNJkeBkWjPiEMv825KCHh1fJuSpUPMzH1RE8mEdden83w4o8rAHebTc772cPWKVo9rVVTgCavWebHkhoRqy
```

`spl-token accounts`
`spl-token balance 4LQLkdCjNBfrtdG7Uhf8kk1vBK8et1KY34bgt1vaiAER`

`spl-token transfer --fund-recipient <Token ID> <amount> <destination>`
`spl-token transfer --fund-recipient 4LQLkdCjNBfrtdG7Uhf8kk1vBK8et1KY34bgt1vaiAER 100 HF9ijP2BHb6JBW6j67qs2iQwiaeZV3QgWtJ25jUJPw1L`
```
Transfer 100 tokens
  Sender: 6jPPKM7ACdFiuPkNtkW7MQUeZWEjRBVdMup9Pq9nN8SL
  Recipient: HF9ijP2BHb6JBW6j67qs2iQwiaeZV3QgWtJ25jUJPw1L
  Recipient associated token account: 7pwZkocUgbXX1ojTuTXjA3ahEjm6YJN6f6B7HJ5v3Rn9
  Funding recipient: 7pwZkocUgbXX1ojTuTXjA3ahEjm6YJN6f6B7HJ5v3Rn9

Signature: 558yX646jyo3o8BcEqh722wjRAN8bNxkgqzUows53Q7Qn8Ea6suo8yAFv3pLcJvKwYpiLiPCHWJPMVHKGJbh21ni
```

`spl-token transfer <Token ID> <amount> <destination>`
`spl-token transfer 4LQLkdCjNBfrtdG7Uhf8kk1vBK8et1KY34bgt1vaiAER 100 HF9ijP2BHb6JBW6j67qs2iQwiaeZV3QgWtJ25jUJPw1L`
```
Transfer 100 tokens
  Sender: 6jPPKM7ACdFiuPkNtkW7MQUeZWEjRBVdMup9Pq9nN8SL
  Recipient: HF9ijP2BHb6JBW6j67qs2iQwiaeZV3QgWtJ25jUJPw1L
  Recipient associated token account: 7pwZkocUgbXX1ojTuTXjA3ahEjm6YJN6f6B7HJ5v3Rn9

Signature: cfH6eSzVmKzuNSmneKWGZbB7F5ExzSVns7Wm2JDeb5SSNQdjHH34udE56tiU1cFRCVfEsrKgiACjYRkB6pBw7Lt
```