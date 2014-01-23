use std::str;
use std::os;
use std::io::File;

fn main() {
    let args: ~[~str] = os::args();
    if args.len() != 3 {
        println!("Usage: {:s} <inputfile>", args[0]); 
    } else {
        let tempArg = &args; 
        let fname = &tempArg[1];
        let fname2 = &tempArg[2];
        
        let path = Path::new(fname.clone());
        let msg_file = File::open(&path);

        let path2 = Path::new(fname2.clone());
        let msg_file2 = File::open(&path2);

        match (msg_file, msg_file2) {
            (Some(mut msg), Some(mut msg2)) => {
                let msg_bytes: ~[u8] = msg.read_to_end();
                let msg_bytes2: ~[u8] = msg2.read_to_end();
                join(msg_bytes, msg_bytes2);
            } ,
            (_,_) => fail!("Error opening message files: {:s} and {:s}", *fname, *fname2)
        }
    }
}

fn xor(a: &[u8], b: &[u8]) -> ~[u8] {
    let mut ret = ~[];
    for i in range(0, a.len()) {
    ret.push(a[i] ^ b[i]);
    }
    ret
}

fn join(msg_bytes: &[u8], msg_bytes2: &[u8]) {
    let undoEncrypt = xor(msg_bytes, msg_bytes2);
    println!("{}", str::from_utf8(undoEncrypt));
}