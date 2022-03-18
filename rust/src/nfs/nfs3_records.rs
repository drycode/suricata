/* Copyright (C) 2017-2022 Open Information Security Foundation
 *
 * You can copy, redistribute or modify this Program under the terms of
 * the GNU General Public License version 2 as published by the Free
 * Software Foundation.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * version 2 along with this program; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA
 * 02110-1301, USA.
 */

//! Nom parsers for RPC & NFSv3

use std::cmp;
use nom::{IResult, be_u32, be_u64, rest};
use crate::nfs::nfs_records::*;

#[derive(Debug,PartialEq)]
pub struct Nfs3Handle<'a> {
    pub len: u32,
    pub value: &'a[u8],
}

named!(pub parse_nfs3_handle<Nfs3Handle>,
    do_parse!(
        obj_len: be_u32
        >> obj: take!(obj_len)
        >> (
            Nfs3Handle {
                len:obj_len,
                value:obj,
            }
        ))
);

#[derive(Debug,PartialEq)]
pub struct Nfs3ReplyCreate<'a> {
    pub status: u32,
    pub handle: Option<Nfs3Handle<'a>>,
}

named!(pub parse_nfs3_response_create<Nfs3ReplyCreate>,
    do_parse!(
        status: be_u32
        >> handle_has_value: verify!(be_u32, |v| v <= 1)
        >> handle: cond!(handle_has_value == 1, parse_nfs3_handle)
        >> (
            Nfs3ReplyCreate {
               status:status,
               handle:handle,
            }
        ))
);

#[derive(Debug,PartialEq)]
pub struct Nfs3ReplyLookup<'a> {
    pub status: u32,
    pub handle: Nfs3Handle<'a>,
}

named!(pub parse_nfs3_response_lookup<Nfs3ReplyLookup>,
    do_parse!(
        status: be_u32
        >> handle: parse_nfs3_handle
        >> (
            Nfs3ReplyLookup {
                status:status,
                handle:handle,
            }
        ))
);

#[derive(Debug,PartialEq)]
pub struct Nfs3RequestCreate<'a> {
    pub handle: Nfs3Handle<'a>,
    pub name_len: u32,
    pub create_mode: u32,
    pub verifier: &'a[u8],
    pub name_vec: Vec<u8>,
}

named!(pub parse_nfs3_request_create<Nfs3RequestCreate>,
    do_parse!(
            handle: parse_nfs3_handle
        >>  name_len: be_u32
        >>  name: take!(name_len)
        >>  create_mode: be_u32
        >>  verifier: rest
        >> (
            Nfs3RequestCreate {
                handle:handle,
                name_len:name_len,
                create_mode:create_mode,
                verifier:verifier,
                name_vec:name.to_vec(),
            }
        ))
);

#[derive(Debug,PartialEq)]
pub struct Nfs3RequestRemove<'a> {
    pub handle: Nfs3Handle<'a>,
    pub name_len: u32,
    pub name_vec: Vec<u8>,
}

named!(pub parse_nfs3_request_remove<Nfs3RequestRemove>,
    do_parse!(
            handle: parse_nfs3_handle
        >>  name_len: be_u32
        >>  name: take!(name_len)
        >>  _fill_bytes: rest
        >> (
            Nfs3RequestRemove {
                handle:handle,
                name_len:name_len,
                name_vec:name.to_vec(),
            }
        ))
);

#[derive(Debug,PartialEq)]
pub struct Nfs3RequestRmdir<'a> {
    pub handle: Nfs3Handle<'a>,
    pub name_vec: Vec<u8>,
}

named!(pub parse_nfs3_request_rmdir<Nfs3RequestRmdir>,
    do_parse!(
            dir_handle: parse_nfs3_handle
        >>  name_len: be_u32
        >>  name: take!(name_len)
        >>  _fill_bytes: cond!(name_len % 4 != 0, take!(4 - name_len % 4))
        >> (
            Nfs3RequestRmdir {
                handle:dir_handle,
                name_vec:name.to_vec(),
            }
        ))
);

#[derive(Debug,PartialEq)]
pub struct Nfs3RequestMkdir<'a> {
    pub handle: Nfs3Handle<'a>,
    pub name_vec: Vec<u8>,
}

named!(pub parse_nfs3_request_mkdir<Nfs3RequestMkdir>,
    do_parse!(
            dir_handle: parse_nfs3_handle
        >>  name_len: be_u32
        >>  name: take!(name_len)
        >>  _fill_bytes: cond!(name_len % 4 != 0, take!(4 - name_len % 4))
        >>  _attributes: rest
        >> (
            Nfs3RequestMkdir {
                handle:dir_handle,
                name_vec:name.to_vec(),
            }
        ))
);

#[derive(Debug,PartialEq)]
pub struct Nfs3RequestRename<'a> {
    pub from_handle: Nfs3Handle<'a>,
    pub from_name_vec: Vec<u8>,
    pub to_handle: Nfs3Handle<'a>,
    pub to_name_vec: Vec<u8>,
}

