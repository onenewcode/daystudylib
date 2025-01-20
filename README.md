# 概述
本仓库是本人体验各种 rust 库的代码，会不定期跟新 rust 各种有趣的库。
# 生成随机值
rust 中官方并没有像以他语言一样，rust 并没有官方并没有提供生成随机数的工具，所以我们要借助 rand 包进行生成随机数。这里我们使用现在使用罪为广泛的 rand 包只需要引入以下依赖就能够使用。`rand = "0.8.5"`

## 生成随机数


在随机数生成器 rand::Rng 的帮助下，通过 rand::thread_rng 生成随机数。可以开启多个线程，每个线程都有一个初始化的生成器。整数在其类型范围内均匀分布，浮点数是从 0 均匀分布到 1，但不包括 1。

```rs
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    // 改变类型
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f64>());
}
```
**结果**
```shell
Random u8: 247
Random u16: 46458
Random u32: 2649532043
Random i32: 1393744920
Random float: 0.5923489382636902
```
## 生成范围内随机数

使用 Rng::gen_range，在半开放的 [0, 10) 范围内（不包括 10）生成一个随机值。

```rs
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    println!("Integer: {}", rng.gen_range(0..10));
    println!("Float: {}", rng.gen_range(0.0..10.0));
}
```
**结果**
```shell
   let mut rng = rand::thread_rng();
    println!("Integer: {}", rng.gen_range(0..10));
    println!("Float: {}", rng.gen_range(0.0..10.0));
```

使用 Uniform 模块可以得到均匀分布的值。下述代码和上述代码具有相同的效果，但在相同范围内重复生成数字时，下述代码性能可能会更好。
```rs

use rand::distributions::{Distribution, Uniform};

fn main() {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..7);

    loop {
        let throw = die.sample(&mut rng);
        println!("Roll the die: {}", throw);
        if throw == 6 {
            break;
        }
    }
}
```
**结果**
```shell
Roll the die: 1
Roll the die: 2
Roll the die: 6
```

## 生成自定义类型随机值


随机生成一个元组 (i32, bool, f64) 和用户定义类型为 Point 的变量。为 Standard 实现 Distribution trait，以允许随机生成。
```rs
use rand::Rng;
use rand::distributions::{Distribution, Standard};

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (rand_x, rand_y) = rng.gen();
        Point {
            x: rand_x,
            y: rand_y,
        }
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let rand_tuple = rng.gen::<(i32, bool, f64)>();
    let rand_point: Point = rng.gen();
    println!("Random tuple: {:?}", rand_tuple);
    println!("Random Point: {:?}", rand_point);
}
```
**结果**
```shell
Random tuple: (590118681, false, 0.7548409339548463)
Random Point: Point { x: 914499268, y: 795986012 }
```
## 从一组字母数字字符创建随机密码

随机生成一个给定长度的 ASCII 字符串，范围为 A-Z，a-z，0-9，使用字母数字样本。

```rs
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

fn main() {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    println!("{}", rand_string);
}
```
**结果**
```shell
fwaZUzdIkK1p78fyNvh44Od5gcr3BL
```
## 从一组用户定义字符创建随机密码


使用用户自定义的字节字符串，使用 gen_range 函数，随机生成一个给定长度的 ASCII 字符串。
```rs
use rand::Rng;
fn main() {
   
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    const PASSWORD_LEN: usize = 30;
    let mut rng = rand::thread_rng();

    let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    println!("{:?}", password);
}
```
**结果**
```shell
"F@QNgOrsviJ2tqM$zOSJSR^Hjevvce"
```

# 日期库chrono
Rust的时间操作主要用到chrono库，接下来我将简单选一些常用的操作进行介绍，如果想了解更多细节，请查看官方文档。

```rs
use chrono::{Datelike, Duration, Local, TimeZone, Timelike};
 
fn main() {
    let fmt = "%Y年%m月%d日 %H:%M:%S";
    let now = Local::now().format(fmt);
    println!("{}", now);
 
    let mut parse = Local
        .datetime_from_str("2022 年 3 月 19 日 13:30:59", fmt)
        .unwrap();
    println!("{:?}", parse);
    println!(
        "{}-{}-{} {}:{}:{}",
        parse.year(),
        parse.month(),
        parse.day(),
        parse.hour(),
        parse.minute(),
        parse.second()
    );
    println!("{}", parse.date());
    parse = Local.ymd(2012, 12, 12).and_hms(12, 12, 12);
    println!("{}", parse);
    parse = parse + Duration::days(2);
    println!("{}", parse);
    parse = parse + Duration::hours(2);
    println!("{}", parse);
    parse = parse + Duration::minutes(2);
    println!("{}", parse);
    parse = parse + Duration::seconds(2);
    println!("{}", parse);
 
}
```


## 测量运行时间
计算从 time::Instant::now 开始运行的时间 time::Instant::elapsed。

调用 time::Instant::elapsed 将返回 time::Duration，我们将在实例末尾打印该时间。此方法不会更改或者重置 time::Instant 对象。

```rs
use std::time::Instant;
use std::{thread, time};
fn main(){
    let start = Instant::now();
    // 设置休眠时间，1s，
    let ten_millis = time::Duration::from_millis(1);
    thread::sleep(ten_millis);
    let duration = start.elapsed();
    //计算获得的时间会超过 1s，因为系统运行也会消耗一定时间
    println!("显示两行代码之间消耗的时间 () is: {:?}", duration);
}
```
**结果**
```shell
显示两行代码之间消耗的时间 () is: 8.1735ms
```
## chrono 
在rust中，使用日期库需要引入第三方库，chrono 是在rsut中使用最多的库，所以我们接下来的的日期处理都基于此库。所以需要我们在Cargo.toml引入`chrono = "0.4.31"`
### 时间计算
chrono 中提供的时间计算的方法有很多，接下来我将介绍几种常用的方法。

