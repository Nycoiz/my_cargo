/*
通过release profile来自定义构建
release profile
    是预定义的
    可自定义：可以使用不同的配置，对代码编译拥有更多的控制权
每个profile的配置都独立于其他的profile
cargo主要的两个proflie：
    dev profile
        用于开发阶段
        默认配置：debug=true，opt-level=0
    release profile
        用于发布阶段
        默认配置：debug=false，opt-level=3
自定义profile
    针对每个profile，cargo允许用户覆盖默认配置
    通过在Cargo.toml中添加[profile.<profile_name>]部分，在里面覆盖默认配置的子集
对于每个配置的默认值和完整选项，参见https://doc.rust-lang.org/cargo/reference/profiles.html
*/

/*
在crates.io上发布库（1）
可以通过发布包来共享你的代码
发布包的步骤：
    1. 创建一个库项目
    2. 在Cargo.toml中添加必要的元数据
    3. 编写文档注释
    4. 使用cargo publish命令发布包
crates的注册表在https://crates.io上
    它会分发已经注册的包的源代码
    主要托管开源的代码
文档注释：用于生成文档
    生成HTML文档，供用户浏览和参考
    显式公共API的文档注释，帮助用户理解如何使用你的API
    通过文档注释来为你的库编写文档
    文档注释以三斜杠（///）开头，位于函数、结构体、枚举等定义之前
    可以使用Markdown语法来格式化文档
    放置在被说明条目之前
使用cargo doc命令运行rustdoc工具生成文档
    生成的文档位于target/doc目录下
可以使用cargo doc --open命令通过浏览器打开index.html文件来查看文档
    该命令构建当前crate的文档（也包括crate依赖项的文档）
常用的文档注释标签：
    # Examples：提供使用示例
    # Panics：说明函数在什么情况下会panic
    # Errors：如果函数返回Result类型，说明函数可能的错误种类以及在什么情况下会返回错误
    # Safety：若函数处于unsafe块中，说明函数的不安全行为和使用要求
文档注释作为测试
    文档注释中的代码块可以被rustdoc工具提取出来，并作为测试运行
    通过在文档注释中的代码块前添加rust标记来指定代码块的语言
    例如：
    /// ```rust
    /// let result = my_function();
    /// assert_eq!(result, expected_value);
    /// ```
    运行cargo test命令时，rustdoc会提取文档注释中的rust代码块，并将其作为测试运行
    这有助于确保文档中的示例代码始终保持正确和可用
为了确保文档注释中的代码块能够正确编译和运行，可以使用以下标记来指定代码块的属性：
    - ignore：忽略该代码块，不进行编译和测试
    - no_run：编译该代码块，但不运行它
    - should_panic：期望该代码块在运行时会panic
    - compile_fail：期望该代码块在编译时会失败
为包含注释的项添加文档注释
    符号：//!
    位置：位于文件顶部，模块顶部，或函数顶部
    作用：为包含注释的项添加文档注释
    这类注释通常用于描述crate和模块的整体功能和用途
*/

/*
使用pub use导出方便使用的公共API
可以重新导出，创建一个与内部私有结构不同的对外公共结构 
*/

/*
在crates.io上发布库（2）
创建并设置crates.io账号
发布crate前，需要在crtaes.io创建账号并获得API token、
    该token用于认证和授权发布操作
    可以在crates.io的账户设置中找到API token
运行命令： cargo login <API token>
    该命令会将API token保存到本地配置文件~/.cargo/credentials中，以便后续使用
    认证成功后，cargo会提示登录成功
API token可以在crates.io撤销
在发布crate之前，确保满足以下条件：
    1. crate的名称在crates.io上是唯一的
    2. crate的版本号遵循语义化版本控制（SemVer）规范
    3. crate的Cargo.toml文件中包含必要的元数据，如name、version、authors等
    4. crate的代码已经准备好，并且通过了测试
    5. description字段提供了对crate的简要描述，会出现在crate搜索的结果里
    6. license字段指定了crate的许可证，多个license之间用OR分隔，明确用户在使用crate时的权利和义务，在http://spdx.org/licenses 上可以找到常用的许可证标识符
修改crate的版本号
    可以通过修改Cargo.toml中的version字段来更新crate的版本号,再重新发布
    版本号应该遵循语义化版本控制（SemVer）规范，格式为MAJOR.MINOR.PATCH
    例如，1.0.0表示第一个正式版本，1.1.0表示添加了新功能但保持向后兼容，1.0.1表示修复了bug但没有添加新功能
从crates.io上撤销crate
    可以通过运行cargo yank <version>命令来撤销特定版本的crate
    这会将该版本标记为不可用，但不会删除它的源代码
    已经依赖该版本的项目仍然可以使用它，但新项目将无法依赖该版本
    如果需要完全删除一个版本，可以联系crates.io管理员进行处理
yank意味着：
    所有已经产生Cargo.lock文件的项目仍然可以使用被yank的版本
    但是新项目将无法依赖被yank的版本
取消yank
    如果需要取消yank，可以运行cargo unyank <version>命令来取消yank特定版本的crate
    这会将该版本重新标记为可用，使新项目可以依赖它
*/

/*
通过workspace组织大项目
*/

/*
从crates.io安装库
*/

/*
使用自定义命令扩展cargo
*/

use my_cargo::kinds::PrimaryColor;
use my_cargo::utils::mix_colors;
fn main() 
{
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    let result = mix_colors(red, yellow);
}