named!(pub parse_nfs3_request_rename<Nfs3RequestRename>,
    do_parse!(
            from_handle: parse_nfs3_handle
        >>  from_name_len: be_u32
        >>  from_name: take!(from_name_len)
        >>  _from_fill_bytes: cond!(from_name_len % 4 != 0, take!(4 - from_name_len % 4))
        >>  to_handle: parse_nfs3_handle
        >>  to_name_len: be_u32
        >>  to_name: take!(to_name_len)
        >>  _to_fill_bytes: rest
        >> (
            Nfs3RequestRename {
                from_handle,
                from_name_vec:from_name.to_vec(),
                to_handle,
                to_name_vec:to_name.to_vec(),
            }
        ))
);

#[derive(Debug,PartialEq)]
pub struct Nfs3RequestGetAttr<'a> {
    pub handle: Nfs3Handle<'a>,
}

named!(pub parse_nfs3_request_getattr<Nfs3RequestGetAttr>,
    do_parse!(
            handle: parse_nfs3_handle
        >> (
            Nfs3RequestGetAttr {
                handle:handle,
            }
        ))
);

#[derive(Debug,PartialEq)]
pub struct Nfs3RequestAccess<'a> {
    pub handle: Nfs3Handle<'a>,
    pub check_access: u32,
}

named!(pub parse_nfs3_request_access<Nfs3RequestAccess>,
    do_parse!(
            handle: parse_nfs3_handle
        >>  check_access: be_u32
        >> (
            Nfs3RequestAccess {
                handle:handle,
                check_access:check_access,
            }
        ))
);

#[derive(Debug,PartialEq)]
pub struct Nfs3RequestCommit<'a> {
    pub handle: Nfs3Handle<'a>,
}

named!(pub parse_nfs3_request_commit<Nfs3RequestCommit>,
    do_parse!(
            handle: parse_nfs3_handle
        >>  _offset: be_u64
        >>  _count: be_u32
        >> (
            Nfs3RequestCommit {
                handle
            }
        ))
);

#[derive(Debug,PartialEq)]
pub struct Nfs3RequestRead<'a> {
    pub handle: Nfs3Handle<'a>,
    pub offset: u64,
}

named!(pub parse_nfs3_request_read<Nfs3RequestRead>,
    do_parse!(
            handle: parse_nfs3_handle
        >>  offset: be_u64
        >>  _count: be_u32
        >> (
            Nfs3RequestRead {
                handle,
                offset
            }
        ))
);

#[derive(Debug,PartialEq)]
pub struct Nfs3RequestLookup<'a> {
    pub handle: Nfs3Handle<'a>,

    pub name_vec: Vec<u8>,
}

named!(pub parse_nfs3_request_lookup<Nfs3RequestLookup>,
    do_parse!(
            handle: parse_nfs3_handle
        >>  name_len: be_u32
        >>  name_contents: take!(name_len)
        >>  _name_padding: rest
        >> (
            Nfs3RequestLookup {
                handle,
                name_vec:name_contents.to_vec(),
            }
        ))
);


#[derive(Debug,PartialEq)]
pub struct Nfs3ResponseReaddirplusEntryC<'a> {
    pub name_vec: Vec<u8>,
    pub handle: Option<Nfs3Handle<'a>>,
}

named!(pub parse_nfs3_response_readdirplus_entry<Nfs3ResponseReaddirplusEntryC>,
    do_parse!(
           _file_id: be_u64
        >> name_len: be_u32
        >> name_content: take!(name_len)
        >> _fill_bytes: cond!(name_len % 4 != 0, take!(4 - name_len % 4))
        >> _cookie: take!(8)
        >> attr_value_follows: verify!(be_u32, |v| v <= 1)
        >> _attr: cond!(attr_value_follows==1, take!(84))
        >> handle_value_follows: verify!(be_u32, |v| v <= 1)
        >> handle: cond!(handle_value_follows==1, parse_nfs3_handle)
        >> (
                Nfs3ResponseReaddirplusEntryC {
                    name_vec:name_content.to_vec(),
                    handle,
                }
           )
        )
);

#[derive(Debug,PartialEq)]
pub struct Nfs3ResponseReaddirplusEntry<'a> {
    pub entry: Option<Nfs3ResponseReaddirplusEntryC<'a>>,
}

named!(pub parse_nfs3_response_readdirplus_entry_cond<Nfs3ResponseReaddirplusEntry>,
    do_parse!(
           value_follows: verify!(be_u32, |v| v <= 1)
        >> entry: cond!(value_follows==1, parse_nfs3_response_readdirplus_entry)
        >> (
            Nfs3ResponseReaddirplusEntry {
                entry
            }
           ))
);

#[derive(Debug,PartialEq)]
pub struct Nfs3ResponseReaddirplus<'a> {
    pub status: u32,
    pub data: &'a[u8],
}

