use log::info;
use sokol::{app as sapp, gfx as sg, glue as sglue};
use std::ffi;

pub trait AppState {
    fn init(&mut self);
    fn frame(&mut self, delta_time: f32) -> sg::Color;
    fn cleanup(&mut self);
}

pub struct AppConfig {
    pub window_title: String,
    pub width: i32,
    pub height: i32,
    pub sample_count: i32,
}

pub fn run<T: AppState + 'static>(config: AppConfig, state: T) {
    let user_data = Box::into_raw(Box::new((state, sg::PassAction::new()))) as *mut ffi::c_void;

    let title = std::ffi::CString::new(config.window_title).unwrap();

    let desc = sapp::Desc {
        init_userdata_cb: Some(init::<T>),
        frame_userdata_cb: Some(frame::<T>),
        cleanup_userdata_cb: Some(cleanup::<T>),
        user_data,
        window_title: title.as_ptr(),
        width: config.width,
        height: config.height,
        sample_count: config.sample_count,
        logger: sapp::Logger {
            func: Some(sokol::log::slog_func),
            ..Default::default()
        },
        icon: sapp::IconDesc {
            sokol_default: true,
            ..Default::default()
        },
        ..Default::default()
    };

    sapp::run(&desc);
}

extern "C" fn init<T: AppState>(user_data: *mut ffi::c_void) {
    let (state, pass_action) = unsafe { &mut *(user_data as *mut (T, sg::PassAction)) };

    sg::setup(&sg::Desc {
        environment: sglue::environment(),
        logger: sg::Logger {
            func: Some(sokol::log::slog_func),
            ..Default::default()
        },
        ..Default::default()
    });

    pass_action.colors[0] = sg::ColorAttachmentAction {
        load_action: sg::LoadAction::Clear,
        ..Default::default()
    };

    state.init();

    let backend = sg::query_backend();

    info!("using '{:?}' backend", backend);
}

extern "C" fn frame<T: AppState>(user_data: *mut ffi::c_void) {
    let (state, pass_action) = unsafe { &mut *(user_data as *mut (T, sg::PassAction)) };

    let clear_color = state.frame(1.0 / 60.0);
    pass_action.colors[0].clear_value = clear_color;

    sg::begin_pass(&sg::Pass {
        action: *pass_action,
        swapchain: sglue::swapchain(),
        ..Default::default()
    });
    sg::end_pass();
    sg::commit();
}

extern "C" fn cleanup<T: AppState>(user_data: *mut ffi::c_void) {
    let (mut state, _) = *unsafe { Box::from_raw(user_data as *mut (T, sg::PassAction)) };
    state.cleanup();
    sg::shutdown();
}
