use crate::FeedImage;
use bevy::prelude::*;
use realsense_wrapper::*;

// Make this buffer a buffer of ImageData?
// need to call pull_frame().get_curr_frame().to_image()
#[derive(Resource)]
pub struct FrameBufferResource {
    pub buffer: FrameBuffer,
}

impl FrameBufferResource {
    pub fn new() -> Self {
        FrameBufferResource {
            buffer: FrameBuffer::new(),
        }
    }
}

#[derive(Resource)]
pub struct RealsenseResource {
    pub realsense: RealsenseInstance,
}

impl RealsenseResource {
    pub fn new() -> Self {
        RealsenseResource {
            realsense: RealsenseInstance::new(),
        }
    }
}

pub struct RealsensePlugin;

impl Plugin for RealsensePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_frame_buffer)
            .add_system(update_display_system);
    }
}

pub fn realsense_start_system(realsense: ResMut<RealsenseResource>) {
    let stream_index = 0;
    let width = 640;
    let height = 480;
    let fps = 30;
    let stream = rs2_stream_RS2_STREAM_COLOR;
    let format = rs2_format_RS2_FORMAT_RGB8;

    unsafe {
        let mut error = std::ptr::null_mut::<rs2_error>();

        rs2_config_enable_stream(
            realsense.realsense.config,
            stream,
            stream_index,
            width,
            height,
            format,
            fps,
            &mut error,
        );

        let pipeline_profile = rs2_pipeline_start_with_config(
            realsense.realsense.pipeline,
            realsense.realsense.config,
            &mut error,
        );
        check_error(error);
    }
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
    image.texture = images.get_handle(&handle);
    //images.clear();
}

pub fn update_frame_buffer(
    mut frame_buffer: ResMut<FrameBufferResource>,
    mut realsense: ResMut<RealsenseResource>,
) {
    frame_buffer.buffer.pull_frame(&mut realsense.realsense);
}
