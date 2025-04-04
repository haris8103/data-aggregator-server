Solana Transaction and Account API

This project provides a RESTful API that connects to the Solana Devnet using the Solana RPC client. It retrieves information about Solana accounts and transactions, offering two main endpoints:

    Get account details by account ID.

    Query transactions from a PostgreSQL database with filters for transaction hash, sender, receiver, and timestamp.

Features:

    Fetch account data from Solana Devnet.

    Query transaction data from PostgreSQL with various filters.

    Handle blocking operations offloaded to a separate thread to avoid blocking the async runtime.

    Multi-threading support for efficient query handling.

Requirements
1. Solana Devnet

    The application fetches account and transaction data from the Solana Devnet.

    Ensure you have access to Solana Devnet or change the RPC endpoint to use other networks like Testnet or Mainnet.

2. PostgreSQL Database

    A PostgreSQL database is required to store transaction data. You can set up a local instance or use a hosted database.

    You must create a database (mydb) and a data_aggregator table as follows:

    CREATE TABLE public.data_aggregator (
        trans_hash TEXT,
        sender TEXT,
        reciever TEXT,
        amount BIGINT,
        time BIGINT
    );

3. Dependencies

Make sure your Cargo.toml contains the following dependencies:

[dependencies]
actix-web = "4"
bb8 = "0.8"
bb8-postgres = "0.8"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1", features = ["full"] }
tokio-postgres = { version = "0.7", features = ["with-chrono"] }
solana-client = "2.2.6"
solana-sdk = "1.9.10"

How to Run
1. Set Up PostgreSQL

Install PostgreSQL if it's not already installed on your machine. You can follow the instructions on the official website.

After installation, create the mydb database and the data_aggregator table using the provided SQL script.

CREATE DATABASE mydb;

2. Run the Application

Clone this repository and run the application with the following commands:

cargo run

3. Access the API

Once the application is running, it will be available at http://localhost:3251.
API Endpoints

    Get Account Info:
    GET /account/{account_id}
    Fetches account details from Solana Devnet.

    Example:

GET http://localhost:3251/account/HUzaTjk2sQopRURDu7Uio7GF5DXV49dAqFxaxcWChvWC

Get Transactions:
GET /transactions?trans_hash={trans_hash}&sender={sender}&receiver={receiver}&time={timestamp}
Fetches transaction details from the PostgreSQL database. All query parameters are optional. You can filter by transaction hash, sender, receiver, and time.

Example:

    GET http://localhost:3251/transactions?sender=some_sender_address&receiver=some_receiver_address

Code Explanation
1. Solana Account Fetching

The /account/{account_id} endpoint retrieves the account data for a given account ID by querying the Solana Devnet using the solana_client::rpc_client::RpcClient. The request is offloaded to a separate thread using task::spawn_blocking to ensure that it doesn't block the Actix async runtime.
2. Transaction Querying

The /transactions endpoint queries the data_aggregator table in PostgreSQL. You can filter the results by the following query parameters:

    trans_hash: Transaction hash.

    sender: Sender's account address.

    receiver: Receiver's account address.

    time: Transaction timestamp.

3. Database Connection Pooling

    The application uses bb8 and bb8-postgres for database connection pooling to efficiently handle database connections.

    The connection pool is initialized and passed to the routes to access the database.

4. Multi-threading

    The application spawns separate threads to handle the transaction processing for each request to ensure that multiple transactions can be processed concurrently without blocking the main Actix runtime.
