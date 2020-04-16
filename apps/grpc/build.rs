fn main() {
    tonic_build::compile_protos("proto/routeguide/route_guide.proto").unwrap();
}
