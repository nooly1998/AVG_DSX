pub mod global_define {
    use bevy::prelude::Resource;

    // 常用分辨率选项
    pub const RESOLUTION_720P: (f32, f32) = (1280.0, 720.0);
    pub const RESOLUTION_1080P: (f32, f32) = (1920.0, 1080.0);
    pub const RESOLUTION_1440P: (f32, f32) = (2560.0, 1440.0);
    pub const RESOLUTION_2160P: (f32, f32) = (3840.0, 2160.0);

    #[derive(Resource)]
    // 游戏全局配置
    pub struct GameConfig {
        pub resolution: (f32, f32),  // 屏幕分辨率
        pub title: String,          // 窗口标题
        pub msaa_samples: u32,      // 抗锯齿设置
        pub target_fps: u32,        // 目标帧率
    }

    impl Default for GameConfig {
        fn default() -> Self {
            GameConfig {
                resolution: RESOLUTION_720P,
                title: "Nooly's AVG Game".to_string(),
                msaa_samples: 4,  // 默认 4xMSAA
                target_fps: 60,   // 默认帧率 60FPS
            }
        }
    }
}
