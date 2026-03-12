use clap::command;
use clap::Parser;
use common_base::config::placement_center::init_placement_center_conf_by_path;
use common_base::config::placement_center::placement_center_conf;
use common_base::log::placement_center::init_placement_center_log;

use log::info;


// 定义默认的配置路径，即当命令行没传配置路径时，默认的配置文件路径
pub const DEFAULT_PLACEMENT_CENTER_CONFIG: &str = "config/placement-center.toml";


// 定义接收哪些参数
#[derive(Parser, Debug)]
#[command(author="robustmq-geek", version="0.0.1", about=" RobustMQ: Next generation cloud-native converged high-performance message queue.", long_about = None)]
#[command(next_line_help = true)]
struct ArgsParams {
    #[arg(short, long, default_value_t=String::from(DEFAULT_PLACEMENT_CENTER_CONFIG))]
    conf: String,
}


fn main() {
    // 解析命令行参数
    let args = ArgsParams::parse();
    println!("conf path: {:?}", args.conf);
    // 初始化配置文件
    init_placement_center_conf_by_path(&args.conf);
    // 初始化日志 
    init_placement_center_log(); 
    let conf = placement_center_conf(); 
    // 记录日志 
    info!("{:?}", conf); 
    // start_server();
}