use crate::FeedImage;
use bevy::prelude::*;
use realsense_wrapper::*;
use std::sync::{Arc, Mutex};

const WIDTH: usize = 640;
const HEIGHT: usize = 480;
const FPS: u32 = 30;
const STREAM_INDEX: u32 = 0;

// Make this buffer a buffer of ImageData?
// need to call pull_frame().get_curr_frame().to_image()
#[derive(Resource)]
pub struct FrameBufferResource {
    pub buffer: Arc<Mutex<FrameBuffer>>,
}

pub struct RealsensePlugin;

impl Plugin for RealsensePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FrameBufferResource::new())
            .add_startup_system(realsense_start_system)
    }
}

pub fn realsense_start_system(frame_buffer: ResMut<FrameBufferResource>) {
    let mut realsense = RealsenseInstance::new();

    let mut frame_buffer = FrameBuffer::new();

    let stream_index = 0;
    let width = 640;
    let height = 480;
    let fps = 30;
    let stream = rs2_stream_RS2_STREAM_COLOR;
    let format = rs2_format_RS2_FORMAT_RGB8;

    unsafe {
        rs2_config_enable_stream(
            realsense.config,
            stream,
            stream_index,
            width,
            height,
            format,
            fps,
            &mut realsense.error,
        );

        let pipeline_profile = rs2_pipeline_start_with_config(
            realsense.pipeline,
            realsense.config,
            &mut realsense.error,
        );
        check_error(realsense.error);
    }

    frame_buffer = frame_buffer;
}

pub fn update_display_system(
    entity_query: Query<(&FeedImage, &Children)>,
    mut image_query: Query<&mut UiImage>,
    mut images: ResMut<Assets<Image>>,
    frame_buffer: Res<FrameBufferResource>,
) {
    let (_flag, children) = entity_query.iter().next().unwrap();
    let child = children.iter().next().unwrap();
    let mut image = image_query.get_mut(*child).unwrap();
    let handle = images.add(Image::from_dynamic(
        frame_buffer.buffer.get_curr_frame().to_image(),
        true,
    ));
    image.0 = images.get_handle(&handle);
    images.clear();
}
