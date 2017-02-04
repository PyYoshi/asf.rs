#[macro_use]
extern crate nom;

#[macro_use]
extern crate log;

use nom::{le_u64, le_u32, le_u16, le_u8, IResult};

// http://uguisu.skr.jp/Windows/format_asf.html
// https://tools.ietf.org/html/draft-fleischman-asf-01
// http://drang.s4.xrea.com/program/tips/id3tag/wmp/

// 75B22630-668E-11CF-A6D9-00AA0062CE6C
pub const HEADER_OBJECT_GUID: [u8; 16] = [0x30, 0x26, 0xB2, 0x75, 0x8E, 0x66, 0xCF, 0x11, 0xA6,
                                          0xD9, 0x00, 0xAA, 0x00, 0x62, 0xCE, 0x6C];

// 8CABDCA1-A947-11CF-8EE4-00C00C205365
pub const FILE_PROPERTIES_OBJECT_GUID: [u8; 16] = [0xA1, 0xDC, 0xAB, 0x8C, 0x47, 0xA9, 0xCF, 0x11,
                                                   0x8E, 0xE4, 0x00, 0xC0, 0x0C, 0x20, 0x53, 0x65];

// B7DC0791-A9B7-11CF-8EE6-00C00C205365
pub const STREAM_PROPERTIES_OBJECT_GUID: [u8; 16] = [0x91, 0x07, 0xDC, 0xB7, 0xB7, 0xA9, 0xCF,
                                                     0x11, 0x8E, 0xE6, 0x00, 0xC0, 0x0C, 0x20,
                                                     0x53, 0x65];

// 7BF875CE-468D-11D1-8D82-006097C9A2B2
pub const STREAM_BITRATE_PROPERTIES_OBJECT_GUID: [u8; 16] = [0xCE, 0x75, 0xF8, 0x7B, 0x8D, 0x46,
                                                             0xD1, 0x11, 0x8D, 0x82, 0x00, 0x60,
                                                             0x97, 0xC9, 0xA2, 0xB2];

// 75B22636-668E-11CF-A6D9-00AA0062CE6C
pub const DATA_OBJECT_GUID: [u8; 16] = [0x36, 0x26, 0xB2, 0x75, 0x8E, 0x66, 0xCF, 0x11, 0xA6,
                                        0xD9, 0x00, 0xAA, 0x00, 0x62, 0xCE, 0x6C];

// F8699E40-5B4D-11CF-A8FD-00805F5C442B
pub const STREAM_PROPERTIES_OBJECT_STREAM_TYPE_AUDIO_GUID: [u8; 16] =
    [0x40, 0x9E, 0x69, 0xF8, 0x4D, 0x5B, 0xCF, 0x11, 0xA8, 0xFD, 0x00, 0x80, 0x5F, 0x5C, 0x44,
     0x2B];

// BC19EFC0-5B4D-11CF-A8FD-00805F5C442B
pub const STREAM_PROPERTIES_OBJECT_STREAM_TYPE_VIDEO_GUID: [u8; 16] =
    [0xC0, 0xEF, 0x19, 0xBC, 0x4D, 0x5B, 0xCF, 0x11, 0xA8, 0xFD, 0x00, 0x80, 0x5F, 0x5C, 0x44,
     0x2B];

#[derive(Debug)]
pub struct HeaderObject {
    object_id: Vec<u8>, // 75B22630-668E-11CF-A6D9-00AA0062CE6C
    object_size: u64, // オブジェクト全体のサイズ
    num_header_objects: u32, // Header Object に含まれる、子オブジェクトの総数
    reserved_1: u8, // 予約領域
    reserved_2: u8, // 予約領域
}

#[derive(Debug)]
pub struct FilePropertiesObject {
    object_id: Vec<u8>, // 8CABDCA1-A947-11CF-8EE4-00C00C205365
    object_size: u64, // オブジェクト全体のサイズ
    file_id: Vec<u8>, // GUID
    file_size: u64, // このサイズはasfのファイルのサイズと等しくなります
    creation_date: u64, // ファイルの作成日 1601年1月1日からのナノ秒で表現
    data_packets_count: u64, // Windows Media Playerの統計情報に現れる受信したパケットに等しくなるはず。
    play_duration: u64, // 再生時間
    send_duration: u64, // 送信時間
    preoll: u64, // ファイル再生を始める前に必要なバッファリング時間
    flags: u32, // フラグ. Broadcast Flag(1bit), Seekable Flag(1bit), Reserved(30bits)
    min_data_packet_size: u32, // 送信時における最小のデータパケットのサイズ
    max_data_packet_size: u32, // 送信時における最大のデータパケットのサイズ
    max_bitrate: u32, // 送信時における最大ビットレート
}

