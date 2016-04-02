// Copyright 2016 PingCAP, Inc. 
// 
// Licensed under the Apache License, Version 2.0 (the "License"); 
// you may not use this file except in compliance with the License. 
// You may obtain a copy of the License at 
// 
//     http://www.apache.org/licenses/LICENSE-2.0 
// 
// Unless required by applicable law or agreed to in writing, software 
// distributed under the License is distributed on an "AS IS" BASIS, 
// See the License for the specific language governing permissions and 
// limitations under the License. 

use std::error;
use std::boxed::Box;
use std::result;
use std::io;
use std::net;
use std::vec::Vec;

use protobuf::ProtobufError;

use util::codec;
use pd;
use raft;
use kvproto::metapb;

quick_error!{
    #[derive(Debug)]
    pub enum Error {
        RegionNotFound(region_id:u64) {
            description("region is not found")
            display("region {} not found", region_id)
        }
        RegionNotInitialized(region_id: u64) {
            description("region has not been initialized yet.")
            display("region {} not initialized yet", region_id)
        }
        NotLeader(region_id: u64, leader:Option<metapb::Peer>) {
            description("peer is not leader")
            display("peer is not leader for region {}, leader may {:?}", region_id, leader)
        }
        KeyNotInRegion(key: Vec<u8>, region: metapb::Region) {
            description("key is not in region")
            display("key {:?} is not in region key range [{:?}, {:?}) for region {}",
                key, region.get_start_key(), region.get_end_key(), region.get_id())
        }
        Other(err: Box<error::Error + Sync + Send>) {
            from()
            cause(err.as_ref())
            description(err.description())
            display("{:?}", err)
        }

        // Following is for From other errors.
        Io(err: io::Error) {
            from()
            cause(err)
            description(err.description())
        }
        // RocksDB uses plain string as the error.
        // Maybe other libs use this too?
        RocksDB(msg: String) {
            from()
            description("RocksDB error")
            display("{}", msg)
        }
        Protobuf(err: ProtobufError) {
            from()
            cause(err)
            description(err.description())
        }
        Codec(err: codec::Error) {
            from()
            cause(err)
            description(err.description())
        }
        AddrParse(err: net::AddrParseError) {
            from()
            cause(err)
            description(err.description())
        }
        Pd(err: pd::Error) {
            from()
            cause(err)
            description(err.description())
        }
        Raft(err: raft::Error) {
            from()
            cause(err)
            description(err.description())
        }
        Timeout(msg: String) {
            description("request timeout")
            display("{}", msg)
        }
        StaleEpoch(msg: String) {
            description("region is stale")
            display("{}", msg)
        }
    }
}


pub type Result<T> = result::Result<T, Error>;
