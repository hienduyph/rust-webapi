fn main() {
    tonic_build::compile_protos("src/apps/grpc/proto/routeguide/route_guide.proto").unwrap();
}
