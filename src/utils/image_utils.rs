use image::imageops::FilterType;
use image::ImageFormat;
use std::path::Path;

pub fn resize_image(
    input_path: &str,
    width: u32,
    height: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    // 加载图片
    let img = image::open(&Path::new(input_path))?;

    // 获取输入路径的扩展名以正确保存文件格式
    let extension = Path::new(input_path)
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();

    // 根据扩展名判断并设置输出文件格式
    let format = match extension.as_str() {
        "jpg" | "jpeg" => ImageFormat::Jpeg,
        "png" => ImageFormat::Png,
        // 添加其他需要支持的格式
        "bmp" => ImageFormat::Bmp,
        "gif" => ImageFormat::Gif,
        _ => return Err("Unsupported file format.".into()),
    };

    // 构建输出文件路径
    let output_path = format!(
        "{}_resized.{}",
        input_path
            .trim_end_matches(&extension)
            .trim_end_matches('.'),
        extension
    );

    // 调整大小
    let resized = img.resize(width, height, FilterType::Nearest);

    // 保存调整后的图片，指定格式
    resized.save_with_format(&Path::new(&output_path), format)?;

    Ok(())
}

pub fn is_supported_image_format(ext: &std::ffi::OsStr) -> bool {
    match ext.to_str() {
        Some("png") | Some("jpg") | Some("jpeg") | Some("bmp") | Some("gif") | Some("tiff")
        | Some("webp") => true,
        _ => false,
    }
}