```rs
use chrono::{DateTime, Duration, Utc, Days, Months};

fn main() {
    // 获取 世界统一时间的现在时间
    let now = Utc::now();
    // 获取当地时间的现在时间
    // let local=Local::now();
    println!("当前时间：{}", now);
    // checked_add_signed 添加指定的时间到
    let almost_three_weeks_from_now = now.checked_add_signed(Duration::weeks(2));
    // checked_add_days 添加指定的天数
    let after_one_day=now.checked_add_days(Days::new(1));
    // checked_sub_months 添加指定的月数
    let after_one_mouth=now.checked_sub_months(Months::new(1));
    
    match almost_three_weeks_from_now {
        Some(x) => println!("两周后的时间：{}", x),
        None => eprintln!("时间格式不对"),
    }
    match after_one_day {
        Some(x) => println!("一天后的时间：{}", x),
        None => eprintln!("时间格式不对"),
    }
    match after_one_mouth {
        Some(x) => println!("一月后的时间：{}", x),
        None => eprintln!("时间格式不对"),
    }
}
```
在计算时间差比较麻烦，需要先指定格式，以下是计算时间差的代码
```rs
    let start_of_period = Utc.ymd(2020, 1, 1).and_hms(0, 0, 0);
    let end_of_period = Utc.ymd(2021, 1, 1).and_hms(0, 0, 0);
    let duration = end_of_period - start_of_period;
    println!("num days = {}", duration.num_days());
```

### 时间的时区转换


使用 offset::Local::now 获取本地时间并显示，然后使用 DateTime::from_utc 结构体方法将其转换为 UTC 标准格式。最后，使用 offset::FixedOffset 结构体，可以将 UTC 时间转换为 UTC+8 和 UTC-2。


```rs
use chrono::{DateTime, FixedOffset, Local, Utc};

fn main() {
    let local_time = Local::now();
    // 设置时间格式
    let utc_time = DateTime::<Utc>::from_utc(local_time.naive_utc(), Utc);
    // 进行时间偏移
    let china_timezone = FixedOffset::east(8 * 3600);
    println!("现在时间 {}", local_time);
    println!("UTC 时间 {}", utc_time);
    println!(
        "香港时间 {}",
        utc_time.with_timezone(&china_timezone)
    );
}

```




### 检查日期和时间
chrono-badge cat-date-and-time-badge

通过 Timelike 获取当前 UTC DateTime 及其时/分/秒，通过 Datelike 获取其年/月/日/工作日。

```rs
use chrono::{Datelike, Timelike, Utc};

fn main() {
    let now = Utc::now();

    let (is_pm, hour) = now.hour12(); //把时间转化为12小时制
    println!(
        "The current UTC time is {:02}:{:02}:{:02} {}", //设置格式
        hour,
        now.minute(),
        now.second(),
        if is_pm { "PM" } else { "AM" }
    );
    println!(
        "And there have been {} seconds since midnight",
        now.num_seconds_from_midnight() //输出到午夜的时间
    );

    let (is_common_era, year) = now.year_ce();//把时间转化为一年为单位
    println!(
        "The current UTC date is {}-{:02}-{:02} {:?} ({})",
        year,
        now.month(),
        now.day(),
        now.weekday(),
        if is_common_era { "CE" } else { "BCE" } //判断时间是公元前，还是公元后
    );
    println!(
        "And the Common Era began {} days ago", //据公元开始有多少年
        now.num_days_from_ce()
    );
}

```




### 日期和时间的格式化显示

使用 Utc::now 获取并显示当前 UTC 时间。使用 DateTime::to_rfc2822 将当前时间格式化为熟悉的 RFC 2822 格式，使用 DateTime::to_rfc3339 将当前时间格式化为熟悉的 RFC 3339 格式，也可以使用 DateTime::format 自定义时间格式。
```rs
use chrono::{DateTime, Utc};

fn main() {
    let now: DateTime<Utc> = Utc::now();

    println!("UTC now is: {}", now);
    println!("UTC now in RFC 2822 is: {}", now.to_rfc2822());
    println!("UTC now in RFC 3339 is: {}", now.to_rfc3339());
    println!("UTC now in a custom format is: {}", now.format("%a %b %e %T %Y"));
}
```
**效果**
```shell
UTC now is: 2023-12-02 13:22:23.639812500 UTC
UTC now in RFC 2822 is: Sat, 2 Dec 2023 13:22:23 +0000
UTC now in RFC 3339 is: 2023-12-02T13:22:23.639812500+00:00
UTC now in a custom format is: Sat Dec  2 13:22:23 2023
```

### 将字符串解析为 DateTime 结构体

熟悉的时间格式 RFC 2822、RFC 3339，以及自定义时间格式，通常用字符串表达。要将这些字符串解析为 DateTime 结构体，可以分别用 DateTime::parse_from_rfc2822、DateTime::parse_from_rfc3339，以及 DateTime::parse_from_str。

可以在 chrono::format::strftime 中找到适用于 DateTime::parse_from_str 的转义序列。注意：DateTime::parse_from_str 要求这些 DateTime 结构体必须是可创建的，以便它唯一地标识日期和时间。要解析不带时区的日期和时间，请使用 NaiveDate、NaiveTime，以及 NaiveDateTime。
```rs
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};
use chrono::format::ParseError;


fn main() -> Result<(), ParseError> {
    let rfc2822 = DateTime::parse_from_rfc2822("Tue, 1 Jul 2003 10:52:37 +0200")?;
    println!("{}", rfc2822);

    let rfc3339 = DateTime::parse_from_rfc3339("1996-12-19T16:39:57-08:00")?;
    println!("{}", rfc3339);

    let custom = DateTime::parse_from_str("5.8.1994 8:00 am +0000", "%d.%m.%Y %H:%M %P %z")?;
    println!("{}", custom);

    let time_only = NaiveTime::parse_from_str("23:56:04", "%H:%M:%S")?;
    println!("{}", time_only);

    let date_only = NaiveDate::parse_from_str("2015-09-05", "%Y-%m-%d")?;
    println!("{}", date_only);

    let no_timezone = NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")?;
    println!("{}", no_timezone);

    Ok(())
}


```
**效果**
```shell
2003-07-01 10:52:37 +02:00
1996-12-19 16:39:57 -08:00
1994-08-05 08:00:00 +00:00
23:56:04
2015-09-05
2015-09-05 23:56:04
```

### 日期和 UNIX 时间戳的互相转换

