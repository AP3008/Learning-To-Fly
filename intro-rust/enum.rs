fn main(){
    enum IpAddrKind{
        V4,
        V6
    }

    let _four: IpAddrKind = IpAddrKind::V4;
    let _six: IpAddrKind = IpAddrKind::V6;

    struct IpAddr{
        kind: IpAddrKind,
        address: String
    }

    let home: IpAddr = IpAddr{
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };


}
