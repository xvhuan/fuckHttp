#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("icon.ico");
    res.set("ProductName", "fuckHttp Browser Selector");
    res.set("FileDescription", "Browser selector for intercepted URLs");
    res.set("CompanyName", "iusTech.");
    res.set("LegalCopyright", "Copyright (c) 2025 iusTech.");
    res.set("ProductVersion", "0.1.0");
    res.set("FileVersion", "0.1.0");
    res.compile().unwrap();
}

#[cfg(not(windows))]
fn main() {
    // 非Windows平台不需要资源文件
}