使用 NaiveDateTime::timestamp 将由 NaiveDate::from_ymd 生成的日期和由 NaiveTime::from_hms 生成的时间转换为 UNIX 时间戳。然后，它使用 NaiveDateTime::from_timestamp 计算自 UTC 时间 1970 年 01 月 01 日 00:00:00 开始的 10 亿秒后的日期。
```rs
use chrono::{NaiveDate, NaiveDateTime};

fn main() {
    let date_time: NaiveDateTime = NaiveDate::from_ymd(2017, 11, 12).and_hms(17, 33, 44);
    println!(
        "Number of seconds between 1970-01-01 00:00:00 and {} is {}.",
        date_time, date_time.timestamp());

    let date_time_after_a_billion_seconds = NaiveDateTime::from_timestamp(1_000_000_000, 0);
    println!(
        "Date after a billion seconds since 1970-01-01 00:00:00 was {}.",
        date_time_after_a_billion_seconds);
}
```
# clap
Clap 是一个用于命令行参数解析的 Rust 库。它提供了一种简单的方式来定义命令行参数，并使用这些参数来解析命令行输入。Clap 支持多种类型的参数，包括选项、子命令、环境变量和配置文件。
Clap 提供了多种功能，包括：
1. 命令行参数的解析：Clap 可以解析命令行参数，并自动将参数转换为指定的类型。
2. 帮助信息：Clap 可以自动生成帮助信息，包括参数的描述、默认值、示例等。

## 例子
首先我们要使用以下命令行引入依赖
>cargo add clap --features derive

然后我们开始编写第一个demo
```rs
use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,
    // 使用计数动作（clap::ArgAction::Count），意味着每多指定一次 -d 或 --debug，它的值就增加 1。
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
    #[command(subcommand)]
    command: Option<Commands>,
}
 // Commands 枚举定义了子命令 Test，它本身可以接受一个布尔类型的参数 list。
#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    // 检查了 name 和 config 参数是否被提供
    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {name}");
    }

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    // debug 参数被指定的次数来判断调试模式的状态。
    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    // 检查是否存在子命令 Test，并根据 list 参数的值来决定是否打印测试列表
    match &cli.command {
        Some(Commands::Test { list }) => {
            if *list {
                println!("Printing testing lists...");
            } else {
                println!("Not printing testing lists...");
            }
        }
        None => {}
    }
}
```
### 运行效果
```shell
./clap.exe --help   
Usage: clap.exe [OPTIONS] [NAME] [COMMAND]

Commands:
  test  does testing things
  help  Print this message or the help of the given subcommand(s)

Arguments:
  [NAME]  Optional name to operate on

Options:
  -c, --config <FILE>  Sets a custom config file
  -d, --debug...       Turn debugging information on
  -h, --help           Print help
  -V, --version        Print version


./clap.exe -dd test 
Debug mode is on
Not printing testing lists...
```
# xtask
在Rust编程语言中，构建和维护项目时，我们常常需要执行一些辅助性的任务，比如编译不同版本的二进制文件、运行测试、格式化代码、构建文档等等。这些任务虽然不是应用程序的核心部分，但对于项目的健康和可维护性至关重要。传统的做法是编写Makefiles或者使用各种shell脚本来完成这些工作，但这种方法存在一些缺点，如跨平台兼容性差、代码复杂难以维护，与rust生态割裂等。为了解决这些问题，Rust社区引入了一种新的模式——xtask。

## 什么是XTask？
XTask（扩展任务）是一种在Rust项目中定义和执行自定义构建任务的方式。它通过创建一个独立的Rust库或二进制项目来封装这些任务，利用Rust语言的强类型、安全性和跨平台能力，使得构建流程更加健壮、可读和可维护。

## XTask的工作原理
XTask项目通常包含在你的主项目目录下，例如在一个名为xtask的子目录中。这个目录可以包含一个Cargo.toml文件和一些Rust源代码文件，用于定义和实现自定义任务。当在终端中运行cargo xtask [command]时，cargo会识别到这是一个特殊的xtask命令，并调用相应的Rust代码来执行该任务。

## 如何创建XTask
要创建一个XTask，你需要在你的项目根目录下创建一个新的Cargo.toml文件和至少一个Rust源代码文件。在Cargo.toml中，你可以指定一个bin类型的包，这样就可以定义一个可执行的二进制文件，用来包含你的自定义任务逻辑。

下面是一个简单的xtask示例目录结构：
```shell
my_project/
├── .cargo/
│   └── config.toml
├── Cargo.toml
├──  subproject/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
│
└── xtask/
    ├── Cargo.toml
    └── src/
        └── main.rs
        
```
在xtask/Cargo.toml中，你可能会看到类似这样的配置：
```toml
[package]
name = "project_xtask"
version = "0.1.0"

[dependencies]
clap = { version = "4", features = ["derive"] }
```
在xtask/src/main.rs中，你可以定义你的自定义任务，例如：
```rs
use clap::Args;
use clap::Subcommand;
use clap::Parser;
// 通过.cargo 中 config.toml 中配置[alias]中
fn main() {
    match Cli::parse().command {
        Commands::ListTurbo => {
          println!("ListTurbo")
        }
        Commands::Deploy => {
            println!("Deploy")
        }
        Commands::Cast => {
           println!("Cast")
        }
        Commands::Generate => {
           println!("Generate")
        }
        Commands::Chat => {
           println!("Chat")
        }
        Commands::Service => {
           println!("Service")
        }
    }
}
#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands {
    ListTurbo,
    Deploy,
    Cast,
    Generate,
    Chat,
    Service,
}
```
上面的代码，构建了一个简单的命令行工具。这是用clap构建而成的，不了解的小伙伴可以了解以下。
同时我们需要在./cargo/config.toml 文件夹添加以下内容
```toml
[alias]
#   Cargo 不要运行默认包，而是运行名为 xtask 的包，同时使用 release 编译模式
xtask = "run --package xtask --release --"
debug = "run --package xtask --"
list-turbo = "xtask list-turbo"
deploy = "xtask deploy"
generate = "xtask generate"
chat = "xtask chat"
cast = "xtask cast"
service = "xtask service"
```
其中alias中的字段就是我们能够执行的命令。
## 测试
我们在在根目录的命令行输入以下内容
> cargo chat

