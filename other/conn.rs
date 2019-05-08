enum IpVersion {
    V4,
    V6
}


struct Host{
    host: String
    IpAddr: IpVersion
}



fn main()i{


    let mut hostOne = Host{
        name: String::from("Localhost"),
        IpAddr: IpVersion::V4,
    }


    match hostOne.IpAddr {
        Err() => panic!("ERROR"),
        Ok(_) => println!("Version 4"),
    }

}
    
