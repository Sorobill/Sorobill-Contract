# Admin role

The address passed to `initialize` is the only caller allowed to invoke `execute_billing`.

In production this should be the Substrata-Backend treasury keypair (or a multisig controlling it).