显示输出
>Chat


##  结论
XTask提供了一种强大的、灵活的方式来管理Rust项目中的构建和自动化任务。它不仅可以简化项目维护，还可以提高团队协作效率，确保项目的一致性和稳定性。通过将常见的构建步骤封装到XTask中，开发者可以专注于业务逻辑，而不用担心构建过程中的细节问题。


# 基准测试 Criterion
## 入门
运行命令cargo bench --bench my_benchmark 

目录结构
```shell
│  Cargo.toml
│
├─benches
│      my_benchmark.rs
│
└─src
        lib.rs
        main.rs
``` 
因此社区 benchmark 就应运而生，其中最有名的就是 criterion.rs，它有几个重要特性:

统计分析，例如可以跟上一次运行的结果进行差异比对
图表，使用 gnuplots 展示详细的结果图表
首先，如果你需要图表，需要先安装 gnuplots，其次，我们需要引入相关的包，在 Cargo.toml 文件中新增 :
```toml
[dev-dependencies]
criterion = "0.3"

[[bench]]
name = "my_benchmark"
harness = false
```
接着，在项目中创建一个测试文件: $PROJECT/benches/my_benchmark.rs，然后加入以下内容：
```rs

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
```
最后，使用` cargo bench `运行并观察结果：
```shell
Gnuplot not found, using plotters backend
Fibonacci/10            time:   [263.10 ps 266.95 ps 270.83 ps]
                        change: [-3.0764% +0.0460% +3.0907%] (p = 0.98 > 0.05)
                        No change in performance detected.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
Fibonacci/20            time:   [267.64 ps 272.26 ps 277.85 ps]
                        change: [+4.0082% +6.8572% +9.9692%] (p = 0.00 < 0.05)
                        Performance has regressed.
Fibonacci/30            time:   [258.78 ps 263.52 ps 268.94 ps]
                        change: [-16.784% -10.402% -3.3094%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) high mild
  4 (4.00%) high severe
```
## 概述
### Warmup
每个 Criterion.rs 基准测试都会在可配置的预热期（默认情况下为 3 秒）内自动迭代基准测试函数。对于 Rust 函数基准测试，这是为了预热处理器缓存和（如果适用）文件系统缓存。

### Collecting Samples  
Criterion 通过不同的迭代次数迭代要进行基准测试的函数，以生成每次迭代所花费时间的估计值。样本数量是可配置的。它还根据预热期间每次迭代的时间打印采样过程所需时间的估计值。

### Time 
```shell
time:   [2.5094 ms 2.5306 ms 2.5553 ms]
thrpt:  [391.34 MiB/s 395.17 MiB/s 398.51 MiB/s]
```
这显示了此基准测试的每次迭代测量时间的置信区间。left 和 right 值分别显示置信区间的下限和上限，而 center 值显示 Criterion.rs 对基准测试例程的每次迭代所花费的时间的最佳估计值。


置信度是可配置的。较大的置信度（例如 99%）将扩大区间，从而为用户提供有关真实斜率的信息较少。另一方面，较小的置信区间（例如 90%）会缩小区间，但用户对区间包含真实斜率的信心会降低。95% 通常是一个不错的平衡。

Criterion.rs 执行 Bootstrap 重采样以生成这些置信区间。引导样本的数量是可配置的，默认为 100,000。或者，Criterion.rs 还可以以字节数或每秒元素数为单位报告基准测试代码的吞吐量。

### Change
运行 Criterion.rs 基准时，它会将统计信息保存在 target/criterion 目录中。基准测试的后续执行将加载此数据并将其与当前样本进行比较，以显示代码更改的效果。
```shell
change: [-38.292% -37.342% -36.524%] (p = 0.00 < 0.05)
Performance has improved.
```
这显示了本次基准测试与上次基准测试之间差异的置信区间，以及测量的差异可能偶然发生的概率。如果无法读取此基准测试的保存数据，则这些行将被省略。


第二行显示快速摘要。如果有强有力的统计证据表明情况确实如此，则此行将指示性能有所improved或者regressed。它还可能指示更改在噪声阈值范围内。Criterion.rs 尝试尽可能减少噪声的影响，但基准测试环境的差异（例如，与其他进程的不同负载、内存使用等）可能会影响结果。对于高度确定性的基准测试，Criterion.rs 可能足够敏感，可以检测到这些微小的波动，因此与 +-noise_threshold 范围重叠的基准测试结果被视为噪声，并被视为无关紧要。noise 阈值是可配置的，默认为 +-2%。

### Detecting Outliers 
```shell
Found 8 outliers among 100 measurements (8.00%)
  4 (4.00%) high mild
  4 (4.00%) high severe
```
Criterion.rs 尝试检测异常高或异常低的样本，并将其报告为异常值。大量异常值表明基准测试结果具有干扰性，应以适当的怀疑态度看待。在这种情况下，您可以看到有些样本花费的时间比正常情况要长得多。这可能是由于运行基准测试的计算机上的负载不可预测、线程或进程调度，或者被基准测试的代码所花费的时间不规则造成的。


为了确保可靠的结果，基准测试应在安静的计算机上运行，并且应设计为每次迭代执行大致相同的工作量。如果无法做到这一点，请考虑增加测量时间以减少异常值对结果的影响，但代价是基准测试周期更长。或者，可以延长预热期（以确保任何 JIT 编译器或类似编译器都已预热），或者可以使用其他迭代循环在每个基准测试之前执行设置，以防止这影响结果。

### Additional Statistics  其他统计数据
```shell
slope  [2.5094 ms 2.5553 ms] R^2            [0.8660614 0.8640630]
mean   [2.5142 ms 2.5557 ms] std. dev.      [62.868 us 149.50 us]
median [2.5023 ms 2.5262 ms] med. abs. dev. [40.034 us 73.259 us]
```
这将显示基于其他统计数据的其他置信区间。


