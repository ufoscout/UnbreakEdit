use backend_common::error::Error;
use crossbeam_channel::Receiver;
use glib::{Cast, ObjectExt};
use gstreamer::prelude::ElementExtManual;
use gstreamer_app::prelude::{ElementExt, PadExt, GstBinExt};
use num_rational::Rational32;

pub struct MediaContainer {
    // bus: gstreamer::Bus,
    source: gstreamer::Bin,

    width: u32,
    height: u32,
    framerate: Rational32,
    duration: std::time::Duration,
    frame_receiver: Receiver<Vec<u8>>,
    muted: bool,
    looping: bool,
    is_eos: bool,
    restart_stream: bool,
}

impl Drop for MediaContainer {
    fn drop(&mut self) {
        self.source
            .set_state(gstreamer::State::Null)
            .expect("failed to set state");
    }
}

impl MediaContainer {

    /// Creates a new media container.
    pub fn new(uri: &url::Url, live: bool) -> Result<Self, Error> {
        let source = gstreamer::parse_launch(&format!("playbin uri=\"{}\" video-sink=\"videoconvert ! videoscale ! appsink name=app_sink caps=video/x-raw,format=RGBA,pixel-aspect-ratio=1/1\"", uri.as_str())).unwrap();
        let source = source.downcast::<gstreamer::Bin>().unwrap();

        let video_sink: gstreamer::Element = source.property("video-sink");
        let pad = video_sink.pads().get(0).cloned().unwrap();
        let pad = pad.dynamic_cast::<gstreamer::GhostPad>().unwrap();
        let bin = pad
            .parent_element()
            .unwrap()
            .downcast::<gstreamer::Bin>()
            .unwrap();

        let app_sink = bin.by_name("app_sink").unwrap();
        let app_sink = app_sink.downcast::<gstreamer_app::AppSink>().unwrap();

        let (frame_sender, frame_receiver) = crossbeam_channel::bounded(5);

        app_sink.set_callbacks(
            gstreamer_app::AppSinkCallbacks::builder()
                .new_sample(move |sink| {
                    let sample = sink.pull_sample().map_err(|_| gstreamer::FlowError::Eos)?;
                    let buffer = sample.buffer().ok_or(gstreamer::FlowError::Error)?;
                    let map = buffer.map_readable().map_err(|_| gstreamer::FlowError::Error)?;

                    let pad = sink.static_pad("sink").ok_or(gstreamer::FlowError::Error)?;

                    let caps = pad.current_caps().ok_or(gstreamer::FlowError::Error)?;
                    let s = caps.structure(0).ok_or(gstreamer::FlowError::Error)?;
                    let width = s.get::<i32>("width").map_err(|_| gstreamer::FlowError::Error)?;
                    let height = s.get::<i32>("height").map_err(|_| gstreamer::FlowError::Error)?;

                    // let thread_id = std::thread::current().id();
                    //println!("thread_id {:?} - video callback: {}x{}", thread_id, width, height);

                    if !frame_sender.is_full() {
                        match frame_sender.try_send(map.as_slice().to_owned()) {
                            Ok(_) => {
                                println!("sent frame in the channel");
                            },
                            Err(err) => {
                                println!("failed to send frame in the channel: {}", err);
                            }
                        }
                    }

                    Ok(gstreamer::FlowSuccess::Ok)
                })
                .build(),
        );

        source.set_state(gstreamer::State::Playing).map_err(|_| Error::MediaStateChangeError("".to_owned())).unwrap();

        // wait for up to 5 seconds until the decoder gets the source capabilities
        source.state(gstreamer::ClockTime::from_seconds(5)).0.unwrap();

        // extract resolution and framerate
        let caps = pad.current_caps().ok_or(Error::MediaCapsError("".to_owned())).unwrap();
        let s = caps.structure(0).ok_or(Error::MediaCapsError("".to_owned())).unwrap();
        let width = s.get::<i32>("width").map_err(|_| Error::MediaCapsError("".to_owned())).unwrap();
        let height = s.get::<i32>("height").map_err(|_| Error::MediaCapsError("".to_owned())).unwrap();
        let framerate = s
            .get::<gstreamer::Fraction>("framerate")
            .map_err(|_| Error::MediaCapsError("".to_owned())).unwrap();

        let duration = if !live {
            std::time::Duration::from_nanos(
                source
                    .query_duration::<gstreamer::ClockTime>()
                    .unwrap()
                    .nseconds(),
            )
        } else {
            std::time::Duration::from_secs(0)
        };

        Ok(MediaContainer {
            // bus: source.bus().unwrap(),
            source,

            width: width as _,
            height: height as _,
            framerate: framerate.into(),
            duration,

            frame_receiver,
            // wait,
            muted: false,
            looping: false,
            is_eos: false,
            restart_stream: false,
        })
    }

        /// Get the size/resolution of the video as `(width, height)`.
        #[inline(always)]
        pub fn size(&self) -> (u32, u32) {
            (self.width, self.height)
        }
    
        /// Get the framerate of the video as frames per second.
        #[inline(always)]
        pub fn framerate(&self) -> Rational32 {
            self.framerate
        }
    
        /// Set the volume multiplier of the audio.
        /// `0.0` = 0% volume, `1.0` = 100% volume.
        ///
        /// This uses a linear scale, for example `0.5` is perceived as half as loud.
        pub fn set_volume(&mut self, volume: f64) {
            self.source.set_property("volume", &volume);
        }
    