#[derive(Debug)]
pub struct StreamPropertiesObject {
    object_id: Vec<u8>, // B7DC0791-A9B7-11CF-8EE6-00C00C205365
    object_size: u64, // オブジェクト全体のサイズ
    stream_type: Vec<u8>, // Audio, VideoなどのGUIDが含まれる
    error_correction_type: Vec<u8>, // デジたりメディアストリームで利用される修正タイプGUID ASF_No_Error_Correction: 20FB5700-5B55-11CF-A8FD-00805F5C442B ASF_Audio_Spread:  	BFC3CD50-618F-11CF-8BB2-00AA00B4E220    time_offset: u64,
    time_offset: u64, // ストリームの表示時間オフセットを100名の秒単位で表現される
    type_specific_data_length: u32, // Type-Specific Dataのサイズ
    error_correction_data_length: u32, // Error Correction Dataのサイズ
    flags: u16, // フラグ. Stream Number(7bits), Reserved(8bits), Encrypted Content Flag(1bit)
    reserved: u32, // 予約領域
    type_specific_data: Vec<u8>,
    error_correction_data: Vec<u8>,
}

#[derive(Debug)]
pub struct BitrateRecord {
    flags: u16, // Stream Number(7bits), Reserved(9bits)
    average_bitrate: u32,
}

#[derive(Debug)]
pub struct StreamBitratePropertiesObject {
    object_id: Vec<u8>, // 7BF875CE-468D-11D1-8D82-006097C9A2B2
    object_size: u64, // オブジェクト全体のサイズ
    bitrate_records_count: u16, // Bitrate Records の総数
    bitrate_records: Vec<BitrateRecord>,
}

#[derive(Debug)]
pub struct DataObject {
    object_id: Vec<u8>, // 75B22636-668E-11CF-A6D9-00AA0062CE6C
    object_size: u64, // オブジェクト全体のサイズ
    file_id: Vec<u8>, // GUID
    total_data_packets: u16, // Data Objectに存在するエントリーの数
    reserved: u16, // 予約領域
    data_packets: Vec<u8>, // 実データ
}

named!(parse_header_object<&[u8], HeaderObject>,
    do_parse!(
        object_id: take!(16) >>
        object_size: le_u64 >>
        num_header_objects: le_u32 >>
        reserved_1: le_u8  >>
        reserved_2: le_u8  >>
        (HeaderObject{ 
            object_id: object_id.to_vec(),
            object_size: object_size,
            num_header_objects: num_header_objects,
            reserved_1: reserved_1,
            reserved_2: reserved_2,
        })
    )
);

named!(parse_file_props_object<&[u8], FilePropertiesObject>,
    do_parse!(
        object_size: le_u64 >>
        file_id: take!(16) >>
        file_size: le_u64 >>
        creation_date: le_u64 >>
        data_packets_count: le_u64 >>
        play_duration: le_u64 >>
        send_duration: le_u64 >>
        preoll: le_u64 >>
        flags: le_u32 >>
        min_data_packet_size: le_u32 >>
        max_data_packet_size: le_u32 >>
        max_bitrate: le_u32 >>
        (FilePropertiesObject{ 
            object_id: FILE_PROPERTIES_OBJECT_GUID.to_vec(),
            object_size: object_size,
            file_id: file_id.to_vec(),
            file_size: file_size,
            creation_date: creation_date,
            data_packets_count: data_packets_count,
            play_duration: play_duration,
            send_duration: send_duration,
            preoll: preoll,
            flags: flags,
            min_data_packet_size: min_data_packet_size,
            max_data_packet_size: max_data_packet_size,
            max_bitrate: max_bitrate,
        })
    )
);

