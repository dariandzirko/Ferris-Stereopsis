<div align="center">

# generic_camera_feed_display
---
</div>

The short term goal of this repo is to test and incorporate a somewhat safe Realsense wrapper (that I also wrote and generated -> <https://github.com/dariandzirko/realsense_wrapper>) written in Rust into something that can be visualised and used by other people. It is a really interesting project because I got a non-game focused project working with Bevy. Showing the other possibilities of the game engine.

These repos are currently a work in progress and if someone wants to use them, they will have to edit the build.rs file and include their own proper path to the necessary files (being Intel Realsense headers and shared object file). Feel free to open an issue to correct me, report a bug, or suggest changes.

## How it Works 
---

I will talk to how both projects are working in tandem as I eventually ended up using this repo as the binary for running the code of my wrapper repo.

### Bindings

 Starting with the bottom there is the realsense .h and .so files that are supplied to the bindgen crate to generate a `bindings.rs` file that wraps all of the C functions in Rust code. The generated bindings are unsafe Rust code but that is still safer and more user friendly than C.
 
 ### Cleaning up 

The current code state is not completely unsafe free but the exposed user API is almost unsafe free. The most useful pattern that helped me remove this unsafe code was wrapping the pointers from C in Rust structs and then implementing the Drop trait for all of those structs. Making pointer management a lot easier and code writing a lot safer. This leads to the drop function being called when the structs go out of scope, which will call the proper free function for the pointers. That pattern was the main reason for fixing my seg faults and null pointer issues.

### Bevy-ify

This part was neat but not extremely complex as I had played around with Bevy before. My main ideas were copying examples for hot-loading assets and just hot-loading my images generated from the realsense data. I made some handshakes with Bevy like Resources that had the C pointers deep underneath, and a queue for the images being pulled as I didn't want the code to wait for an image to load. Also I needed to make sure that all the data pulled from the realsense was valid. If it was valid then use the formatting data from frame to convert the bytes to an image type I could attach to a `Handle<Image>` in Bevy. 

## Repo Direction
---

I would love for this project to be workable for local video recordings, other camera types, and eventually able to display a point cloud in 3d.


## Proof
---

Here is my magnum opus of screen recording

![Demo](https://github.com/dariandzirko/generic_camera_feed_display/blob/main/demo/30fps_realsense.gif)

### Goals
---

- [x] Get stable fps on video feed
- [ ] Increase fps closer to realsense spec of 30
- [ ] Remove most unsafe code
- [ ] Write generic camera object pattern
- [ ] Read local mp4 and display
- [ ] Testing pipeline