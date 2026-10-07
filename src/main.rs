fn main() {
    let status: u32 = 1218042085;
    // 01001000 10011001 11011000 11100101
    
    // Left to right bits = first to last bits
    
    // 1 - 16 : server_id
    // 17 - 24 : server_passkey
    // 25 - 28 : gamerules on (which of the four game rules are on)
    // 29 - 30 : gamemode id
    // 31 : server_full (0 for no, 1 for yes)
    // 32 : endianness (0 for big, 1 for little)
    
    
    let gamemode = get_gamemode(status);
    let gamerules = get_gamerules(status);
    let server_passkey = get_server_passkey(status);
    let server_id = get_server_id(status);
    
    println!("Server ID: {server_id}");
    println!("Server Game Mode: {gamemode}");
    println!("Server Host Rules: {gamerules}");
    println!("Server Passkey: {server_passkey}");
    println!("Server status: ");
    is_server_full(status);
}

fn endian_conversion (status: u16) -> u16 { 
    // assume that you simply want to convert 
    // the passed-in variable into big endian
    
    let le_bytes = status.to_le_bytes();
    // convert to array of bytes arranged in little endian format
    
    let be_value = u16::from_be_bytes(le_bytes);
    // converting the array into big endian format undoes the le-status from before
    
    return be_value;
    
}


fn is_server_full(status: u32) {

    let server_full = (status & 2) >> 1;
    // in binary 2 = 00000000 00000000 00000000 00000010
    // so this will only get the server_full flag
    
    if server_full == 1 {
        println!("This server is full!");
    } else {
        println!("Positions open!");
    }
}

fn get_gamemode (status: u32) -> u8 {
    let gamemode = (status & 12) >> 2;
    // in binary 12 = 00000000 00000000 00000000 00001100
    
    return gamemode as u8;
}

fn get_gamerules (status: u32) -> u8 {
    let gamerules = (status & 240) >> 4;
    // in binary 240 = 00000000 00000000 00000000 11110000
    
    return gamerules as u8;
}

fn get_server_passkey (status: u32) -> u8 {
    let server_passkey = (status & 65280) >> 8;
    // in binary 65280 = 00000000 00000000 11111111 00000000
    
    return server_passkey as u8;
}

fn get_server_id (status: u32) -> u16 {
    let server_id_bits = (status & 4294901760) >> 16;
    // in binary 4294901760 = 11111111 11111111 00000000 00000000
    
    let mut server_id = server_id_bits as u16;
    
    if (status & 1) == 1 { // if endian bit indicates little endian
        server_id = endian_conversion(server_id);
    }
    
    return server_id;
}