Criterion.rs 执行线性回归以计算每次迭代的时间。第一行显示线性回归中斜率的置信区间，而 R^2 区域显示该置信区间的下限和上限的拟合优度值。如果 R^2 值较低，这可能表示基准测试在每次迭代中执行的工作量不同。您可能希望检查绘图输出并考虑提高基准测试例程的一致性。


第二行显示每次迭代时间的平均值和标准差的置信区间（天真地计算）。如果 std. dev. 与上面的时间值相比很大，则基准测试是嘈杂的。您可能需要更改基准测试以减少噪音。


中位数/中值绝对偏差线与平均值/标准差偏差线类似，不同之处在于它使用中位数和中位数绝对偏差。与标准差一样，如果 med. abs. dev. 很大，则表明基准测试有噪声。
# flamegraph
## windos
windos需要安装 DTrace 可以通过以下网址安装 https://www.microsoft.com/en-us/download/details.aspx?id=100441
# 构建脚本( Build Scripts)
一些项目希望编译第三方的非 Rust 代码，例如 C 依赖库；一些希望链接本地或者基于源码构建的 C 依赖库；还有一些项目需要功能性的工具，例如在构建之间执行一些代码生成的工作等。

对于这些目标，社区已经提供了一些工具来很好的解决，Cargo 并不想替代它们，但是为了给用户带来一些便利，Cargo 提供了自定义构建脚本的方式，来帮助用户更好的解决类似的问题。

## build.rs
若要创建构建脚本，我们只需在项目的根目录下添加一个 build.rs 文件即可。这样一来， Cargo 就会先编译和执行该构建脚本，然后再去构建整个项目。

以下是一个非常简单的脚本示例:
```rs
fn main() {
    // 以下代码告诉 Cargo，一旦指定的文件 `src/hello.c` 发生了改变，就重新运行当前的构建脚本
    println!("cargo:rerun-if-changed=src/hello.c");
    // 使用 `cc` 来构建一个 C 文件，然后进行静态链接
    cc::Build::new()
        .file("src/hello.c")
        .compile("hello");
}
```
关于构建脚本的一些使用场景如下：
- 构建 C 依赖库
- 在操作系统中寻找指定的 C 依赖库
- 根据某个说明描述文件生成一个 Rust 模块
- 执行一些平台相关的配置

下面的部分我们一起来看看构建脚本具体是如何工作的，然后在下个章节中还提供了一些关于如何编写构建脚本的示例。

Note: package.build 可以用于改变构建脚本的名称，或者直接禁用该功能

## 构建脚本的生命周期
在项目被构建之前，Cargo 会将构建脚本编译成一个可执行文件，然后运行该文件并执行相应的任务。

在运行的过程中，脚本可以使用之前 println 的方式跟 Cargo 进行通信：通信内容是以 cargo: 开头的格式化字符串。

需要注意的是，Cargo 也不是每次都会重新编译构建脚本，只有当脚本的内容或依赖发生变化时才会。默认情况下，任何文件变化都会触发重新编译，如果你希望对其进行定制，可以使用 rerun-if命令，后文会讲。

在构建脚本成功执行后，我们的项目就会开始进行编译。如果构建脚本的运行过程中发生错误，脚本应该通过返回一个非 0 码来立刻退出，在这种情况下，构建脚本的输出会被打印到终端中。

## 构建脚本的输入
我们可以通过环境变量的方式给构建脚本提供一些输入值，除此之外，构建脚本所在的当前目录也可以。

## 构建脚本的输出
构建脚本如果会产出文件，那么这些文件需要放在统一的目录中，该目录可以通过 OUT_DIR 环境变量来指定，构建脚本不应该修改该目录之外的任何文件！

在之前提到过，构建脚本可以通过 println! 输出内容跟 Cargo 进行通信：Cargo 会将每一行带有 cargo: 前缀的输出解析为一条指令，其它的输出内容会自动被忽略。

通过 println! 输出的内容在构建过程中默认是隐藏的，如果大家想要在终端中看到这些内容，你可以使用 -vv 来调用，以下 build.rs ：
```rs
fn main() {
    println!("hello, build.rs");
}
```
将输出:
```shell
$ cargo run -vv
[study_cargo 0.1.0] hello, build.rs
```
构建脚本打印到标准输出 stdout 的所有内容将保存在文件 target/debug/build/<pkg>/output 中 (具体的位置可能取决于你的配置)，stderr 的输出内容也将保存在同一个目录中。

以下是 Cargo 能识别的通信指令以及简介，如果大家希望深入了解每个命令，可以点击具体的链接查看官方文档的说明。

- cargo:rerun-if-changed=PATH — 当指定路径的文件发生变化时，Cargo 会重新运行脚本
- cargo:rerun-if-env-changed=VAR — 当指定的环境变量发生变化时，Cargo 会重新运行脚本
- cargo:rustc-link-arg=FLAG – 将自定义的 flags 传给 linker，用于后续的基准性能测试 benchmark、 可执行文件 binary,、cdylib 包、示例和测试
- cargo:rustc-link-arg-bin=BIN=FLAG – 自定义的 flags 传给 linker，用于可执行文件 BIN
- cargo:rustc-link-arg-bins=FLAG – 自定义的 flags 传给 linker，用于可执行文件
- cargo:rustc-link-arg-tests=FLAG – 自定义的 flags 传给 linker，用于测试
- cargo:rustc-link-arg-examples=FLAG – 自定义的 flags 传给 linker，用于示例
- cargo:rustc-link-arg-benches=FLAG – 自定义的 flags 传给 linker，用于基准性能测试 benchmark
- cargo:rustc-cdylib-link-arg=FLAG — 自定义的 flags 传给 linker，用于 cdylib 包
- cargo:rustc-link-lib=[KIND=]NAME — 告知 Cargo 通过 -l 去链接一个指定的库，往往用于链接一个本地库，通过 FFI
- cargo:rustc-link-search=[KIND=]PATH — 告知 Cargo 通过 -L 将一个目录添加到依赖库的搜索路径中
- cargo:rustc-flags=FLAGS — 将特定的 flags 传给编译器
- cargo:rustc-cfg=KEY[="VALUE"] — 开启编译时 cfg 设置
- cargo:rustc-env=VAR=VALUE — 设置一个环境变量
- cargo:warning=MESSAGE — 在终端打印一条 warning 信息
- cargo:KEY=VALUE — links 脚本使用的元数据

