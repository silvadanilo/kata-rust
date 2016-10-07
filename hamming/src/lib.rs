pub fn hamming_distance(strands1: &str, strands2: &str) -> Result<u32, &'static str> {
    if strands1.len() != strands2.len() {
        return Err("sequences must have the same length");
    }

    // //#ZIP way
    let distance: u32 = strands1.chars()
        .zip(strands2.chars())
        .fold(0, |acc, (strands1_char, strands2_char)| {
              if strands1_char != strands2_char {
                  acc + 1
              } else {
                  acc
              }
        });

    Ok(distance)

    // //#FOR way
    // let mut distance: u32 = 0;

    // for (i, c) in strands1.chars().enumerate() {
    //     if c != strands2.chars().nth(i).unwrap() {
    //         distance += 1;
    //     }
    // }

    // Ok(distance)
}
