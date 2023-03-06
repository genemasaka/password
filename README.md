# password

This Rust code is an example of how to interact with a smart contract on the Ethereum blockchain using the ethers library.

The code defines a Cryptic contract instance by generating Rust bindings from the smart contract's ABI file using the abigen! macro. 
It then creates a Client type using SignerMiddleware with an Http provider and a k256 ECDSA signing key.

The main() function creates a provider instance for the Goerli testnet, creates a local wallet from a private key, and sets the chain ID to Goerli.
It then creates a client with the provider and local wallet, defines the contract address, and creates a Cryptic contract instance with the client.

The program then calls the encrypt_password function of the contract to encrypt a password and waits for transaction confirmation. It then calls the 
get_encrypted function of the contract to retrieve the encrypted password and prints the encrypted password and transaction receipt as a JSON string.

To run the program, ensure you have Rust installed, copy the code into your project directory, and run the command cargo run.
