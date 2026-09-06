use std::error::Error;

use byteorder::{BigEndian, ReadBytesExt};
use bytes::{Buf, BufMut, Bytes, BytesMut};
use chrono::Local;
use log::info;

use logging::init_logger;

fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    init_logger();
    let now = Local::now();
    let n = 35_usize;

    //  std:

    // num -> &[u8]
    let n_bytes = n.to_be_bytes();
    // &[u8] -> num
    let new_n = usize::from_be_bytes(n_bytes);
    info!("new_n={}, n_bytes={:?}", new_n, n_bytes);

    // bytes/byteorder crate:

    // num -> &[u8]
    let mut buf = BytesMut::with_capacity(16);
    buf.put_u64(n as u64);
    // &[u8] -> num
    // buf 是 BytesMut，as_ref() 返回借用的 &[u8]，不复制字节。
    // buf.get_u64()
    // 会推进 buf 自己的读取位置。这里只有 8 字节，读取后 buf 就空了。
    // buf.as_ref().get_u64()
    // 消耗的是临时切片视图，原来的 buf 不变
    let new_n = buf.as_ref().get_u64();
    info!("new_n={}, n_bytes={:?}", new_n, buf);
    // &[u8] -> num with byteorder
    let new_n = buf.as_ref().read_u64::<BigEndian>()?;
    let new_n2 = buf.as_ref().get_u64();
    info!("new_n={}, new_n2={}, n_bytes={:?}", new_n, new_n2, buf);

    // std

    // str -> &[u8]
    let s = "aaa春风吹又生".as_bytes();
    // &[u8] -> str
    let new_s = String::from_utf8(Vec::from(s))?;
    info!("new_s={}", new_s);

    let str_bytes_a = Bytes::from("aaa");
    let str_bytes_b = Bytes::from("春风吹又生");
    let mut bytes_c = BytesMut::with_capacity(1024);
    bytes_c.put(str_bytes_a);
    bytes_c.put(str_bytes_b);
    let new_s = String::from_utf8(bytes_c.to_vec())?;
    info!("new_s={}", new_s);

    info!(
        "run time: {}s",
        Local::now().signed_duration_since(now).num_seconds()
    );
    Ok(())
}

#[test]
fn byte_conversion_example_completes() {
    main().unwrap();
}