## 构建脚本的依赖
构建脚本也可以引入其它基于 Cargo 的依赖包，只需要在 Cargo.toml 中添加或修改以下内容:
```toml
[build-dependencies]
cc = "1.0.46"
```
需要这么配置的原因在于构建脚本无法使用通过 [dependencies] 或 [dev-dependencies] 引入的依赖包，因为构建脚本的编译运行过程跟项目本身的编译过程是分离的的，且前者先于后者发生。同样的，我们项目也无法使用 [build-dependencies] 中的依赖包。

大家在引入依赖的时候，需要仔细考虑它会给编译时间、开源协议和维护性等方面带来什么样的影响。如果你在 [build-dependencies] 和 [dependencies] 引入了同样的包，这种情况下 Cargo 也许会对依赖进行复用，也许不会，例如在交叉编译时，如果不会，那编译速度自然会受到不小的影响。

## links
在 Cargo.toml 中可以配置 package.links 选项，它的目的是告诉 Cargo 当前项目所链接的本地库，同时提供了一种方式可以在项目构建脚本之间传递元信息。
```toml
[package]
# ...
links = "foo"
```
以上配置表明项目链接到一个 libfoo 本地库，当使用 links 时，项目必须拥有一个构建脚本，并且该脚本需要使用 rustc-link-lib 指令来链接目标库。

Cargo 要求一个本地库最多只能被一个项目所链接，换而言之，你无法让两个项目链接到同一个本地库，但是有一种方法可以降低这种限制，感兴趣的同学可以看看官方文档。

假设 A 项目的构建脚本生成任意数量的 kv 形式的元数据，那这些元数据将传递给 A 用作依赖包的项目的构建脚本。例如，如果包 bar 依赖于 foo，当 foo 生成 key=value 形式的构建脚本元数据时，那么 bar 的构建脚本就可以通过环境变量的形式使用该元数据：DEP_FOO_KEY=value。

需要注意的是，该元数据只能传给直接相关者，对于间接的，例如依赖的依赖，就无能为力了。

## 覆盖构建脚本
当 Cargo.toml 设置了 links 时， Cargo 就允许我们使用自定义库对现有的构建脚本进行覆盖。在 Cargo 使用的配置文件中添加以下内容：
```toml
[target.x86_64-unknown-linux-gnu.foo]
rustc-link-lib = ["foo"]
rustc-link-search = ["/path/to/foo"]
rustc-flags = "-L /some/path"
rustc-cfg = ['key="value"']
rustc-env = {key = "value"}
rustc-cdylib-link-arg = ["…"]
metadata_key1 = "value"
metadata_key2 = "value"
```
增加这个配置后，在未来，一旦我们的某个项目声明了它链接到 foo，那项目的构建脚本将不会被编译和运行，替代的是这里的配置将被使用。

warning, rerun-if-changed 和 rerun-if-env-changed 这三个 key 在这里不应该被使用，就算用了也会被忽略。

# tokio
## 简介
Tokio 速度很快，构建在 Rust 编程语言之上，而 Rust 编程语言本身就是 快。这是本着 Rust 的精神完成的，目标是你不应该 能够通过手动编写等效代码来提高性能。

Tokio 是可扩展的，构建在 async/await 语言功能之上，该功能 本身是可扩展的。在处理网络时，速度是有限制的 由于延迟，您可以处理连接，因此扩展的唯一方法是 一次处理多个连接。使用 async/await 语言功能，增加并发操作的数量变得非常便宜，允许您扩展到大量并发任务。

## 快速开始
### 第一个 tokio 程序
首先，在 Cargo.toml 文件中添加 Tokio 依赖项：
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```
然后，在 src/main.rs 中编写如下代码：
```rs
use tokio;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
}
```
以上就是我们的第一个 tokio 程序，可能看以来很疑惑，为什么仅仅通过添加一行代码就能启动一个异步程序，接下来将会展示`#[tokio::main]`宏下隐藏的细节。

`#[tokio::main]`宏主要做了以下的功能。
- 启动运行时：宏会生成代码来创建并启动 Tokio 运行时（Runtime）。
进入事件循环：
启动后，运行时会进入一个事件循环，等待异步任务被唤醒或完成。这个循环是无阻塞的，并且能够高效地处理大量的并发任务。
- 调用异步函数：宏会确保你的 main 函数作为异步函数被执行，这意味着它可以使用 .await 语法来暂停和恢复执行，直到所有依赖的异步操作完成。
- 阻塞当前线程：main 函数本身是异步的，但整个程序需要有一个同步的起点。因此，#[tokio::main] 宏会阻塞主线程，直到 main 函数返回。
- 配置选项：宏还允许你通过属性指定不同的配置选项。例如，你可以选择不同的调度器类型（如多线程或单线程），或者启用或禁用各种特性。
- 错误传播：如果 main 函数返回一个结果类型（即 Result<T, E>），宏还会处理可能发生的错误，这通常意味着如果异步操作失败，程序将会以非零状态退出。
- 资源清理：当所有的异步操作都完成后，宏会确保正确地清理所有分配的资源，包括关闭运行时。

接下来展示以下我们的`#[tokio::main]` 宏展开后的代码。
```rs

fn main() {
    // 输出的异步函数
    let body = async {
        {
            println!("Hello, world!\n");
        };
    };
    {   //构建我们的异步运行时
        return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body); //阻塞，直到异步函数执行完成
    }
}
```
## runtime
Tokio 运行时就像是一个繁忙的机场调度中心，而异步程序则是那些需要起飞和降落的飞机。

在机场（运行时）里，调度中心负责协调所有航班（任务）的活动。它知道什么时候跑道是空闲的，可以安排一架飞机起飞或降落；它还管理着地勤服务，确保加油、乘客上下机等操作不会阻碍其他飞机的操作。同时，调度中心还处理各种意外情况，比如天气变化导致的延误，或者紧急情况下的优先降落。