named!(parse_stream_props_object<&[u8], StreamPropertiesObject>,
    do_parse!(
        object_size: le_u64 >>
        stream_type: take!(16) >>
        error_correction_type: take!(16) >>
        time_offset: le_u64 >>
        type_specific_data_length: le_u32 >>
        error_correction_data_length: le_u32 >>
        flags: le_u16 >>
        reserved: le_u32 >>
        type_specific_data: take!(type_specific_data_length) >>
        error_correction_data: take!(error_correction_data_length) >>
        (StreamPropertiesObject{ 
            object_id: STREAM_PROPERTIES_OBJECT_GUID.to_vec(),
            object_size: object_size,
            stream_type: stream_type.to_vec(),
            error_correction_type: error_correction_type.to_vec(),
            time_offset: time_offset,
            type_specific_data_length: type_specific_data_length,
            error_correction_data_length: error_correction_data_length,
            flags: flags,
            reserved: reserved,
            type_specific_data: type_specific_data.to_vec(),
            error_correction_data: error_correction_data.to_vec(),
        })
    )
);

named!(parse_bitrate_record<&[u8], BitrateRecord>,
    do_parse!(
        flags: le_u16 >>
        average_bitrate: le_u32 >>
        (BitrateRecord{
            flags: flags,
            average_bitrate: average_bitrate,
        })
    )
);

named!(parse_stream_bitrate_props_object<&[u8], StreamBitratePropertiesObject>,
    do_parse!(
        object_size: le_u64 >>
        bitrate_records_count: le_u16 >>
        bitrate_records: many_m_n!(bitrate_records_count as usize, bitrate_records_count as usize, parse_bitrate_record) >>
        (StreamBitratePropertiesObject{
            object_id: STREAM_BITRATE_PROPERTIES_OBJECT_GUID.to_vec(),
            object_size: object_size,
            bitrate_records_count: bitrate_records_count,
            bitrate_records: bitrate_records,
        })
    )
);

named!(parse_data_object_record<&[u8], DataObject>,
    do_parse!(
        object_size: le_u64 >>
        file_id: take!(16) >>
        total_data_packets: le_u16 >>
        reserved: le_u16 >>
        data_packets: take!(object_size-44) >>
        (DataObject{
            object_id: DATA_OBJECT_GUID.to_vec(),
            object_size: object_size,
            file_id: file_id.to_vec(),
            total_data_packets: total_data_packets,
            reserved: reserved,
            data_packets: data_packets.to_vec(),
        })
    )
);

named!(parse_guid<&[u8], Vec<u8>>,
    do_parse!(
        object_id: take!(16) >>
        (
            object_id.to_vec()
        )
    )
);

named!(parse_object<&[u8], Vec<u8>>,
    do_parse!(
        object_size: le_u64 >>
        object: take!(object_size - 24) >>
        (
            object.to_vec()
        )
    )
);

#[derive(Debug)]
pub struct ASF {
    header_object: HeaderObject,
    file_props_object: Option<Box<FilePropertiesObject>>,
    stream_props_objects: Vec<StreamPropertiesObject>,
    stream_bitrate_props_object: Option<Box<StreamBitratePropertiesObject>>,
    data_object: Option<Box<DataObject>>,
}

pub fn parse_asf(input: &[u8]) -> IResult<&[u8], ASF> {
    let mut file_props_object: Option<Box<FilePropertiesObject>> = None;
    let mut stream_props_objects: Vec<StreamPropertiesObject> = Vec::new();
    let mut stream_bitrate_props_object: Option<Box<StreamBitratePropertiesObject>> = None;
    let mut data_object: Option<Box<DataObject>> = None;

    let (mut input, header_object) = try_parse!(input, parse_header_object);
    for _ in 0..header_object.num_header_objects {
        let (remain, guid) = try_parse!(input, parse_guid);
        input = remain;
        let guid_arr = guid.as_slice();
        if &guid_arr == &FILE_PROPERTIES_OBJECT_GUID {
            let (remain, file_props_object_r) = try_parse!(input, parse_file_props_object);
            file_props_object = Some(Box::new(file_props_object_r));
            input = remain;
        } else if &guid_arr == &STREAM_PROPERTIES_OBJECT_GUID {
            let (remain, stream_props_object_r) = try_parse!(input, parse_stream_props_object);
            stream_props_objects.push(stream_props_object_r);
            input = remain;
        } else if &guid_arr == &STREAM_BITRATE_PROPERTIES_OBJECT_GUID {
            let (remain, stream_bitrate_props_object_r) =
                try_parse!(input, parse_stream_bitrate_props_object);
            stream_bitrate_props_object = Some(Box::new(stream_bitrate_props_object_r));
            input = remain;
        } else if &guid_arr == &DATA_OBJECT_GUID {
            let (remain, data_object_r) = try_parse!(input, parse_data_object_record);
            data_object = Some(Box::new(data_object_r));
            input = remain;
        } else {
            // skip this object
            debug!("skip this object: GUID={:?}", guid_arr);
            let (remain, _) = try_parse!(input, parse_object);
            input = remain;
        }
    }

    IResult::Done(input,
                  ASF {
                      header_object: header_object,
                      file_props_object: file_props_object,
                      stream_props_objects: stream_props_objects,
                      stream_bitrate_props_object: stream_bitrate_props_object,
                      data_object: data_object,
                  })
}

