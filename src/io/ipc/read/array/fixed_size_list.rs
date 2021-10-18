use std::collections::VecDeque;
use std::io::{Read, Seek};

use arrow_format::ipc;

use crate::array::FixedSizeListArray;
use crate::datatypes::DataType;
use crate::error::Result;

use super::super::deserialize::{read, skip, Node};
use super::super::read_basic::*;

#[allow(clippy::too_many_arguments)]
pub fn read_fixed_size_list<R: Read + Seek>(
    field_nodes: &mut VecDeque<Node>,
    data_type: DataType,
    buffers: &mut VecDeque<&ipc::Schema::Buffer>,
    reader: &mut R,
    block_offset: u64,
    is_little_endian: bool,
    compression: Option<ipc::Message::BodyCompression>,
    version: ipc::Schema::MetadataVersion,
) -> Result<FixedSizeListArray> {
    let field_node = field_nodes.pop_front().unwrap().0;

    let validity = read_validity(
        buffers,
        field_node,
        reader,
        block_offset,
        is_little_endian,
        compression,
    )?;

    let (value_data_type, _) = FixedSizeListArray::get_child_and_size(&data_type);

    let values = read(
        field_nodes,
        value_data_type.data_type().clone(),
        buffers,
        reader,
        block_offset,
        is_little_endian,
        compression,
        version,
    )?;
    Ok(FixedSizeListArray::from_data(data_type, values, validity))
}

pub fn skip_fixed_size_list(
    field_nodes: &mut VecDeque<Node>,
    data_type: &DataType,
    buffers: &mut VecDeque<&ipc::Schema::Buffer>,
) {
    let _ = field_nodes.pop_front().unwrap();

    let _ = buffers.pop_front().unwrap();

    let (field, _) = FixedSizeListArray::get_child_and_size(data_type);

    skip(field_nodes, field.data_type(), buffers)
}
