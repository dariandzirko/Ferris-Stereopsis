<div align="center">

# generic_camera_feed_display
---
</div>

The short term goal of this repo is to test and incorporate a somewhat safe Realsense wrapper (that I also wrote and generated -> <https://github.com/dariandzirko/realsense_wrapper>) written in Rust into something that can be visualised and used by other people. It is even better that I got a non-game focused project working in Bevy showing that this game engine is more versatile than people are exploring.

These repos are currently a work in progress and if someone wants to use them, they will have to edit the build.rs file and include their own proper path to the necessary files (being Intel Realsense headers and shared object file). Feel free to open an issue to correct me, report a bug, or suggest changes.

## How it Works 
---

I will talk to how both projects are working in tandem as I eventually ended up using this repo as the binary for running the code of my wrapper repo.

### Bindings


 Starting with the bottom there is the realsense .h and .so files that are supplied to the bindgen crate to generate a `bindings.rs` file that wraps all of the C functions in Rust code. It is unsafe Rust, that I then had to take care of so that it is even remotely usable, and there is still a lot of unsafe code at the moment. 

 ### Cleaning up 


Of course it is not all completely unsafe free but there has been a lot of progress. A nice and easy but extremely useful pattern has been wrapping the pointers from C in Rust structs and implementing Drop for all of those structs. Making pointer management a lot easier and code writing a lot safer. Probably the main reason why I fixed any null pointer exceptions and seg faults

### Bevy-ify

This part was neat but super completely intense as I had played around with Bevy before. My main ideas were copying examples for hot-loading assets and just hot-loading my images generated from the realsense data. There were other handshakes I had to make, like making Resources that had the C pointers deep underneath, and a queue for the images being pulled as I didn't want the code to wait for an image to load. Also making sure that all the data pulled from the realsense was valid and if it was using the formatting data to convert the bytes to an image type I could attach to a `Handle<Image>` in Bevy. 

## Repo Direction
---

I would love for this project to be workable for local video recordings, other camera types, and eventually able to display a point cloud in 3d.


## Proof
---

Here is my magnum opus of screen recording

![Demo](https://github.com/dariandzirko/generic_camera_feed_display/blob/main/demo/2023-04-26%2008-01-15.gif)

### Goals
---

- [x] Get stable fps on video feed
- [ ] Increase fps closer to realsense spec of 30
- [ ] Remove most unsafe code
- [ ] Write generic camera object pattern
- [ ] Read local mp4 and display
- [ ] Testing pipeline