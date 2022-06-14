// extern "C" {
//     pub fn gethostname(name: *mut c_char, len: usize) -> c_int;
// }

pub mod socket;

fn main() {
    socket::socket::sock_test();
}
