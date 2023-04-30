use crate::{button_utilities::FormatSelectionResource, FeedImage};
use bevy::prelude::*;
use realsense_wrapper::{format::Rs2Format, stream::Rs2StreamKind, *};

pub struct RestartRealsenseEvent {
    pub does_exist: bool,
}

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

pub fn realsense_start_system(
    mut realsense: ResMut<RealsenseResource>,
    format: Res<FormatSelectionResource>,
) {
    let stream_index = 0;
    let width = 640;
    let height = 480;
    let fps = 30;
    let stream = format.stream;
    let format = format.format;

    realsense
        .realsense
        .stream_frames(stream_index, width, height, fps, stream, format);
}

pub fn update_display_system(
    entity_query: Query<(&FeedImage, &Children)>,
    mut image_query: Query<&mut UiImage>,
    mut images: ResMut<Assets<Image>>,
    mut frame_buffer: ResMut<FrameBufferResource>,
) {
    let (_flag, children) = entity_query.iter().next().unwrap();
    let child = children.iter().next().unwrap();
    let mut image = image_query.get_mut(*child).unwrap();

    let mut data = ImageData::default();
    if let Some(frame) = frame_buffer.buffer.get_curr_frame() {
        data = frame;
    }

    if let Some(image_data) = data.to_image() {
        let handle = images.add(Image::from_dynamic(image_data, true));
        image.texture = images.get_handle(&handle);
    }
}

pub fn update_frame_buffer(
    mut frame_buffer: ResMut<FrameBufferResource>,
    mut realsense: ResMut<RealsenseResource>,
) {
    frame_buffer.buffer.populate_queue(&mut realsense.realsense);
}

pub fn restart_realsense_system(
    mut events: EventReader<RestartRealsenseEvent>,
    mut realsense: ResMut<RealsenseResource>,
    format: Res<FormatSelectionResource>,
) {
    if !events.is_empty() {
        let stream_index = 0;
        let width = 640;
        let height = 480;
        let fps = 30;
        let stream = format.stream;
        let format = format.format;

        realsense.realsense = RealsenseInstance::new();

        realsense
            .realsense
            .stream_frames(stream_index, width, height, fps, stream, format);
    }
}
