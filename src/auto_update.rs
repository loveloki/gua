use anyhow::{Ok, Result};
use gpui::{App, AppContext, AsyncApp, Context, Entity, Global, SemanticVersion, Task, Window};
use serde::Deserialize;
use std::{
    env::consts::{ARCH, OS},
    str::FromStr,
    time::Duration,
};

#[derive(Deserialize, Debug)]
pub struct GithubRelease {
    pub tag_name: String,
    pub assets: Vec<GithubReleaseAsset>,
}

#[derive(Deserialize, Debug)]
pub struct GithubReleaseAsset {
    pub name: String,
    pub browser_download_url: String,
    pub digest: Option<String>,
}

/**
 * 检查更新类型
 */
pub enum UpdateCheckType {
    Automatic,
    Manual,
}

/**
 * github 最新发布链接
 */
const RELEASE_URL: &'static str = "https://api.github.com/repos/loveloki/gua/releases/latest";

/**
 * 软件版本
 */
pub const CURRENT_VERSION: &'static str = env!("CARGO_PKG_VERSION");

/**
 * 轮询间隔
 */
const POLL_INTERVAL: Duration = Duration::from_secs(60 * 60);

#[derive(Default)]
struct GlobalAutoUpdate(Option<Entity<AutoUpdater>>);

impl Global for GlobalAutoUpdate {}

pub fn init(cx: &mut App) {
    let auto_update = cx.new(|cx| {
        let current_version = SemanticVersion::from_str(CURRENT_VERSION).unwrap();

        println!("当前应用版本为： {CURRENT_VERSION}");

        let updater = AutoUpdater::new(current_version);

        // 暂时禁用自动更新，等异步请求处理后再开启
        // updater.start_polling(cx).detach();

        updater
    });

    cx.set_global(GlobalAutoUpdate(Some(auto_update)));

    println!("当前应用版本为： {CURRENT_VERSION}")
}

pub struct AutoUpdater {
    current_version: SemanticVersion,
    /**
     * 正在进行中的轮询任务
     */
    pending_poll: Option<Task<Option<()>>>,
    update_info: Option<(SemanticVersion, String)>,
}

impl AutoUpdater {
    /**
     * 获取实例
     */
    pub fn get(cx: &mut App) -> Option<Entity<Self>> {
        cx.default_global::<GlobalAutoUpdate>().0.clone()
    }

    /**
     * 获取当前版本
     */
    pub fn current_version(&self) -> SemanticVersion {
        self.current_version
    }

    /**
     * 获取更新信息
     */
    pub fn update_info(&self) -> Option<(SemanticVersion, String)> {
        self.update_info.clone()
    }

    pub fn new(current_version: SemanticVersion) -> Self {
        Self {
            current_version,
            pending_poll: None,
            update_info: None,
        }
    }

    /**
     * 启动轮询
     */
    pub fn start_polling(&self, cx: &mut Context<Self>) -> Task<Result<()>> {
        println!("开始轮询！");

        cx.spawn(async move |this, cx| {
            loop {
                this.update(cx, |this, cx| this.poll(UpdateCheckType::Automatic, cx))?;
                cx.background_executor().timer(POLL_INTERVAL).await
            }
        })
    }

    /**
     * 检查更新
     */
    pub fn check(_: &mut Window, cx: &mut App) {
        if let Some(updater) = AutoUpdater::get(cx) {
            updater.update(cx, |this, cx| {
                this.poll(UpdateCheckType::Manual, cx);
            })
        }
    }

    pub fn poll(&mut self, check_type: UpdateCheckType, cx: &mut Context<Self>) {
        println!(
            "执行 poll，类型：{}",
            match check_type {
                UpdateCheckType::Automatic => "自动",
                UpdateCheckType::Manual => "手动",
            }
        );
        // 有正在执行的就直接停止
        if self.pending_poll.is_some() {
            return;
        }

        cx.notify();

        self.pending_poll = Some(cx.spawn(async move |this, cx| {
            let result = Self::update(this.upgrade()?, cx).await;

            this.update(cx, |this, cx| {
                this.pending_poll = None;
                // TODO 处理 result
            })
            .ok()
        }));
    }

    /**
     * 更新应用
     */
    pub async fn update(this: Entity<Self>, cx: &mut AsyncApp) -> Result<()> {
        println!("执行 update");
        let current_version = this.read_with(cx, |this, _| this.current_version)?;

        let release = ureq::get(RELEASE_URL)
            .call()?
            .body_mut()
            .read_json::<GithubRelease>()?;

        let tag_name = release.tag_name.clone().split_off(1);

        let remote_version = SemanticVersion::from_str(&tag_name)?;

        println!("remote_version {}", remote_version);

        let is_newer = remote_version > current_version;

        println!(
            "当前版本：{current_version}，服务器版本：{remote_version}，是否需要更新：{is_newer}"
        );

        if is_newer {
            let filename = get_download_filename()?;

            let browser_download_url = release
                .assets
                .iter()
                .find(|asset| asset.name == filename)
                .unwrap()
                .browser_download_url
                .clone();

            println!("下载地址为： {browser_download_url}");

            this.update(cx, |this, cx| {
                this.update_info = Some((remote_version, browser_download_url));

                cx.notify();
            })?;
        }

        Ok(())
    }
}

/**
 * 根据操作系统和架构获取下载文件名
 */
fn get_download_filename() -> Result<String> {
    // let os = match OS {
    //     "macos" => Ok("gua"),
    //     "windows" => Ok("gua"),
    //     "linux" => Ok("gua"),
    //     unsupported_os => anyhow::bail!("not supported: {unsupported_os}"),
    // }?;

    print!("操作系统为：{OS}");

    let arch = match ARCH {
        "x86_64" => Ok("x86_64"),
        "aarch64" => Ok("aarch64"),
        unsupported_arch => anyhow::bail!("not supported: {unsupported_arch}"),
    }?;

    println!("架构：{arch}");

    let filename = match (OS, arch) {
        ("macos", "aarch64") => "gua-aarch64-apple-darwin.tar.xz",
        ("windows", "x86_64") => "gua-x86_64-pc-windows-msvc.zip",
        ("linux", "x86_64") => "gua-x86_64-unknown-linux-gnu.tar.xz",
        _ => anyhow::bail!("not supported: {OS}-{arch}"),
    };

    println!("文件名：{filename}");

    Ok(filename.into())
}