        /// Set if the audio is muted or not, without changing the volume.
        pub fn set_muted(&mut self, muted: bool) {
            self.muted = muted;
            self.source.set_property("mute", &muted);
        }
    
        /// Get if the audio is muted or not.
        #[inline(always)]
        pub fn muted(&self) -> bool {
            self.muted
        }
    
        /// Get if the stream ended or not.
        #[inline(always)]
        pub fn eos(&self) -> bool {
            self.is_eos
        }
    
        /// Get if the media will loop or not.
        #[inline(always)]
        pub fn looping(&self) -> bool {
            self.looping
        }
    
        /// Set if the media will loop or not.
        #[inline(always)]
        pub fn set_looping(&mut self, looping: bool) {
            self.looping = looping;
        }
    
        /// Set if the media is paused or not.
        pub fn set_paused(&mut self, paused: bool) {

            // println!("start set state to paused: {paused}. Is lock? {}",self.source.is_locked_state());
            self.source
                .set_state(if paused {
                    gstreamer::State::Paused
                } else {
                    gstreamer::State::Playing
                })
                .unwrap(/* state was changed in ctor; state errors caught there */);
            // self.paused = paused;
    
            // println!("done set state to paused: {paused}");

            // Set restart_stream flag to make the stream restart on the next Message::NextFrame
            if self.is_eos && !paused {
                self.restart_stream = true;
            }

            // println!("completed set state to paused: {paused}");
        }
    
        /// Get if the media is paused or not.
        #[inline(always)]
        pub fn paused(&self) -> bool {

            self.source.current_state() == gstreamer::State::Paused
            // self.paused/
        }
    
        /// Jumps to a specific position in the media.
        /// The seeking is not perfectly accurate.
        // pub fn seek(&mut self, position: impl Into<Position>) -> Result<(), Error> {
        //     let formatted_value = GenericFormattedValue::from(position.into());
        //     self.source
        //         .seek_simple(gst::SeekFlags::FLUSH, formatted_value)?;
        //     Ok(())
        // }
    
        /// Get the current playback position in time.
        pub fn position(&self) -> std::time::Duration {
            std::time::Duration::from_nanos(
                self.source
                    .query_position::<gstreamer::ClockTime>()
                    .map_or(0, |pos| pos.nseconds()),
            )
        }
    
        /// Get the media duration.
        #[inline(always)]
        pub fn duration(&self) -> std::time::Duration {
            self.duration
        }
    
        // /// Generates a list of thumbnails based on a set of positions in the media.
        // ///
        // /// Slow; only needs to be called once for each instance.
        // /// It's best to call this at the very start of playback, otherwise the position may shift.
        // pub fn thumbnails(&mut self, positions: &[Position]) -> Result<Vec<Handle>, Error> {
        //     let paused = self.paused();
        //     let pos = self.position();
        //     self.set_paused(false);
        //     let out = positions
        //         .iter()
        //         .map(|&pos| {
        //             self.seek(pos)?;
        //             self.wait.recv().map_err(|_| Error::Sync)?;
        //             Ok(self.frame_image())
        //         })
        //         .collect();
        //     self.set_paused(paused);
        //     self.seek(pos)?;
        //     out
        // }
    
        // pub fn update(&mut self, message: VideoPlayerMessage) -> Command<VideoPlayerMessage> {
        //     match message {
        //         VideoPlayerMessage::NextFrame => {
        //             let mut cmds = Vec::new();
    
        //             let mut restart_stream = false;
        //             if self.restart_stream {
        //                 restart_stream = true;
        //                 // Set flag to false to avoid potentially multiple seeks
        //                 self.restart_stream = false;
        //             }
        //             let mut eos_pause = false;
    
        //             for msg in self.bus.iter() {
        //                 match msg.view() {
        //                     gst::MessageView::Error(err) => panic!("{:#?}", err),
        //                     gst::MessageView::Eos(_eos) => {
        //                         cmds.push(VideoPlayerMessage::EndOfPlayback.into_cmd());
        //                         if self.looping {
        //                             restart_stream = true;
        //                         } else {
        //                             eos_pause = true;
        //                         }
        //                     }
        //                     _ => {}
        //                 }
        //             }
    
        //             // Don't run eos_pause if restart_stream is true; fixes "pausing" after restarting a stream
        //             if restart_stream {
        //                 if let Err(err) = self.restart_stream() {
        //                     eprintln!("cannot restart stream (can't seek): {:#?}", err);
        //                 }
        //             } else if eos_pause {
        //                 self.is_eos = true;
        //                 self.set_paused(true);
        //             }
    
        //             return Command::batch(cmds);
        //         }
        //         VideoPlayerMessage::EndOfPlayback => {}
        //     }
        //     Command::none()
        // }
    
        /// Get the current frame.
        pub fn frame_receiver(&self) -> &Receiver<Vec<u8>> {
            &self.frame_receiver
        }
   
        // /// Restarts a stream; seeks to the first frame and unpauses, sets the `eos` flag to false.
        // pub fn restart_stream(&mut self) -> Result<(), Error> {
        //     self.is_eos = false;
        //     self.set_paused(false);
        //     self.seek(0)?;
        //     Ok(())
        // }

}