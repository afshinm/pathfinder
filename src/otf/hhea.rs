// Copyright 2017 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use byteorder::{BigEndian, ReadBytesExt};
use otf::{Error, FontTable};
use std::mem;
use util::Jump;

#[derive(Clone, Debug)]
pub struct HheaTable {
    pub line_gap: i16,
    pub number_of_h_metrics: u16,
}

impl HheaTable {
    pub fn new(table: FontTable) -> Result<HheaTable, Error> {
        let mut reader = table.bytes;

        // Check the version.
        let major_version = try!(reader.read_u16::<BigEndian>().map_err(Error::eof));
        let minor_version = try!(reader.read_u16::<BigEndian>().map_err(Error::eof));
        if (major_version, minor_version) != (1, 0) {
            return Err(Error::UnsupportedHheaVersion)
        }

        // Read the height-related metrics.
        let _ascender = try!(reader.read_i16::<BigEndian>().map_err(Error::eof));
        let _descender = try!(reader.read_i16::<BigEndian>().map_err(Error::eof));
        let line_gap = try!(reader.read_i16::<BigEndian>().map_err(Error::eof));

        // Read the number of `hmtx` entries.
        try!(reader.jump(mem::size_of::<u16>() * 12).map_err(Error::eof));
        let number_of_h_metrics = try!(reader.read_u16::<BigEndian>().map_err(Error::eof));

        Ok(HheaTable {
            line_gap: line_gap,
            number_of_h_metrics: number_of_h_metrics,
        })
    }
}