#[test]
fn parse_asf_test1() {
    let input = include_bytes!("../assets/320x180_10fps.asf");
    let asf_obj = parse_asf(input);

    match asf_obj {
        IResult::Done(_, v) => {
            let ans_header_object = HeaderObject {
                object_id: HEADER_OBJECT_GUID.to_vec(),
                object_size: 1106,
                num_header_objects: 6,
                reserved_1: 1,
                reserved_2: 2,
            };

            let ans_file_props_object = FilePropertiesObject {
                object_id: FILE_PROPERTIES_OBJECT_GUID.to_vec(),
                object_size: 104,
                file_id: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                file_size: 33248,
                creation_date: 116444736000000000,
                data_packets_count: 10,
                play_duration: 41460000,
                send_duration: 10460000,
                preoll: 3100,
                flags: 2,
                min_data_packet_size: 3200,
                max_data_packet_size: 3200,
                max_bitrate: 232000,
            };

            let ans_stream_props_objects =
                vec![StreamPropertiesObject {
                         object_id: STREAM_PROPERTIES_OBJECT_GUID.to_vec(),
                         object_size: 133,
                         stream_type: vec![192, 239, 25, 188, 77, 91, 207, 17, 168, 253, 0, 128,
                                           95, 92, 68, 43],
                         error_correction_type: vec![0, 87, 251, 32, 85, 91, 207, 17, 168, 253, 0,
                                                     128, 95, 92, 68, 43],
                         time_offset: 0,
                         type_specific_data_length: 55,
                         error_correction_data_length: 0,
                         flags: 1,
                         reserved: 0,
                         type_specific_data: vec![64, 1, 0, 0, 180, 0, 0, 0, 2, 44, 0, 44, 0, 0,
                                                  0, 64, 1, 0, 0, 180, 0, 0, 0, 1, 0, 24, 0, 87,
                                                  77, 86, 50, 0, 163, 2, 0, 0, 0, 0, 0, 0, 0, 0,
                                                  0, 0, 0, 0, 0, 0, 0, 0, 0, 80, 195, 180, 128],
                         error_correction_data: vec![],
                     },
                     StreamPropertiesObject {
                         object_id: STREAM_PROPERTIES_OBJECT_GUID.to_vec(),
                         object_size: 114,
                         stream_type: vec![64, 158, 105, 248, 77, 91, 207, 17, 168, 253, 0, 128,
                                           95, 92, 68, 43],
                         error_correction_type: vec![80, 205, 195, 191, 143, 97, 207, 17, 139,
                                                     178, 0, 170, 0, 180, 226, 32],
                         time_offset: 0,
                         type_specific_data_length: 28,
                         error_correction_data_length: 8,
                         flags: 2,
                         reserved: 0,
                         type_specific_data: vec![97, 1, 1, 0, 68, 172, 0, 0, 160, 15, 0, 0, 185,
                                                  0, 16, 0, 10, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
                         error_correction_data: vec![1, 185, 0, 185, 0, 1, 0, 0],
                     }];

            let ans = ASF {
                header_object: ans_header_object,
                file_props_object: Some(Box::new(ans_file_props_object)),
                stream_props_objects: ans_stream_props_objects,
                stream_bitrate_props_object: None,
                data_object: None,
            };
            assert_eq!(v, ans);
        }
        IResult::Incomplete(a) => {
            panic!("Incomplete: {:?}", a);
        }
        IResult::Error(a) => {
            panic!("Error: {:?}", a);
        }
    }
}

#[test]
fn parse_asf_test2() {
    let input = include_bytes!("../assets/kte.asf");
    let asf_obj = parse_asf(input);

    match asf_obj {
        IResult::Done(_, v) => {
            let ans_header_object = HeaderObject {
                object_id: HEADER_OBJECT_GUID.to_vec(),
                object_size: 5267,
                num_header_objects: 7,
                reserved_1: 1,
                reserved_2: 2,
            };

            let ans_file_props_object = FilePropertiesObject {
                object_id: FILE_PROPERTIES_OBJECT_GUID.to_vec(),
                object_size: 104,
                file_id: vec![43, 141, 105, 203, 0, 18, 13, 78, 169, 16, 243, 97, 122, 251, 50,
                              255],
                file_size: 1107099,
                creation_date: 131299883009790000,
                data_packets_count: 153,
                play_duration: 193990000,
                send_duration: 178220000,
                preoll: 5000,
                flags: 2,
                min_data_packet_size: 7200,
                max_data_packet_size: 7200,
                max_bitrate: 585498,
            };

            let ans_stream_props_objects =
                vec![StreamPropertiesObject {
                         object_id: STREAM_PROPERTIES_OBJECT_GUID.to_vec(),
                         object_size: 114,
                         stream_type: vec![64, 158, 105, 248, 77, 91, 207, 17, 168, 253, 0, 128,
                                           95, 92, 68, 43],
                         error_correction_type: vec![80, 205, 195, 191, 143, 97, 207, 17, 139,
                                                     178, 0, 170, 0, 180, 226, 32],
                         time_offset: 0,
                         type_specific_data_length: 28,
                         error_correction_data_length: 8,
                         flags: 1,
                         reserved: 32762,
                         type_specific_data: vec![97, 1, 2, 0, 68, 172, 0, 0, 69, 31, 0, 0, 207,
                                                  5, 16, 0, 10, 0, 0, 136, 0, 0, 15, 0, 0, 0, 0, 0],
                         error_correction_data: vec![1, 207, 5, 207, 5, 1, 0, 0],
                     },
                     StreamPropertiesObject {
                         object_id: STREAM_PROPERTIES_OBJECT_GUID.to_vec(),
                         object_size: 134,
                         stream_type: vec![192, 239, 25, 188, 77, 91, 207, 17, 168, 253, 0, 128,
                                           95, 92, 68, 43],
                         error_correction_type: vec![0, 87, 251, 32, 85, 91, 207, 17, 168, 253, 0,
                                                     128, 95, 92, 68, 43],
                         time_offset: 0,
                         type_specific_data_length: 56,
                         error_correction_data_length: 0,
                         flags: 2,
                         reserved: 298803599,
                         type_specific_data: vec![128, 2, 0, 0, 104, 1, 0, 0, 2, 45, 0, 45, 0, 0,
                                                  0, 128, 2, 0, 0, 104, 1, 0, 0, 1, 0, 24, 0, 87,
                                                  77, 86, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                                  0, 0, 0, 0, 0, 0, 0, 0, 78, 137, 72, 1, 32],
                         error_correction_data: vec![],
                     }];

            let ans_stream_bitrate_props_object = StreamBitratePropertiesObject {
                object_id: STREAM_BITRATE_PROPERTIES_OBJECT_GUID.to_vec(),
                object_size: 38,
                bitrate_records_count: 2,
                bitrate_records: vec![BitrateRecord {
                                          flags: 1,
                                          average_bitrate: 65733,
                                      },
                                      BitrateRecord {
                                          flags: 2,
                                          average_bitrate: 519765,
                                      }],
            };

            let ans = ASF {
                header_object: ans_header_object,
                file_props_object: Some(Box::new(ans_file_props_object)),
                stream_props_objects: ans_stream_props_objects,
                stream_bitrate_props_object: Some(Box::new(ans_stream_bitrate_props_object)),
                data_object: None,
            };
            assert_eq!(v, ans);
        }
        IResult::Incomplete(a) => {
            panic!("Incomplete: {:?}", a);
        }
        IResult::Error(a) => {
            panic!("Error: {:?}", a);
        }
    }
}