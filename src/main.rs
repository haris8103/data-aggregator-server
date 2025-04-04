use actix_web::{web, App, HttpResponse, HttpServer, Responder, web::Query, get};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use serde::{Serialize, Deserialize};
use tokio_postgres::NoTls;
use tokio::task;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;

#[derive(Serialize)]
struct Transaction {
    trans_hash: String,
    sender: String,
    reciever: String,
    amount: i64,
    time: Option<i64>,
}

type DbPool = Pool<PostgresConnectionManager<NoTls>>;

#[derive(Deserialize)]
struct TransactionFilters {
    trans_hash: Option<String>,
    sender: Option<String>,
    receiver: Option<String>,
    time: Option<i64>,
}


#[get("/account/{account_id}")] // <- define path parameters
async fn get_account(path: web::Path<String>) -> impl Responder {
    let account_id = path.into_inner();
    
    // Offload the blocking operation to a separate thread
    let account = task::spawn_blocking(move || {
        let rpc = RpcClient::new("https://api.devnet.solana.com".to_string());
        let pubkey = Pubkey::from_str_const(&account_id);
        rpc.get_account(&pubkey)
    })
    .await
    .unwrap(); // Handle result here, ensure it's not an error

    match account {
        Ok(account) => HttpResponse::Ok().json(account),
        Err(_) => HttpResponse::InternalServerError().body("Failed to get account data"),
    }
}

async fn get_transactions(pool: web::Data<DbPool>, query: Query<TransactionFilters>) -> impl Responder {
    let client = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("DB pool error"),
    };

    let mut sql = String::from("SELECT trans_hash, sender, reciever, amount, time FROM data_aggregator WHERE true");
    let mut params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::new();
    let trans_hash_opt = &query.trans_hash;
    let sender_option =&query.sender;
    let recier_opt = &query.receiver;
    let time_opt = &query.time;
    let mut count = 1;
    if let Some(trans_hash) = trans_hash_opt {
        sql.push_str(&format!(" AND trans_hash = ${}", count));
        params.push(trans_hash);
        count+=1;
    }

    if let Some(sender) = sender_option {
        sql.push_str(&format!(" AND sender = ${}",count));
        params.push(sender);
        count+=1;
    }

    if let Some(receiver) = recier_opt {
        sql.push_str(&format!(" AND reciever = ${}",count));
        params.push(receiver);
        count+=1;
    }

    if let Some(time) = time_opt {
        sql.push_str(&format!(" AND time = ${}", count));
        params.push(time);
        
    }

    // Run the query
    let rows = match client.query(&sql, &params).await {
        Ok(r) => r,
        Err(_) => return HttpResponse::InternalServerError().body("Query failed"),
    };

    // Map the results into a vector of Transaction structs
    let transactions: Vec<Transaction> = rows.iter().map(|row| Transaction {
        trans_hash: row.get("trans_hash"),
        sender: row.get("sender"),
        reciever: row.get("reciever"),
        amount: row.get("amount"),
        time: row.get("time"),
    }).collect();

    // Return the results as JSON
    HttpResponse::Ok().json(transactions)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pg_config = "host=localhost user=postgres dbname=mydb password=12345678"
        .parse()
        .expect("❌ Failed to parse DB config");

    let manager = PostgresConnectionManager::new(pg_config, NoTls);
    let pool = Pool::builder()
        .build(manager)
        .await
        .expect("❌ Failed to build DB pool");

    println!("🚀 Server running at http://localhost:3251");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(self::get_account)
            .route("/transactions", web::get().to(get_transactions))
            
    })
    .bind("127.0.0.1:3251")?
    .run()
    .await
}
