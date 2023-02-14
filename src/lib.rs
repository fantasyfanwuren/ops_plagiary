use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt;
use std::fs;
use std::io::Write;
use std::path::Path;

#[derive(Debug)]
struct StrError(String);

impl std::error::Error for StrError {}

impl std::fmt::Display for StrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct NFTData {
    token_id: Option<serde_json::Value>,
    #[serde(alias = "artist")]
    created_by: Option<String>,
    dna: Option<String>,
    name: Option<String>,
    description: Option<String>,
    #[serde(alias = "imageUrl")]
    image: Option<String>,
    image_url: Option<String>,
    external_url: Option<String>,
    holder: Option<String>,
    #[serde(rename = "imageHash")]
    image_hash: Option<String>,
    image_details: Option<serde_json::Value>,
    edition: Option<serde_json::Value>,
    date: Option<serde_json::Value>,
    attributes: Option<Vec<Attribute>>,
    compiler: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Attribute {
    trait_type: String,
    value: Option<serde_json::Value>,
    display_type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ImageDetails {
    bytes: Option<serde_json::Value>,
    format: Option<String>,
    sha256: Option<String>,
    width: Option<serde_json::Value>,
    height: Option<serde_json::Value>,
}

// 用户下载形式
#[derive(Debug)]
pub enum UrlType {
    Https,
    Ar,
}
pub fn get_url_type() -> UrlType {
    println!("\n请输入您的NFT元数据存储链接的形式(若不明白,请查看README):");
    println!(
        "1.所有元数据的地址,地址与NFT编号之间存在具有明显的规律性.
    例如:https://be.undeadblocks.com/api/metadata/123
    或者:ipfs://QmboCxEoxbJDwkq5RgFJsAHXFHYQUjUsSLxSaKtdsqgRbb/123"
    );
    println!("2.采用AR形式进行存储,且地址与NFT编号之间不具有明显的规律性,但拥有公开的接口可以查询");

    let mut url_type = String::new();
    std::io::stdin()
        .read_line(&mut url_type)
        .expect("读取用户输入信息失败");
    if url_type.trim() == "1" {
        UrlType::Https
    } else if url_type.trim() == "2" {
        UrlType::Ar
    } else {
        panic!("请按照要求输入正确数字,数字范围1或者2");
    }
}

pub fn get_meta_urls(url_type: &UrlType) -> (Vec<String>, usize) {
    println!("\n请输入同步线程数:(推荐25-100)");
    let mut thread_num = String::new();
    std::io::stdin()
        .read_line(&mut thread_num)
        .expect("读取同步线程数失败");
    let thread_num: usize = thread_num
        .trim()
        .parse()
        .expect("解析同步线程数失败,请输入一个数字");

    match url_type {
        UrlType::Https => {
            println!("检测到您选择了https类型");
            (get_https_urls(), thread_num)
        }

        UrlType::Ar => {
            println!("检测到您选择了随机/AR类型");
            (get_ar_urls(), thread_num)
        }
    }
}

// 获取http链接集合
fn get_https_urls() -> Vec<String> {
    println!(
        "\n那么,请您输入NFT元数据存储链接的根地址(最后记得要有斜杠):
        (例如:https://be.undeadblocks.com/api/metadata/
        或者ipfs://QmboCxEoxbJDwkq5RgFJsAHXFHYQUjUsSLxSaKtdsqgRbb/):"
    );
    let mut base_url = String::new();
    std::io::stdin()
        .read_line(&mut base_url)
        .expect("读取根地址失败");
    let base_url = base_url.trim();

    println!("\n请输入NFT的结尾编号:");
    let mut end = String::new();
    std::io::stdin()
        .read_line(&mut end)
        .expect("读取结尾编号失败");
    let end: usize = end.trim().parse().expect("解析结尾编号失败");

    // 将bese_url转化为https类型的地址
    let base_url = match convert_to_https(base_url) {
        Some(s) => s,
        None => panic!("您输入的地址无效"),
    };

    let mut result = vec![];
    for i in 1..=end {
        let url = format!("{}{}", base_url, i);
        result.push(url);
    }
    result
}

// 获取Ar随机链接集合
struct EthCilent {
    nodes: Vec<String>,
    client: Client,
    contract: String,
    curent_node_id: usize,
}
impl EthCilent {
    pub fn new(client: Client, contract: String, nodes: Vec<String>) -> Self {
        Self {
            nodes,
            client,
            contract,
            curent_node_id: 0,
        }
    }
    pub fn get_mata_url(&mut self, id: usize) -> Option<String> {
        let data = usize_to_ar_data(id);
        let content = json!(
        {"jsonrpc":"2.0",
        "id":2,
        "method":"eth_call",
        "params":[
            {"from":"0x0000000000000000000000000000000000000000",
            "data":data,
            "to":self.contract},
            "latest"
            ]});

        let result = loop {
            match self.nodes.get(self.curent_node_id) {
                Some(node) => {
                    let res = match self
                        .client
                        .post(node)
                        .header("Content-Type", "application/json")
                        .header("referer", "https://etherscan.io")
                        .header("authority", "node1.web3api.com")
                        .json(&content)
                        .send()
                    {
                        Ok(s) => s,
                        Err(_) => {
                            self.curent_node_id += 1;
                            continue;
                        }
                    };

                    let res = res.text().expect("无法将response转化为文本");
                    let result_index = match res.find("\"result\":\"") {
                        Some(s) => s,
                        None => {
                            self.curent_node_id += 1;
                            continue;
                        }
                    };
                    let res = &res[result_index + 10..];
                    let last_index = match res.find("\"") {
                        Some(s) => s,
                        None => {
                            self.curent_node_id += 1;
                            continue;
                        }
                    };

                    let res = &res[..last_index];
                    let res = h_to_string(res);
                    break Some(res);
                }
                None => {
                    break None;
                }
            }
        };
        result
    }
}

fn h_to_string(hash_str: &str) -> String {
    let hash_str = hash_str.trim_start_matches("0x");
    let len = hash_str.len();
    let mut i = 0;
    let mut hash_str_vec = vec![];
    while i < len {
        let temp = &hash_str[i..i + 2];
        let hash_byte = u8::from_str_radix(temp, 16).expect("将16进制字符串转化为数字失败");
        hash_str_vec.push(hash_byte);
        i += 2;
    }
    let res = String::from_utf8_lossy(&hash_str_vec).to_string();
    let res_vec: Vec<&str> = res.split("?").collect();
    match res_vec.get(1) {
        Some(&e)=>e.to_string(),
        None => panic!("转化哈希过程失败,这可能由三方面原因导致,第一种是合约地址不正确,第二种是输入的范围超出了要爬取nft的编号范围,第三种是网络问题")
        }
}

fn usize_to_ar_data(nft_num: usize) -> String {
    let max_len = 74_usize;
    let nft_num = format!("{:x}", nft_num);
    let len = nft_num.len();
    let zero_len = max_len - len - "0x0e89341c".len();
    let mut zero = String::new();
    for _ in 0..zero_len {
        zero.push('0');
    }
    format!("0x0e89341c{}{}", zero, nft_num)
}

fn get_ar_urls() -> Vec<String> {
    println!("那么,请您输入NFT合约地址(通过查看nft的opensea链接就可以获得,例如:0x960b7a6bcd451c9968473f7bbfd9be826efd549a)");
    let mut contract = String::new();
    std::io::stdin()
        .read_line(&mut contract)
        .expect("无法读取到您输入的合约地址");
    let contract = contract.trim().to_string();

    println!("请输入NFT的结尾编号:");
    let mut end = String::new();
    std::io::stdin()
        .read_line(&mut end)
        .expect("无法读取您输入的结尾编号");
    let end: usize = end.trim().parse().expect("解析结尾编号失败");
    let nodes = vec![
        "https://eth-mainnet.gateway.pokt.network/v1/5f3453978e354ab992c4da79".to_string(),
        "https://cloudflare-eth.com/".to_string(),
        "https://nodes.mewapi.io/rpc/eth".to_string(),
        "https://node1.web3api.com".to_string(),
        "https://eth-mainnet.token.im".to_string(),
    ];

    let client = reqwest::blocking::Client::new();
    let mut eth_cilent = EthCilent::new(client, contract, nodes);
    let mut result = vec![];
    for id in 1..=end {
        println!("正在为您从以太坊节点上查询该NFT的元数据链接...");
        let mata_url = eth_cilent.get_mata_url(id).expect(
            "无法获取到mata链接,这可能由于三方面原因:\n
                1.您的合约地址可能不正确;\n
                2.您的网络存在问题;\n
                3.您输入的范围可能超出了nft边界;\n
                4.您的ip被目前已知的ethprc节点封杀了\n",
        );
        println!("获取编号为{id}的NFT的元数据链接为:{mata_url}");
        result.push(mata_url);
    }

    result
}

// 创建文件夹功能
pub fn make_nft_dir() -> String {
    let mut root_dir = String::new();
    println!("请填写保存文件夹的名称(建议英文):");
    std::io::stdin()
        .read_line(&mut root_dir)
        .expect("读取nft名称失败");
    let root_dir = root_dir.trim();
    let meta_dir = format!("{}/meta", root_dir);
    let img_dir = format!("{}/img", root_dir);

    // 尝试删除根文件夹
    match fs::metadata(root_dir) {
        Ok(_) => {
            match fs::remove_dir_all(root_dir) {
                Ok(_) => println!("删除已经存在的根文件夹成功"),
                Err(e) => println!("删除根文件夹失败 {:?}", e),
            };
        }
        Err(_) => {}
    }

    // 创建根文件夹
    match fs::create_dir(root_dir) {
        Ok(_) => println!("根文件夹创建成功:{}", root_dir),
        Err(e) => println!("根文件夹创建失败: {:?}", e),
    }

    // 创建meta文件夹
    match fs::create_dir(&meta_dir) {
        Ok(_) => println!("子文件夹创建成功:{}", meta_dir),
        Err(e) => println!("子文件夹创建失败:{:?}", e),
    }

    // 创建img文件夹
    match fs::create_dir(&img_dir) {
        Ok(_) => println!("子文件夹创建成功:{}", img_dir),
        Err(e) => println!("子文件夹创建失败: {:?}", e),
    }
    root_dir.to_owned()
}

// 所有下载功能,同时下载元数据和imgurl所指向的文件
pub fn download_nft(
    id: usize,
    url: &str,
    root_dir: &str,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    // 将元数据下载链接转化为:https类型
    let url = convert_to_https(url).expect("无法将元数据地址该转为https格式");
    // 下载元数据
    let meta_path = format!("{root_dir}/meta/{id}");
    let mut err_time = 0;
    loop {
        match download(&url, &meta_path) {
            Ok(_) => break,
            Err(e) => {
                err_time += 1;
                if err_time >= 100 {
                    return Err(e);
                }
            }
        }
    }

    // 读取元数据
    let meta = String::from_utf8(fs::read(meta_path)?)?;

    // 从元数据获取链接地址
    let img_url = match get_imag_url(&meta) {
        Some(s) => s,
        None => {
            return Err(Box::new(StrError(
                "无法将元数据中的链接进行提取,别慌，程序将继续推进..".to_owned(),
            )))
        }
    };

    // 将img地址转为https地址
    let img_url = convert_to_https(&img_url).expect("无法将img地址该转为https格式");
    let img_path = format!("{root_dir}/img/{id}");

    // 下载img文件
    let mut err_time = 0;
    loop {
        match download(&img_url, &img_path) {
            Ok(_) => break,
            Err(e) => {
                err_time += 1;
                if err_time >= 100 {
                    return Err(e);
                }
            }
        }
    }
    Ok(())
}

fn download(url: &str, file_path: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
    // 先获取响应
    let response = reqwest::blocking::get(url)?;
    let content_type = response
        .headers()
        .get("content-type")
        .and_then(|ct| ct.to_str().ok())
        .unwrap_or("");
    // 获得后缀名
    let extension = match content_type {
        "image/png" => ".png".to_owned(),
        "image/jpeg" => ".jpg".to_owned(),
        "image/img" => ".img".to_owned(),
        "image/webp" => ".webp".to_owned(),
        "application/json; charset=utf-8" => "".to_owned(),
        _ => {
            // 需要根据url来解析
            let url_path = Path::new(url);
            match url_path.extension() {
                Some(s) => {
                    let ss = s.to_str().unwrap();
                    format!(".{ss}")
                }
                None => "".to_string(),
            }
        }
    };
    let buffer = response.bytes()?;
    let filename = format!("{file_path}{extension}");
    let mut file = std::fs::File::create(filename)?;
    file.write_all(&buffer)?;

    Ok(())
}

pub fn get_imag_url(meta_json: &str) -> Option<String> {
    let nft_meta: NFTData = match serde_json::from_str(meta_json) {
        Ok(s) => s,
        Err(_) => {
            println!("无法将元数据转化为可用的数据类型");
            return None;
        }
    };
    match nft_meta.image {
        Some(s) => Some(s),
        None => None,
    }
}

pub fn parse_json(text: &str) -> String {
    let start_index = text.find("{").expect("未找到{");
    let end_index = text.rfind("}").expect("未找到}");
    let json_text = &text[start_index..end_index + 1];
    json_text.to_string()
}

//
fn convert_to_https(input: &str) -> Option<String> {
    if input.starts_with("https://") || input.starts_with("http://") {
        return Some(input.to_string());
    }
    if !input.starts_with("ipfs://") {
        return None;
    }

    let https_url = format!("https://ipfs.io/ipfs/{}", &input[7..]);
    Some(https_url)
}
