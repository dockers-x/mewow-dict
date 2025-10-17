use crate::mdict::header::parse_header;
use crate::mdict::keyblock::{
    parse_key_block_header, parse_key_block_info, parse_key_blocks, RecordDeBufOffset,
};
use crate::mdict::recordblock::{parse_record_blocks, record_block_parser, RecordBlockSize};
use nom::Parser;

/// MDD file structure is similar to MDX, but stores resources (images, audio, etc.) instead of text
/// The key difference is that records contain binary data instead of text definitions
#[derive(Debug)]
pub struct ResourceOffsetInfo {
    pub(crate) path: String,
    // record所在block在buf的offset 截取block使用
    block_offset_in_buf: usize,
    // 解析block使用
    block_csize: usize,
    block_dsize: usize,
    // record在解压后的block的offset 和 end
    record_start_in_de_block: usize,
    record_end_in_de_block: usize,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Resource {
    pub(crate) path: String,
    pub(crate) data: Vec<u8>,
}

/// MDD (Mdict Data) file parser for resource files
/// Structure is similar to MDX but contains binary resources
#[allow(dead_code)]
pub struct Mdd {
    pub resources_offset: Vec<ResourceOffsetInfo>,
    pub record_block_buf: Vec<u8>,
    pub encoding: String,
    pub encrypted: String,
}

impl Mdd {
    /// Create a new MDD instance from raw bytes
    /// let data = include_bytes!("/file.mdd");
    /// let mdd = Mdd::new(&data);
    pub fn new(data: &[u8]) -> Mdd {
        let (data, header) = parse_header(data).unwrap();

        let (data, kbh) = parse_key_block_header(data, &header).unwrap();
        let (data, key_blocks_size) =
            parse_key_block_info(data, kbh.key_block_info_len, &header).unwrap();
        let (data, entries) =
            parse_key_blocks(data, kbh.key_blocks_len, &header, &key_blocks_size).unwrap();
        let (data, record_blocks_size) = parse_record_blocks(data, &header).unwrap();

        //计算position耗时，一次计算就保存下来
        let offset: Vec<ResourceOffsetInfo> = resources_offset(&entries, &record_blocks_size);

        Mdd {
            resources_offset: offset,
            record_block_buf: Vec::from(data),
            encoding: header.encoding,
            encrypted: header.encrypted,
        }
    }

    #[allow(unused)]
    pub fn entries(&self) -> impl Iterator<Item=&ResourceOffsetInfo> {
        return self.resources_offset.iter();
    }

    #[allow(dead_code)]
    pub fn items(&self) -> impl Iterator<Item=Resource> + '_ {
        self.resources_offset.iter().map(|rs| {
            let data = self.find_resource(&rs);
            Resource {
                path: rs.path.clone(),
                data,
            }
        })
    }

    pub fn find_resource(&self, rs: &ResourceOffsetInfo) -> Vec<u8> {
        // block bytes with tail
        let block_buf = &self.record_block_buf[rs.block_offset_in_buf..];

        let (_, block_decompressed) =
            record_block_parser(rs.block_csize, rs.block_dsize).parse(block_buf).unwrap();

        let resource_data =
            &block_decompressed[rs.record_start_in_de_block..rs.record_end_in_de_block];

        return resource_data.to_vec();
    }

    pub fn get_resource_by_path(&self, path: &str) -> Option<Vec<u8>> {
        // Normalize path - remove leading slash or backslash
        let normalized_path = path.trim_start_matches('/').trim_start_matches('\\');
        
        // Find the resource with matching path (case-insensitive)
        self.resources_offset.iter().find(|rs| {
            let rs_path = rs.path.trim_start_matches('/').trim_start_matches('\\');
            rs_path.eq_ignore_ascii_case(normalized_path)
        }).map(|rs| self.find_resource(rs))
    }
}

/// bytes structure: buf -> block -> record(entry)
fn resources_offset(
    records_debuf_index: &Vec<RecordDeBufOffset>,
    record_blocks_size: &Vec<RecordBlockSize>,
) -> Vec<ResourceOffsetInfo> {
    let mut positions: Vec<ResourceOffsetInfo> = vec![];
    let mut i: usize = 0;
    let mut pre_blocks_dsize_sum = 0;
    let mut pre_blocks_csize_sum = 0;
    // 同时开始遍历record_blocks_size和entries，每个block包含0或n个entry，
    // 当entry的buf_decompressed_offset > pre_blocks_dsize_sum时 说明当前block已经遍历结束
    for block in record_blocks_size {
        while i < records_debuf_index.len() {
            let record = &records_debuf_index[i];

            // 当前entry已经属于下一个block，注意等于号
            if record.record_offset_in_debuf >= pre_blocks_dsize_sum + block.dsize {
                break;
            }

            let record_end_in_de_block;
            if i < records_debuf_index.len() - 1 {
                let next_entry = &records_debuf_index[i + 1];
                record_end_in_de_block = next_entry.record_offset_in_debuf - pre_blocks_dsize_sum;
            } else {
                // last entry
                record_end_in_de_block = block.dsize
            }

            positions.push(ResourceOffsetInfo {
                path: record.text.to_string(),
                block_offset_in_buf: pre_blocks_csize_sum,
                block_csize: block.csize,
                block_dsize: block.dsize,
                record_start_in_de_block: record.record_offset_in_debuf - pre_blocks_dsize_sum,
                record_end_in_de_block: record_end_in_de_block,
            });
            i += 1;
        }
        pre_blocks_dsize_sum += block.dsize;
        pre_blocks_csize_sum += block.csize;
    }
    return positions;
}