异步程序就像是这些飞机。它们准备好了就请求起飞（启动一个异步任务），一旦完成某个阶段的任务（例如到达目的地），就会通知调度中心（完成一部分 Future）。如果途中遇到等待（如等待网络响应或文件 I/O 操作），它们会告诉调度中心自己正在等待，并允许其他飞机使用跑道（让出线程资源给其他任务）。

每当有飞机完成了它的飞行计划（任务完成），调度中心就会记录下来，并根据需要安排下一个航班。如果有突发状况，比如某架飞机需要立即降落（高优先级任务），调度中心也会迅速做出调整，确保一切顺利进行。

因此，Tokio 运行时就像这个高效的机场调度中心，它确保所有的异步任务（飞机）都能有序、高效地运行，最大化利用可用资源（跑道、地勤人员等），并且能够灵活应对各种不可预见的情况。通过这种方式，即使在高并发环境下，系统也能够保持流畅和平稳的运作。

所以与其他 Rust 程序不同，异步应用程序需要运行时 支持。具体而言，以下运行时服务是必需的。

可能在大多数情况下我们的程序入口是一串行的程序，只有到某一个点才会进入异步代码，这时候我们 runtime 就派上用场了，他会负责给我们的异步程序提供一个能够运行的环境。

例子：
```rs
use tokio::runtime::Builder;

fn main() {
    // 创建一个运行时构建器
    let runtime = Builder::new_multi_thread() // 或者 new_current_thread() 为单线程调度
        .worker_threads(1) // 设置工作线程的数量
        .thread_name("my-custom-thread") // 设置线程名称
        .enable_all() // 启用 I/O 和时间驱动，
        .build()
        .unwrap(); // 构建运行时

    // 使用运行时执行异步代码
    (0..100).into_iter().for_each(|i|{
        runtime.spawn(async move {
            println!("Hello, world! time: {}",i);
        });
    });
}
```

以上的的例子就是展示我们直接使用 runtime 构建我们的异步程序的例子，可能有些同学觉得有些奇怪，为什么明明看到的循环是 100 次为什么自己运行的结果却小于 100。这就涉及的异步程序的特点，在习惯同步程序中，程序都是串行的执行的，程序会逐行执行，但是在异步程序中却不是这样。我们的异步程序一般不会和同步代码在一个线程之中，这就导致了如果我们的主线程的同步代码运行的太快，就会导致我们的异步代码还来不及完成整个主线程就结束了，就如同上面的例子一样，这样该如何改进呢？答案就是我们要获取个任务的 handle，然后使用它的 await 方法来等待异步任务完成。

```rs
use tokio::runtime::Builder;

fn main() {
    // 创建一个运行时构建器
    let runtime = Builder::new_multi_thread() // 或者 new_current_thread() 为单线程调度
        .worker_threads(1) // 设置工作线程的数量
        .thread_name("my-custom-thread") // 设置线程名称
        .enable_all() // 启用 I/O 和时间驱动
        .build()
        .unwrap(); // 构建运行时
    // 使用运行时执行异步代码

    let tasks: Vec<_> = (0..100)
        .into_iter()
        .map(|i| {
            // 使用运行时执行异步代码
            runtime.spawn(async move {
                println!("Hello, world! time: {}", i);
            })
        })
        .collect();
    // 等待所有任务完成
    runtime.block_on(async {
        for task in tasks {
            // 等待每一个任务完成
            // 注意：如果你的任务中可能会有错误，这里应该处理 join 的结果
            let _ = task.await;
        }
    });
}
```

修改为以上的代码就可以正确显示我们的结果了。但是你可能觉得比较繁琐，为什么我们的 await 方法要放在 block_on 方法之中呢？这是因为 await 方法只能在异步代码块中使用，我们的 main 方法是一个同步代码块，所以必须使用 runtime 生成一个异步代码块，然后再使用 await 方法。有些同学可能觉得写循环太不优雅了和有点繁琐，当然我们也有更优雅的方式来实现。首先我们需要引入一个rust官方提供的包futures，通过`cargo add futures`来引入。然后把我们最后的一个异步代码块改成以下形式，你是不是绝的优雅了许多。
```rs
    // 等待所有任务完成
    runtime.block_on(futures::future::join_all(tasks)); 
```

## sync
可能会有人疑惑，在异步程序内如何进行通信呢，接下来将为你介绍几个 常用的方法。
当然，下面是每种方法的一个简单代码示例。请注意，这些例子假设你已经安装了 Tokio，并且在你的 Cargo.toml 文件中添加了必要的依赖项。

### Channels 
在 Tokio 中，Channels 用于线程间或任务间的通信。以下是几种常见的 Channels 类型及其作用：
- mpsc (Multi-producer, single-consumer):
    - tokio::sync::mpsc 提供了多生产者单消费者的通道。它可以有多个发送者（producers），但只有一个接收者（consumer）。这种类型的通道适用于多个任务向同一个任务发送消息的情况。
    - 包含 unbounded 和 bounded 两种类型：
        - unbounded: 不限制通道中可以同时存在的消息数量，可能导致内存使用不受控制地增长。
        - bounded: 设置了一个容量上限，当通道满了的时候，发送操作会等待直到有足够的空间。
oneshot:
    - tokio::sync::oneshot 提供了一次性的通道，只允许发送一条消息。一旦消息被发送并接收后，该通道即关闭。这通常用于任务完成信号或者返回值传递。
- watch:
    - tokio::sync::watch 提供了一个广播式的通道，允许多个接收者监听来自一个发送者的更新。每个接收者只会看到最新的值，并且可以在接收到新值时得到通知。这非常适合用于状态广播，例如配置更新。
- broadcast:
    - tokio::sync::broadcast 提供了一个广播通道，支持多个接收者。与 watch 不同的是，所有的接收者都可以选择是否接收每条消息，并且如果它们没有及时接收，可能会错过一些消息。它有一个固定的缓冲区大小，超出的消息会被丢弃。

