use chrono::{DateTime, Duration, FixedOffset, Local, TimeZone, Timelike, Utc};
use chrono_tz::Asia::Shanghai;
use chrono_tz::Etc::UTC;

fn main() {
    let utc: DateTime<Utc> = Utc::now();
    let local: DateTime<Local> = Local::now();
    println!("Utc now {}", utc);
    println!("timestamp {}", Utc::now().timestamp());
    println!(
        "datetime from timestamp {}",
        Shanghai.timestamp_opt(1683275206, 0).unwrap()
    );
    //转换时区
    let tz = FixedOffset::east_opt(8 * 3600).unwrap();
    println!(
        "utc-> FixedOffset{}",
        utc.with_timezone(&tz).format("%Y-%m-%d %H:%M:%S")
    );
    println!(
        "utc-> Shanghai{}",
        utc.with_timezone(&Shanghai).format("%Y-%m-%d %H:%M:%S")
    );
    // 替换时区
    println!(
        "replace tz {}",
        local.naive_local().and_local_timezone(UTC).unwrap()
    );
    //修改日期--指定时间
    let local1 = local.with_hour(5).unwrap();
    println!("replace hour {}", local1);
    let dt1 = Utc.with_ymd_and_hms(2013, 11, 14, 8, 9, 10).unwrap();
    let dt2 = Utc.with_ymd_and_hms(2014, 1, 14, 10, 9, 8).unwrap();
    //修改日期--增量
    println!(
        "add Duration {}",
        dt1.checked_add_signed(Duration::try_days(1).unwrap())
            .unwrap()
    );
    println!("add Duration {}", dt1 + Duration::try_days(1).unwrap());
    // 遍历某段时间
    let mut dt = dt1;
    while dt < dt2 {
        println!("{}", dt);
        dt += Duration::try_days(1).unwrap();
    }
}
