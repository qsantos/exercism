pub fn get_diamond(c: char) -> Vec<String> {
    let size = (c as u8 - b'A') as usize;
    let mut ret = Vec::new();
    ret.push(" ".repeat(size as usize) + "A" + &" ".repeat(size as usize));
    for c in 'B'..=c {
        let row_number = (c as u8 - b'A') as usize;
        let separation = " ".repeat(row_number * 2 - 1);
        let margin = " ".repeat(size - row_number);
        let mut row = margin.clone();
        row.push(c);
        row += &separation;
        row.push(c);
        row += &margin;
        ret.push(row);
    }
    if size > 0 {
        for row_number in (0..size).rev() {
            ret.push(ret[row_number].clone());
        }
    }
    ret
}