由于他们的用法都差不多，这里就以mpsc为例来进行说明。
```rs
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);

    tokio::spawn(async move {
        tx.send("hello").await.unwrap();
    });

    if let Some(message) = rx.recv().await {
        println!("Got = {}", message);
    }
}
```
这就是最简单的例子，当然其中还有很多用法的细节，就留在以后探索了。
### Shared State
在 Tokio 中，共享状态是指多个任务或线程可以同时访问和修改的状态。最常见的方式就是加锁。
```rs
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let data = Arc::new(Mutex::new(0));
    let mut tasks = Vec::new();

    for i in 0..10 {
        let my_data = Arc::clone(&data);
        let task = tokio::spawn(async move {
            let mut data = my_data.lock().await;
            *data += i;
        });
        tasks.push(task);
    }

    for task in tasks {
        task.await.unwrap();
    }

    println!("Final count: {}", *data.lock().await);
}
```

### Synchronization Primitives (Notify)
在 Tokio 中，Notify 是一种同步原语，用于任务间的简单通知机制。它允许一个或多个任务等待某个事件的发生，而另一个任务负责通知这个事件已经发生。Notify 的设计目的是为了简化异步代码中常见的“唤醒”模式，即一个任务需要等待直到另一任务完成某个操作或者满足某个条件。
```rs
use tokio::sync::Notify;

#[tokio::main]
async fn main() {
    let notify = Notify::new();
    let notified = notify.notified();

    // Spawn a task that will notify others.
    tokio::spawn(async {
        // Do some work...
        notify.notify_one();
    });

    // Wait to be notified.
    notified.await;
    println!("Received notification!");
}
```
### Task Local Storage (TLS)
线程局部存储（Thread Local Storage, TLS）允许每个线程拥有其自己的变量副本。而在异步编程中，我们通常使用单线程或多线程的异步运行时，如 Tokio，其中多个任务可能在同一线程上复用。
```rs
use std::cell::RefCell;
use tokio::task_local;

task_local! {
    static TASK_ID: RefCell<u64>;
}

#[tokio::main]
async fn main() {
    TASK_ID.scope(RefCell::new(42), async {
        println!("Task ID is: {}", TASK_ID.with(|id| *id.borrow()));
    }).await;
}
```
### Async-aware Data Structures (Watch)
在 Tokio 中，Watch 是一个异步感知的数据结构，它用于一对多的通信模式，即一个发送者（sender）可以向多个接收者（receiver）广播更新。它的设计目的是为了简化跨任务之间的状态同步问题，特别是当这些任务可能在不同的线程上并发执行时。
```rs
use tokio::sync::watch;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = watch::channel("initial");

    tokio::spawn(async move {
        // Update the value
        tx.send("updated").unwrap();
    });

    // Wait for an update
    rx.changed().await.unwrap();
    println!("Value changed to: {}", *rx.borrow());
}
```


# axum
## 简介
Axum 是一个用于构建 Web 应用程序的 Rust 框架，它建立在 Tokio 和 Tower 之上。Axum 的设计目标是提供一种简单且灵活的方式来创建异步 Web 服务，同时保持高性能和安全性。
**特点**：
- 异步优先：Axum 是为异步编程而设计的，充分利用了 Rust 的 async/await 语法，使得编写非阻塞的 Web 服务变得简单。
- 基于 HTTP 规范：Axum 使用 http crate 来处理请求和响应，这确保了框架与标准 HTTP 协议的良好兼容性。
- 中间件支持：通过使用 Tower 提供的功能，Axum 支持中间件来处理跨多个路由的通用逻辑，如日志记录、身份验证等。
- 路由：Axum 提供了一个直观的 API 来定义路由，可以轻松地将 URL 路径映射到相应的处理函数。
- 提取器：这是 Axum 中的一个重要概念，允许从请求中提取数据（例如路径参数、查询参数、请求体等）并直接作为处理器函数的参数传递。
- 类型安全：由于 Rust 的强类型系统，Axum 的 API 设计使得许多错误可以在编译时被捕捉，从而提高了代码的安全性和可靠性。
- 社区和生态系统：作为一个活跃维护的项目，Axum 有一个不断增长的社区和丰富的插件生态系统。
- 性能：得益于 Rust 的零成本抽象和其他优化特性，Axum 可以实现非常高的性能。
- 灵活性：虽然 Axum 提供了很多开箱即用的功能，但它也允许开发者根据需要进行定制和扩展。

## 快速开始
以下是一个简单的 Axum 应用程序示例，它定义了一个根路由和一个创建用户的路由：
```rs
use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
// 引入 serde 库中的 Deserialize 和 Serialize 特性，用于数据结构的序列化和反序列化。
use serde::{Deserialize, Serialize};

// 使用 tokio 作为异步运行时，并定义异步 main 函数。
#[tokio::main]
async fn main() {
    // 初始化日志记录格式器，以便在程序中输出跟踪信息。
    tracing_subscriber::fmt::init();

    // 创建一个新的Router实例，并定义两个路由：
    // 1. 对根路径"/"的GET请求，调用`root`处理函数；
    // 2. 对"/users"路径的POST请求，调用`create_user`处理函数。
    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user));

    // 绑定到TCP地址0.0.0.0:3000并开始监听传入连接。
    // 然后使用`axum::serve`来服务HTTP请求。
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// 定义一个异步函数`root`，它响应根路径的 GET 请求，
// 并返回一个静态字符串"Hello, World!"。
async fn root() -> &'static str {
    "Hello, World!"
}

// 定义一个异步函数`create_user`，它接受一个 JSON 格式的`CreateUser`对象作为输入，
// 并返回一个包含 HTTP 状态码和 JSON 格式的`User`对象的元组。
async fn create_user(
    // 解构出从请求体中提取的 JSON 数据，并将其转换为`CreateUser`类型。
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // 创建一个新的`User`实例，给定固定的 ID `1337`和从请求体中提取的用户名。
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // 返回HTTP状态码201 Created和序列化后的`User`对象作为响应。
    (StatusCode::CREATED, Json(user))
}

// 定义了一个`CreateUser`结构体，表示创建用户的请求体，
// 其中只包含一个字段`username`，并且实现了`Deserialize`特性，
// 这使得它可以被自动地从 JSON 格式的数据中解析出来。
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// 定义了一个`User`结构体，表示用户信息，
// 包含一个无符号 64 位整数类型的`id`和一个字符串类型的`username`，
// 并且实现了`Serialize`特性，这使得它可以被自动地序列化为 JSON 格式的数据。
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
```