named!(pub parse_nfs3_response_readdirplus<Nfs3ResponseReaddirplus>,
    do_parse!(
        status: be_u32
        >> dir_attr_follows: verify!(be_u32, |v| v <= 1)
        >> _dir_attr: cond!(dir_attr_follows == 1, take!(84))
        >> _verifier: take!(8)
        >> data: rest

        >> ( Nfs3ResponseReaddirplus {
                status,
                data
        } ))
);

pub(crate) fn many0_nfs3_response_readdirplus_entries<'a>(input: &'a [u8]) -> IResult<&'a[u8], Vec<Nfs3ResponseReaddirplusEntry<'a>>> {
    many0!(input, complete!(parse_nfs3_response_readdirplus_entry_cond))
}

#[derive(Debug,PartialEq)]
pub struct Nfs3RequestReaddirplus<'a> {
    pub handle: Nfs3Handle<'a>,

    pub cookie: u32,
    pub verifier: &'a[u8],
    pub dircount: u32,
    pub maxcount: u32,
}

named!(pub parse_nfs3_request_readdirplus<Nfs3RequestReaddirplus>,
    do_parse!(
            handle: parse_nfs3_handle
        >>  cookie: be_u32
        >>  verifier: take!(8)
        >>  dircount: be_u32
        >>  maxcount: be_u32
        >> (
            Nfs3RequestReaddirplus {
                handle:handle,
                cookie:cookie,
                verifier:verifier,
                dircount:dircount,
                maxcount:maxcount,
            }
        ))
);

#[derive(Debug,PartialEq)]
pub struct Nfs3RequestWrite<'a> {
    pub handle: Nfs3Handle<'a>,

    pub offset: u64,
    pub count: u32,
    pub stable: u32,
    pub file_len: u32,
    pub file_data: &'a[u8],
}

/// Complete data expected
fn parse_nfs3_data_complete(i: &[u8], file_len: usize, fill_bytes: usize) -> IResult<&[u8], &[u8]> {
    let (i, file_data) = take!(i, file_len as usize)?;
    let (i, _) = cond!(i, fill_bytes > 0, take!(fill_bytes))?;
    Ok((i, file_data))
}

/// Partial data. We have all file_len, but need to consider fill_bytes
fn parse_nfs3_data_partial(i: &[u8], file_len: usize, fill_bytes: usize) -> IResult<&[u8], &[u8]> {
    let (i, file_data) = take!(i, file_len as usize)?;
    let fill_bytes = cmp::min(fill_bytes as usize, i.len());
    let (i, _) = cond!(i, fill_bytes > 0, take!(fill_bytes))?;
    Ok((i, file_data))
}

pub fn parse_nfs3_request_write(i: &[u8], complete: bool) -> IResult<&[u8], Nfs3RequestWrite> {
    let (i, handle) = parse_nfs3_handle(i)?;
    let (i, offset) = be_u64(i)?;
    let (i, count) = be_u32(i)?;
    let (i, stable) = verify!(i, be_u32, |v| v <= 2)?;
    let (i, file_len) = verify!(i, be_u32, |v| v <= count)?;
    let fill_bytes = if file_len % 4 != 0 { 4 - file_len % 4 } else { 0 };
    let (i, file_data) = if complete {
        parse_nfs3_data_complete(i, file_len as usize, fill_bytes as usize)?
    } else if i.len() >= file_len as usize {
        parse_nfs3_data_partial(i, file_len as usize, fill_bytes as usize)?
    } else {
        rest(i)?
    };
    Ok((i, Nfs3RequestWrite {
        handle,
        offset,
        count,
        stable,
        file_len,
        file_data,
    }))
}

pub fn parse_nfs3_reply_read(i: &[u8], complete: bool) -> IResult<&[u8], NfsReplyRead> {
    let (i, status) = be_u32(i)?;
    let (i, attr_follows) = verify!(i, be_u32, |v| v <= 1)?;
    let (i, attr_blob) = take!(i, 84_usize)?; // fixed size?
    let (i, count) = be_u32(i)?;
    let (i, eof) = verify!(i, be_u32, |v| v <= 1)?;
    let (i, data_len) = verify!(i, be_u32, |v| v <= count)?;
    let fill_bytes = if data_len % 4 != 0 { 4 - data_len % 4 } else { 0 };
    // Handle the various file data parsing logics
    let (i, data) = if complete {
        parse_nfs3_data_complete(i, data_len as usize, fill_bytes as usize)?
    } else if i.len() >= data_len as usize {
        parse_nfs3_data_partial(i, data_len as usize, fill_bytes as usize)?
    } else {
        rest(i)?
    };
    let reply = NfsReplyRead {
        status,
        attr_follows,
        attr_blob,
        count,
        eof: eof != 0,
        data_len,
        data,
    };
    Ok((i, reply))
}
