// This file is generated by rust-protobuf 2.0.4. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0ctikvpb.proto\x12\x06tikvpb\x1a\x11coprocessor.proto\x1a\rkvrpcpb.p\
    roto\x1a\x13raft_serverpb.proto\x1a\x14gogoproto/gogo.proto2\x93\x10\n\
    \x04Tikv\x124\n\x05KvGet\x12\x13.kvrpcpb.GetRequest\x1a\x14.kvrpcpb.GetR\
    esponse\"\0\x127\n\x06KvScan\x12\x14.kvrpcpb.ScanRequest\x1a\x15.kvrpcpb\
    .ScanResponse\"\0\x12C\n\nKvPrewrite\x12\x18.kvrpcpb.PrewriteRequest\x1a\
    \x19.kvrpcpb.PrewriteResponse\"\0\x12=\n\x08KvCommit\x12\x16.kvrpcpb.Com\
    mitRequest\x1a\x17.kvrpcpb.CommitResponse\"\0\x12=\n\x08KvImport\x12\x16\
    .kvrpcpb.ImportRequest\x1a\x17.kvrpcpb.ImportResponse\"\0\x12@\n\tKvClea\
    nup\x12\x17.kvrpcpb.CleanupRequest\x1a\x18.kvrpcpb.CleanupResponse\"\0\
    \x12C\n\nKvBatchGet\x12\x18.kvrpcpb.BatchGetRequest\x1a\x19.kvrpcpb.Batc\
    hGetResponse\"\0\x12R\n\x0fKvBatchRollback\x12\x1d.kvrpcpb.BatchRollback\
    Request\x1a\x1e.kvrpcpb.BatchRollbackResponse\"\0\x12C\n\nKvScanLock\x12\
    \x18.kvrpcpb.ScanLockRequest\x1a\x19.kvrpcpb.ScanLockResponse\"\0\x12L\n\
    \rKvResolveLock\x12\x1b.kvrpcpb.ResolveLockRequest\x1a\x1c.kvrpcpb.Resol\
    veLockResponse\"\0\x121\n\x04KvGC\x12\x12.kvrpcpb.GCRequest\x1a\x13.kvrp\
    cpb.GCResponse\"\0\x12L\n\rKvDeleteRange\x12\x1b.kvrpcpb.DeleteRangeRequ\
    est\x1a\x1c.kvrpcpb.DeleteRangeResponse\"\0\x12;\n\x06RawGet\x12\x16.kvr\
    pcpb.RawGetRequest\x1a\x17.kvrpcpb.RawGetResponse\"\0\x12J\n\x0bRawBatch\
    Get\x12\x1b.kvrpcpb.RawBatchGetRequest\x1a\x1c.kvrpcpb.RawBatchGetRespon\
    se\"\0\x12;\n\x06RawPut\x12\x16.kvrpcpb.RawPutRequest\x1a\x17.kvrpcpb.Ra\
    wPutResponse\"\0\x12J\n\x0bRawBatchPut\x12\x1b.kvrpcpb.RawBatchPutReques\
    t\x1a\x1c.kvrpcpb.RawBatchPutResponse\"\0\x12D\n\tRawDelete\x12\x19.kvrp\
    cpb.RawDeleteRequest\x1a\x1a.kvrpcpb.RawDeleteResponse\"\0\x12S\n\x0eRaw\
    BatchDelete\x12\x1e.kvrpcpb.RawBatchDeleteRequest\x1a\x1f.kvrpcpb.RawBat\
    chDeleteResponse\"\0\x12>\n\x07RawScan\x12\x17.kvrpcpb.RawScanRequest\
    \x1a\x18.kvrpcpb.RawScanResponse\"\0\x12S\n\x0eRawDeleteRange\x12\x1e.kv\
    rpcpb.RawDeleteRangeRequest\x1a\x1f.kvrpcpb.RawDeleteRangeResponse\"\0\
    \x12M\n\x0cRawBatchScan\x12\x1c.kvrpcpb.RawBatchScanRequest\x1a\x1d.kvrp\
    cpb.RawBatchScanResponse\"\0\x12_\n\x12UnsafeDestroyRange\x12\".kvrpcpb.\
    UnsafeDestroyRangeRequest\x1a#.kvrpcpb.UnsafeDestroyRangeResponse\"\0\
    \x12<\n\x0bCoprocessor\x12\x14.coprocessor.Request\x1a\x15.coprocessor.R\
    esponse\"\0\x12D\n\x11CoprocessorStream\x12\x14.coprocessor.Request\x1a\
    \x15.coprocessor.Response\"\00\x01\x12;\n\x04Raft\x12\x1a.raft_serverpb.\
    RaftMessage\x1a\x13.raft_serverpb.Done\"\0(\x01\x12A\n\x08Snapshot\x12\
    \x1c.raft_serverpb.SnapshotChunk\x1a\x13.raft_serverpb.Done\"\0(\x01\x12\
    J\n\x0bSplitRegion\x12\x1b.kvrpcpb.SplitRegionRequest\x1a\x1c.kvrpcpb.Sp\
    litRegionResponse\"\0\x12M\n\x0cMvccGetByKey\x12\x1c.kvrpcpb.MvccGetByKe\
    yRequest\x1a\x1d.kvrpcpb.MvccGetByKeyResponse\"\0\x12Y\n\x10MvccGetBySta\
    rtTs\x12\x20.kvrpcpb.MvccGetByStartTsRequest\x1a!.kvrpcpb.MvccGetByStart\
    TsResponse\"\0B\x1e\n\x10org.tikv.kvproto\xe0\xe2\x1e\x01\xc8\xe2\x1e\
    \x01\xd0\xe2\x1e\x01J\xa1\x12\n\x06\x12\x04\0\0;\x01\n\x08\n\x01\x0c\x12\
    \x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x01\x08\x0e\n\t\n\x02\x03\0\x12\x03\
    \x03\x07\x1a\n\t\n\x02\x03\x01\x12\x03\x04\x07\x16\n\t\n\x02\x03\x02\x12\
    \x03\x05\x07\x1c\n\t\n\x02\x03\x03\x12\x03\x07\x07\x1d\n\x08\n\x01\x08\
    \x12\x03\t\0$\n\x0b\n\x04\x08\xe7\x07\0\x12\x03\t\0$\n\x0c\n\x05\x08\xe7\
    \x07\0\x02\x12\x03\t\x07\x1c\n\r\n\x06\x08\xe7\x07\0\x02\0\x12\x03\t\x07\
    \x1c\n\x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\t\x08\x1b\n\x0c\n\x05\
    \x08\xe7\x07\0\x03\x12\x03\t\x1f#\n\x08\n\x01\x08\x12\x03\n\0(\n\x0b\n\
    \x04\x08\xe7\x07\x01\x12\x03\n\0(\n\x0c\n\x05\x08\xe7\x07\x01\x02\x12\
    \x03\n\x07\x20\n\r\n\x06\x08\xe7\x07\x01\x02\0\x12\x03\n\x07\x20\n\x0e\n\
    \x07\x08\xe7\x07\x01\x02\0\x01\x12\x03\n\x08\x1f\n\x0c\n\x05\x08\xe7\x07\
    \x01\x03\x12\x03\n#'\n\x08\n\x01\x08\x12\x03\x0b\0*\n\x0b\n\x04\x08\xe7\
    \x07\x02\x12\x03\x0b\0*\n\x0c\n\x05\x08\xe7\x07\x02\x02\x12\x03\x0b\x07\
    \"\n\r\n\x06\x08\xe7\x07\x02\x02\0\x12\x03\x0b\x07\"\n\x0e\n\x07\x08\xe7\
    \x07\x02\x02\0\x01\x12\x03\x0b\x08!\n\x0c\n\x05\x08\xe7\x07\x02\x03\x12\
    \x03\x0b%)\n\x08\n\x01\x08\x12\x03\r\0)\n\x0b\n\x04\x08\xe7\x07\x03\x12\
    \x03\r\0)\n\x0c\n\x05\x08\xe7\x07\x03\x02\x12\x03\r\x07\x13\n\r\n\x06\
    \x08\xe7\x07\x03\x02\0\x12\x03\r\x07\x13\n\x0e\n\x07\x08\xe7\x07\x03\x02\
    \0\x01\x12\x03\r\x07\x13\n\x0c\n\x05\x08\xe7\x07\x03\x07\x12\x03\r\x16(\
    \n1\n\x02\x06\0\x12\x04\x10\0;\x01\x1a%\x20Serve\x20as\x20a\x20distribut\
    ed\x20kv\x20database.\n\n\n\n\x03\x06\0\x01\x12\x03\x10\x08\x0c\n3\n\x04\
    \x06\0\x02\0\x12\x03\x12\x04B\x1a&\x20KV\x20commands\x20with\x20mvcc/txn\
    \x20supported.\n\n\x0c\n\x05\x06\0\x02\0\x01\x12\x03\x12\x08\r\n\x0c\n\
    \x05\x06\0\x02\0\x02\x12\x03\x12\x0e\x20\n\x0c\n\x05\x06\0\x02\0\x03\x12\
    \x03\x12+>\n\x0b\n\x04\x06\0\x02\x01\x12\x03\x13\x04E\n\x0c\n\x05\x06\0\
    \x02\x01\x01\x12\x03\x13\x08\x0e\n\x0c\n\x05\x06\0\x02\x01\x02\x12\x03\
    \x13\x0f\"\n\x0c\n\x05\x06\0\x02\x01\x03\x12\x03\x13-A\n\x0b\n\x04\x06\0\
    \x02\x02\x12\x03\x14\x04Q\n\x0c\n\x05\x06\0\x02\x02\x01\x12\x03\x14\x08\
    \x12\n\x0c\n\x05\x06\0\x02\x02\x02\x12\x03\x14\x13*\n\x0c\n\x05\x06\0\
    \x02\x02\x03\x12\x03\x145M\n\x0b\n\x04\x06\0\x02\x03\x12\x03\x15\x04K\n\
    \x0c\n\x05\x06\0\x02\x03\x01\x12\x03\x15\x08\x10\n\x0c\n\x05\x06\0\x02\
    \x03\x02\x12\x03\x15\x11&\n\x0c\n\x05\x06\0\x02\x03\x03\x12\x03\x151G\n\
    \x0b\n\x04\x06\0\x02\x04\x12\x03\x16\x04K\n\x0c\n\x05\x06\0\x02\x04\x01\
    \x12\x03\x16\x08\x10\n\x0c\n\x05\x06\0\x02\x04\x02\x12\x03\x16\x11&\n\
    \x0c\n\x05\x06\0\x02\x04\x03\x12\x03\x161G\n\x0b\n\x04\x06\0\x02\x05\x12\
    \x03\x17\x04N\n\x0c\n\x05\x06\0\x02\x05\x01\x12\x03\x17\x08\x11\n\x0c\n\
    \x05\x06\0\x02\x05\x02\x12\x03\x17\x12(\n\x0c\n\x05\x06\0\x02\x05\x03\
    \x12\x03\x173J\n\x0b\n\x04\x06\0\x02\x06\x12\x03\x18\x04Q\n\x0c\n\x05\
    \x06\0\x02\x06\x01\x12\x03\x18\x08\x12\n\x0c\n\x05\x06\0\x02\x06\x02\x12\
    \x03\x18\x13*\n\x0c\n\x05\x06\0\x02\x06\x03\x12\x03\x185M\n\x0b\n\x04\
    \x06\0\x02\x07\x12\x03\x19\x04`\n\x0c\n\x05\x06\0\x02\x07\x01\x12\x03\
    \x19\x08\x17\n\x0c\n\x05\x06\0\x02\x07\x02\x12\x03\x19\x184\n\x0c\n\x05\
    \x06\0\x02\x07\x03\x12\x03\x19?\\\n\x0b\n\x04\x06\0\x02\x08\x12\x03\x1a\
    \x04Q\n\x0c\n\x05\x06\0\x02\x08\x01\x12\x03\x1a\x08\x12\n\x0c\n\x05\x06\
    \0\x02\x08\x02\x12\x03\x1a\x13*\n\x0c\n\x05\x06\0\x02\x08\x03\x12\x03\
    \x1a5M\n\x0b\n\x04\x06\0\x02\t\x12\x03\x1b\x04Z\n\x0c\n\x05\x06\0\x02\t\
    \x01\x12\x03\x1b\x08\x15\n\x0c\n\x05\x06\0\x02\t\x02\x12\x03\x1b\x160\n\
    \x0c\n\x05\x06\0\x02\t\x03\x12\x03\x1b;V\n\x0b\n\x04\x06\0\x02\n\x12\x03\
    \x1c\x04?\n\x0c\n\x05\x06\0\x02\n\x01\x12\x03\x1c\x08\x0c\n\x0c\n\x05\
    \x06\0\x02\n\x02\x12\x03\x1c\r\x1e\n\x0c\n\x05\x06\0\x02\n\x03\x12\x03\
    \x1c);\n\x0b\n\x04\x06\0\x02\x0b\x12\x03\x1d\x04Z\n\x0c\n\x05\x06\0\x02\
    \x0b\x01\x12\x03\x1d\x08\x15\n\x0c\n\x05\x06\0\x02\x0b\x02\x12\x03\x1d\
    \x160\n\x0c\n\x05\x06\0\x02\x0b\x03\x12\x03\x1d;V\n\x1e\n\x04\x06\0\x02\
    \x0c\x12\x03\x20\x04I\x1a\x11\x20RawKV\x20commands.\n\n\x0c\n\x05\x06\0\
    \x02\x0c\x01\x12\x03\x20\x08\x0e\n\x0c\n\x05\x06\0\x02\x0c\x02\x12\x03\
    \x20\x0f$\n\x0c\n\x05\x06\0\x02\x0c\x03\x12\x03\x20/E\n\x0b\n\x04\x06\0\
    \x02\r\x12\x03!\x04X\n\x0c\n\x05\x06\0\x02\r\x01\x12\x03!\x08\x13\n\x0c\
    \n\x05\x06\0\x02\r\x02\x12\x03!\x14.\n\x0c\n\x05\x06\0\x02\r\x03\x12\x03\
    !9T\n\x0b\n\x04\x06\0\x02\x0e\x12\x03\"\x04I\n\x0c\n\x05\x06\0\x02\x0e\
    \x01\x12\x03\"\x08\x0e\n\x0c\n\x05\x06\0\x02\x0e\x02\x12\x03\"\x0f$\n\
    \x0c\n\x05\x06\0\x02\x0e\x03\x12\x03\"/E\n\x0b\n\x04\x06\0\x02\x0f\x12\
    \x03#\x04X\n\x0c\n\x05\x06\0\x02\x0f\x01\x12\x03#\x08\x13\n\x0c\n\x05\
    \x06\0\x02\x0f\x02\x12\x03#\x14.\n\x0c\n\x05\x06\0\x02\x0f\x03\x12\x03#9\
    T\n\x0b\n\x04\x06\0\x02\x10\x12\x03$\x04R\n\x0c\n\x05\x06\0\x02\x10\x01\
    \x12\x03$\x08\x11\n\x0c\n\x05\x06\0\x02\x10\x02\x12\x03$\x12*\n\x0c\n\
    \x05\x06\0\x02\x10\x03\x12\x03$5N\n\x0b\n\x04\x06\0\x02\x11\x12\x03%\x04\
    a\n\x0c\n\x05\x06\0\x02\x11\x01\x12\x03%\x08\x16\n\x0c\n\x05\x06\0\x02\
    \x11\x02\x12\x03%\x174\n\x0c\n\x05\x06\0\x02\x11\x03\x12\x03%?]\n\x0b\n\
    \x04\x06\0\x02\x12\x12\x03&\x04L\n\x0c\n\x05\x06\0\x02\x12\x01\x12\x03&\
    \x08\x0f\n\x0c\n\x05\x06\0\x02\x12\x02\x12\x03&\x10&\n\x0c\n\x05\x06\0\
    \x02\x12\x03\x12\x03&1H\n\x0b\n\x04\x06\0\x02\x13\x12\x03'\x04a\n\x0c\n\
    \x05\x06\0\x02\x13\x01\x12\x03'\x08\x16\n\x0c\n\x05\x06\0\x02\x13\x02\
    \x12\x03'\x174\n\x0c\n\x05\x06\0\x02\x13\x03\x12\x03'?]\n\x0b\n\x04\x06\
    \0\x02\x14\x12\x03(\x04[\n\x0c\n\x05\x06\0\x02\x14\x01\x12\x03(\x08\x14\
    \n\x0c\n\x05\x06\0\x02\x14\x02\x12\x03(\x150\n\x0c\n\x05\x06\0\x02\x14\
    \x03\x12\x03(;W\nJ\n\x04\x06\0\x02\x15\x12\x03+\x04m\x1a=\x20Store\x20co\
    mmands\x20(to\x20the\x20whole\x20tikv\x20but\x20not\x20a\x20certain\x20r\
    egion)\n\n\x0c\n\x05\x06\0\x02\x15\x01\x12\x03+\x08\x1a\n\x0c\n\x05\x06\
    \0\x02\x15\x02\x12\x03+\x1b<\n\x0c\n\x05\x06\0\x02\x15\x03\x12\x03+Gi\n&\
    \n\x04\x06\0\x02\x16\x12\x03.\x04J\x1a\x19\x20SQL\x20push\x20down\x20com\
    mands.\n\n\x0c\n\x05\x06\0\x02\x16\x01\x12\x03.\x08\x13\n\x0c\n\x05\x06\
    \0\x02\x16\x02\x12\x03.\x14'\n\x0c\n\x05\x06\0\x02\x16\x03\x12\x03.2F\n\
    \x0b\n\x04\x06\0\x02\x17\x12\x03/\x04X\n\x0c\n\x05\x06\0\x02\x17\x01\x12\
    \x03/\x08\x19\n\x0c\n\x05\x06\0\x02\x17\x02\x12\x03/\x1a-\n\x0c\n\x05\
    \x06\0\x02\x17\x06\x12\x03/9?\n\x0c\n\x05\x06\0\x02\x17\x03\x12\x03/@T\n\
    -\n\x04\x06\0\x02\x18\x12\x032\x04N\x1a\x20\x20Raft\x20commands\x20(tikv\
    \x20<->\x20tikv).\n\n\x0c\n\x05\x06\0\x02\x18\x01\x12\x032\x08\x0c\n\x0c\
    \n\x05\x06\0\x02\x18\x05\x12\x032\r\x13\n\x0c\n\x05\x06\0\x02\x18\x02\
    \x12\x032\x14-\n\x0c\n\x05\x06\0\x02\x18\x03\x12\x0328J\n\x0b\n\x04\x06\
    \0\x02\x19\x12\x033\x04T\n\x0c\n\x05\x06\0\x02\x19\x01\x12\x033\x08\x10\
    \n\x0c\n\x05\x06\0\x02\x19\x05\x12\x033\x11\x17\n\x0c\n\x05\x06\0\x02\
    \x19\x02\x12\x033\x183\n\x0c\n\x05\x06\0\x02\x19\x03\x12\x033>P\n\x1f\n\
    \x04\x06\0\x02\x1a\x12\x036\x04Y\x1a\x12\x20Region\x20commands.\n\n\x0c\
    \n\x05\x06\0\x02\x1a\x01\x12\x036\x08\x13\n\x0c\n\x05\x06\0\x02\x1a\x02\
    \x12\x036\x15/\n\x0c\n\x05\x06\0\x02\x1a\x03\x12\x036:U\n-\n\x04\x06\0\
    \x02\x1b\x12\x039\x04[\x1a\x20\x20transaction\x20debugger\x20commands.\n\
    \n\x0c\n\x05\x06\0\x02\x1b\x01\x12\x039\x08\x14\n\x0c\n\x05\x06\0\x02\
    \x1b\x02\x12\x039\x150\n\x0c\n\x05\x06\0\x02\x1b\x03\x12\x039;W\n\x0b\n\
    \x04\x06\0\x02\x1c\x12\x03:\x04g\n\x0c\n\x05\x06\0\x02\x1c\x01\x12\x03:\
    \x08\x18\n\x0c\n\x05\x06\0\x02\x1c\x02\x12\x03:\x198\n\x0c\n\x05\x06\0\
    \x02\x1c\x03\x12\x03:Ccb\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
