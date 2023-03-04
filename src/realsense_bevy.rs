use std::process::Child;

use bevy::prelude::*;
use realsense_wrapper::*;

use crate::FeedImage;

const WIDTH: usize = 640;
const HEIGHT: usize = 480;
const FPS: u32 = 30;
const STREAM_INDEX: u32 = 0;

pub fn image_read_display_system(
    entity_query: Query<(&FeedImage, &Children)>,
    mut image_query: Query<&mut UiImage>,
    mut images: ResMut<Assets<Image>>,
) {
    unsafe {
        let mut error = std::ptr::null_mut::<realsense_wrapper::rs2_error>();

        let context = rs2_create_context(RS2_API_VERSION as i32, &mut error);
        check_error(error);

        let device_list = rs2_query_devices(context, &mut error);

        let device_count = rs2_get_device_count(device_list, &mut error);
        check_error(error);

        if device_count == 0 {
            println!("No devices connected");
        } else {
            println!("Device count is {}", device_count);
        }

        let device = rs2_create_device(device_list, 0, &mut error);
        check_error(error);
        print_device_info(device);

        let pipeline = rs2_create_pipeline(context, &mut error);
        check_error(error);

        let config = rs2_create_config(&mut error);
        check_error(error);

        rs2_config_enable_stream(
            config,
            rs2_stream_RS2_STREAM_COLOR,
            STREAM_INDEX as i32,
            WIDTH as i32,
            HEIGHT as i32,
            rs2_format_RS2_FORMAT_RGB8,
            FPS as i32,
            &mut error,
        );

        let pipeline_profile = rs2_pipeline_start_with_config(pipeline, config, &mut error);
        check_error(error);

        if let Some(_) = error.as_ref() {
            println!("Error with color streaming");
        }

        let frames = rs2_pipeline_wait_for_frames(pipeline, RS2_DEFAULT_TIMEOUT, &mut error);
        check_error(error);

        let num_of_frames = rs2_embedded_frames_count(frames, &mut error);
        check_error(error);

        for i in 0..num_of_frames {
            let frame = rs2_extract_frame(frames, i, &mut error);
            check_error(error);

            let frame_info = get_frame_info(frame); //should probably just give the struct the frame info and extract all the data
            check_error(error);

            println!("RGB frame arrived");
            let mut test_data = realsense_wrapper::ImageData::new(
                frame_info.format,
                frame_info.width as usize,
                frame_info.height as usize,
                frame_info.bits_per_pixel as usize,
                frame_info.stride as usize,
            );

            test_data.copy_data_from_frame(frame);
            rs2_release_frame(frame);

            let (_flag, children) = entity_query.iter().next().unwrap();
            let child = children.iter().next().unwrap();
            let mut image = image_query.get_mut(*child).unwrap();
            let handle = images.add(Image::from_dynamic(test_data.to_image(), true));
            image.0 = images.get_handle(&handle);
            images.clear();
        }

        rs2_release_frame(frames);

        rs2_pipeline_stop(pipeline, &mut error);
        check_error(error);

        rs2_delete_pipeline_profile(pipeline_profile);
        rs2_delete_config(config);
        rs2_delete_pipeline(pipeline);
        rs2_delete_device(device);
        rs2_delete_device_list(device_list);
        rs2_delete_context(context);
    }
}
