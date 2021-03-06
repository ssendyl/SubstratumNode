// Copyright (c) 2017-2018, Substratum LLC (https://substratum.net) and/or its affiliates. All rights reserved.

#[derive (Debug, PartialEq)]
pub struct FramedChunk {
    pub chunk: Vec<u8>,
    pub last_chunk: bool
}

pub trait Framer: Send {
    fn add_data (&mut self, data: &[u8]);
    fn take_frame (&mut self) -> Option<FramedChunk>;
}
