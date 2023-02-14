> 本脚本用于爬取OPensea交易所上整套项目的元数据和IMG,使用脚本带来的任何风险与本作者无关,您下载本脚本的同时就意味着您赞同这一点.
> 有什么问题请联系QQ:756423901
> 如果这个脚本对您产生了帮助,如果您愿意的话,可以对我进行捐赠(XCH):xch1qysvhalp7f6xlecxcp0xmx4dsjx3zw8y669ef7lr6yy4z6948qrsazwa4a
> 本脚本MIT开源,您可对脚本进行任何形式的借鉴,修改或者进行任何商业化行为.

## 本脚本使用教程
#### 一.启动方式
1. 首先要下载编译后的文件:ops_plagiary.exe,因为opensea上大多数NFT的访问都需要代理,而他们的图片和元数据也大多数需要代理,所以建议国内用户提前开代理.
2. 双击ops_plagiary.exe可以启动这个脚本,但是推荐您使用下面的方式.
3. 打开CMD,通过cd指令到ops_plagiary.exe所在目录下,然后
```
.\ops_plagiary.exe
```
就可以启动本软件了,这种方式有助于找到错误,因为这种方式在脚本运行出错后驻留CMD指令窗口,方便根据提示自行查找原因或者联系作者,发送线索.
#### 二.内部设置
本脚本可以对大多数Opensea上的NFT项目进行下载元数据和IMG(支持png/jpg/img/webp)格式,但下载之前您需要了解以下知识点
1. 关于NFT元数据存储链接的形式
在脚本最开始,会询问用户采用计划爬取的NFT元数据存储链接的形式,本脚本一共支持两种形式,第一种是有规律的类型,第二种是无规律的类型.(第三种是base64类型,目前本脚本不支持).这是针对于他们的元数据链接而言的,那么我们如何知道一个NFT的元数据链接是否有规律呢?
2. 如何确定一个NFT的元数据链接是否有规律
* 我们打开一个NFT的项目首页: https://opensea.io/zh-CN/collection/doodles-official
* 我们随便打开一个它里面的项目: https://opensea.io/zh-CN/assets/ethereum/0x8a90cab2b38dba80c64b7734e58ee1db38b8992e/2358
* 在图片下方的"详情"中,点开它的"合约地址",它会进行一个跳转到合约浏览器中: https://etherscan.io/address/0x8a90cab2b38dba80c64b7734e58ee1db38b8992e
* 新弹出的网页中我们依次点击网页中部的:"Contract"->"ReadContract"(或者"Read As Proxy)
* 然后我们在下方找到"token_url"或者"url"类的字样,点开.(这里是我们查询当前这张NFT的元数据链接的一个入口,用来帮助我们判断它是哪种类型的链接.)
* 在输入框内输入:"1",点击Query,下方将出现字样: ipfs://QmPMc4tcBsMqLRuCQtPmPe84bpSjrC3Ky7t3JWuHXYB4aS/1
* 当我们输入2的时候,它会变成: ipfs://QmPMc4tcBsMqLRuCQtPmPe84bpSjrC3Ky7t3JWuHXYB4aS/2
> 由此可以看出它是有规律的
在opensea上大多数元数据链接都是有规律的,并且token_id与元数据链接尾部的数字一一对应
* 还有另外一种类型是无规律的,这个目前只支持AR类型的存储,可以参照上面的方式查看这个NFT项目来做理解:https://opensea.io/zh-CN/assets/ethereum/0x52de2cbad65d709631e5245dbc92a04c0c0de49f/1
> 接下来软件询问我们输入根地址或者合约地址(仅针对AR类型的无规律)

3. 什么是根地址和合约地址
* 在上面的第一个例子中,我们在输入1的时候,它的返回的链接是:ipfs://QmPMc4tcBsMqLRuCQtPmPe84bpSjrC3Ky7t3JWuHXYB4aS/1,那么它的根地址就是: ipfs://QmPMc4tcBsMqLRuCQtPmPe84bpSjrC3Ky7t3JWuHXYB4aS/
* 再举个例子,在其他的NFT中很多不是以ipfs开头,而是以https开头的,设置也十分雷同;假设有另外一个项目,当我们在token_id中输入1时,它返回的是: https://api.hello/meta/1 ,那么它的根地址就是: https://api.hello/meta/
> 记得后面一定有一个"/".
* 在无规律的AR类型地址中,脚本会让我们输入合约地址,合约地址更简单,以我们第一个例子为例,我们看到它的合约浏览器地址为: https://etherscan.io/address/0x8a90cab2b38dba80c64b7734e58ee1db38b8992e ,那么它的合约地址就是 0x8a90cab2b38dba80c64b7734e58ee1db38b8992e
3. 关于最大NFT的结尾编号
* 这是一个NFT项目中最大的NFT编号,我们还是以第一个例子为例: https://opensea.io/zh-CN/collection/doodles-official ,打开后,我们会发现在左侧的搜索栏中,会显示可搜索的总量是"10,000项结果"(英文版本是:"10,000 results"),因此我们可以确信它有10000个NFT,但我们最好还是验证以下为好.
* 我们按照2中所述的步骤,来到这个项目的合约浏览器中:https://etherscan.io/address/0x8a90cab2b38dba80c64b7734e58ee1db38b8992e,然后按照2中所述的步骤,在token_url中输入10000,它返回了"Error: Returned error: execution reverted: ERC721Metadata: URI query for nonexistent token",这意味着它的最大NFT编号并不是10000;我们试试9999,它返回了:ipfs://QmPMc4tcBsMqLRuCQtPmPe84bpSjrC3Ky7t3JWuHXYB4aS/9999,因此我们得出结论,它的最大编号是9999

#### 三.目前存在的问题
* 本脚本默认从NFT编号1开始下载,因此对于部分存在0编号的NFT,0编号的元数据和img文件(支持png/jpg/img/webp)会被漏掉.
* 本脚本目前不支持极其少数的元数据链接中带有.json的NFT项目;例如查询后发现编号1的NFT元数据链接为ipfs://QmPMc4tcBsMqLRuCQtPmPe84bpSjrC3Ky7t3JWuHXYB4aS/1.json,这种情况下脚本是无法下载的,因为修改这个小毛病会改变本脚本的函数式结构,且这种NFT的数量并不大,因此并未对其进行支持.
* 本脚本不支持少数编号不规则的NFT项目,例如胡乱编排编号的垃圾NFT项目.
* 本脚本不支持极少数https:base64元数据链接,这部分代码并不会太难,但是比较繁琐,懒得写了,有需要的单独联系吧.

> 如果您有任何好的想法,欢迎与我联系.