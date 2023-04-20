use crate::int::create_u24_le;
use crate::int::Off64Int;

#[test]
fn test_int_functions() {
  let mut raw = create_u24_le(987654).to_vec();
  assert_eq!(raw, [0x06, 0x12, 0x0f]);
  assert_eq!(raw.read_u24_le_at(0), 987654);
  assert_eq!(raw.read_u24_be_at(0), 397839);
  raw.write_i16_be_at(1, 1000);
  assert_eq!(raw, [0x06, 0x03, 0xe8]);

  raw.push(0);
  raw.push(0);
  raw.push(0);
  raw.push(0);
  raw.write_u48_be_at(1, 115922130);
}
