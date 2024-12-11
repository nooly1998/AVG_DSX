pub mod image_factory {
    use crate::utils::image_utils::is_supported_image_format;
    use std::{env, fs};

    /// 获取目录下所有图片文件的文件名
    pub fn get_image_filenames() -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let dir_path = env::current_dir()?.join("assets/images");
        let mut filenames = Vec::new();

        // 遍历目录中的所有文件
        for entry in fs::read_dir(dir_path)? {
            let entry = entry?;
            let path = entry.path();

            // 检查是否为文件以及是否为支持的图片格式
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if is_supported_image_format(ext) {
                        if let Some(filename) = path.file_name() {
                            if let Some(filename_str) = filename.to_str() {
                                filenames.push(filename_str.to_string());
                            }
                        }
                    }
                }
            }
        }

        Ok(filenames)
    }
}
