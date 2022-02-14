use warp::Filter;

#[tokio::main]
async fn main() {
    let filter = warp::path!("hello" / String)
    .map(|name| format!("hello {}", name)); 

    println!("Entró al servidor en el puerto 8080");

   warp::serve(filter)
   .run(([127, 0, 0, 1], 8080))
    .await;
}
