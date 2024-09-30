mod cid;
mod utils;
mod types;
mod cover;
mod pool;

use std::time::Duration;

use cover::get_covers;
use pool::get_pools;
use tokio::time::sleep;
use utils::{cover_set, create_cover, create_pool, pool_active, pool_set};

#[tokio::main]
async fn main() {

    cover_set().await;
    pool_set().await;
    pool_active().await;
    let pools = get_pools();
    for pool in pools {
        create_pool(pool.clone()).await;
        println!("Pool {:?} created", pool.name);

        sleep(Duration::from_secs(10)).await;
    }

    // sleep(Duration::from_secs(20)).await;

    let covers = get_covers();
    // for cover in covers {
    //     create_cover(cover.clone()).await;
    //     println!("Cover {:?} created", cover.name);

    //     sleep(Duration::from_secs(10)).await;
    // }
    println!("Length {:?}", covers.len())

}