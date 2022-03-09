use etcd_rs::*;

#[macro_use]
mod support;
use crate::support::{Context, KVOp};

#[tokio::test]
async fn test_watch() {
    let ctx = Context::new(false);
    let cli = ctx.connect_to_cluster().await;

    const PREFIX: &str = "prefix-test-watch";

    let (mut stream, _cancel) = cli
        .watch(KeyRange::prefix(PREFIX))
        .await
        .expect("watch created");

    let ops: Vec<_> = vec![
        KVOp::Put("foo1".to_owned(), "bar1".to_owned()),
        KVOp::Put("foo2".to_owned(), "bar2".to_owned()),
        KVOp::Put("foo3".to_owned(), "bar3".to_owned()),
        KVOp::Delete("foo1".to_owned()),
        KVOp::Delete("foo2".to_owned()),
    ]
    .into_iter()
    .map(|op| match op {
        KVOp::Put(k, v) => KVOp::Put(format!("{}-{}", PREFIX, k), v),
        KVOp::Delete(k) => KVOp::Delete(format!("{}-{}", PREFIX, k)),
    })
    .collect();

    apply_kv_ops!(cli, ops);

    assert_ops_events!(ops, stream);
}

#[tokio::test]
async fn test_watch_multi() {
    let ctx = Context::new(false);
    let cli = ctx.connect_to_cluster().await;

    const PREFIX1: &str = "prefix-test-watch-multi1";
    const PREFIX2: &str = "prefix-test-watch-multi2";

    let (mut stream1_1, _cancel) = cli
        .watch(KeyRange::prefix(PREFIX1))
        .await
        .expect("watch created");
    let (mut stream2, _cancel) = cli
        .watch(KeyRange::prefix(PREFIX2))
        .await
        .expect("watch created");

    let ops_1: Vec<_> = vec![
        KVOp::Put("foo1".to_owned(), "bar1".to_owned()),
        KVOp::Put("foo2".to_owned(), "bar2".to_owned()),
        KVOp::Put("foo1".to_owned(), "bar3".to_owned()),
        KVOp::Delete("foo1".to_owned()),
        KVOp::Delete("foo2".to_owned()),
    ]
    .into_iter()
    .map(|op| match op {
        KVOp::Put(k, v) => KVOp::Put(format!("{}-{}", PREFIX1, k), v),
        KVOp::Delete(k) => KVOp::Delete(format!("{}-{}", PREFIX1, k)),
    })
    .collect();

    let ops_2: Vec<_> = vec![
        KVOp::Put("foo1".to_owned(), "bar1".to_owned()),
        KVOp::Put("foo2".to_owned(), "bar2".to_owned()),
        KVOp::Put("foo3".to_owned(), "bar3".to_owned()),
        KVOp::Put("foo4".to_owned(), "bar3".to_owned()),
        KVOp::Delete("foo1".to_owned()),
        KVOp::Delete("foo2".to_owned()),
    ]
    .into_iter()
    .map(|op| match op {
        KVOp::Put(k, v) => KVOp::Put(format!("{}-{}", PREFIX2, k), v),
        KVOp::Delete(k) => KVOp::Delete(format!("{}-{}", PREFIX2, k)),
    })
    .collect();

    apply_kv_ops!(cli, ops_1);
    apply_kv_ops!(cli, ops_2);

    assert_ops_events!(ops_1, stream1_1);
    assert_ops_events!(ops_2, stream2);

    let (mut stream1_2, _cancel) = cli
        .watch(KeyRange::prefix(PREFIX1))
        .await
        .expect("watch created");

    let ops_1: Vec<_> = vec![
        KVOp::Put("foo4".to_owned(), "bar4".to_owned()),
        KVOp::Put("foo5".to_owned(), "bar5".to_owned()),
        KVOp::Put("foo6".to_owned(), "bar6".to_owned()),
        KVOp::Delete("foo4".to_owned()),
        KVOp::Delete("foo5".to_owned()),
    ]
    .into_iter()
    .map(|op| match op {
        KVOp::Put(k, v) => KVOp::Put(format!("{}-{}", PREFIX1, k), v),
        KVOp::Delete(k) => KVOp::Delete(format!("{}-{}", PREFIX1, k)),
    })
    .collect();

    apply_kv_ops!(cli, ops_1);

    assert_ops_events!(ops_1, stream1_1);
    assert_ops_events!(ops_1, stream1_2);
}
