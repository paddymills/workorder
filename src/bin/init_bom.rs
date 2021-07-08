
use std::error::Error;

use tiberius::Client;
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;

use workorder::db::{self, queries, Part};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    
    let config = db::config::eng()?;

    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    // Handling TLS, login and other details related to the SQL Server.
    let mut client = Client::connect(config, tcp.compat_write()).await?;
    
    let job = "1200055C";
    let ship: i32 = 3;
    let stream = client.query(queries::GET_BOM, &[&job, &ship]).await?;

    let res = stream.into_first_result().await?;

    let mut rows = Vec::<Part>::new();
    for row in res {

        rows.push(Part::from_sql(&row));
    }

    let mut i = 0;
    for row in rows.iter().filter(|x| x.is_pl()) {
        println!("{:}", row);

        i += 1;
        if i > 5 {
            break;
        }
    }

    Ok(())
}
