use std::{
    sync::{Arc, Mutex},
    thread::spawn,
};

use ops_plagiary::*;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("鸣谢本开源项目创始捐赠人:徐龙先生");
    println!("本脚本唯一出品方:中国数字党");
    println!("中国数字党QQ交流群:369056311(春耕)");
    println!("本脚本遵循MIT协议,开源免费,您可对本脚本进行任何形式的修改或进行任何形式的商业化行为,但请注明出处!");
    println!("本脚本支持捐赠形式的定制化开源服务,本人QQ:756423901");
    println!(
        "若该脚本对您有所帮助,如果您愿意的话,可以进行捐赠(XCH):xch1qysvhalp7f6xlecxcp0xmx4dsjx3zw8y669ef7lr6yy4z6948qrsazwa4a"
    );
    println!("熊市当头,请谨慎捐赠,一经捐赠,概不退还!");
    println!("\n\n");
    let url_type = get_url_type();
    let root_dir = make_nft_dir();
    let (meta_urls, thread_num) = get_meta_urls(&url_type);

    let id = Arc::new(Mutex::new(0));
    let meta_urls = Arc::new(Mutex::new(meta_urls));
    let root_dir = Arc::new(Mutex::new(root_dir));
    let fail_ids = Arc::new(Mutex::new(vec![]));

    let mut handles = vec![];
    for i in 0..thread_num {
        let id = Arc::clone(&id);
        let meta_urls = Arc::clone(&meta_urls);
        let root_dir = Arc::clone(&root_dir);
        let fail_ids = Arc::clone(&fail_ids);

        let handle = spawn(move || loop {
            // 上锁
            let mut id = id.lock().unwrap();
            let meta_urls = meta_urls.lock().unwrap();
            let root_dir = root_dir.lock().unwrap();

            // 解锁id
            let current_index = *id;
            (*id) += 1;
            drop(id);

            // 解锁root_dir
            let dir_clone = (*root_dir).clone();
            drop(root_dir);

            // 解锁meta_urls
            let current_url = match meta_urls.get(current_index) {
                Some(s) => s.clone(),
                None => break,
            };
            drop(meta_urls);

            match download_nft(current_index + 1, &current_url, &dir_clone) {
                Ok(_) => println!(
                    "线程{}:<NFT编号:{}任务>{}下载成功",
                    i,
                    current_index + 1,
                    current_url
                ),
                Err(_) => {
                    println!(
                        "线程{}:检测到下载编号{}的NFT,100次链接失败,该任务将跳过,本线程将进行其他队列任务...",
                        i,
                        current_index + 1
                    );
                    // 记录错误编号
                    let mut fail_ids = fail_ids.lock().unwrap();
                    fail_ids.push(current_index + 1);
                    drop(fail_ids);

                    continue;
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    // 查看是否有失败的

    // 结束后清理下载失败的残留文件,确保元数据文件和img文件一一对应
    let fail_ids = fail_ids.lock().unwrap();
    let root_dir = root_dir.lock().unwrap();
    if !fail_ids.is_empty() {
        println!("开始清理因下载失败而废弃的垃圾文件,确保元数据与图片一一对应...");
        for fail in fail_ids.iter() {
            let fail_meta_path = format!("{}/meta/{fail}", *root_dir);
            if let Ok(_) = std::fs::remove_file(&fail_meta_path) {
                println!("{}删除成功", fail_meta_path);
            };
        }
        println!("废弃文件,已经清理完成");
    }
    println!("运行结束,按任意键退出..");
    let mut out = String::new();
    std::io::stdin().read_line(&mut out).unwrap();

    Ok(())
}
