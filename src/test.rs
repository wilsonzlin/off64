use super::*;

#[test]
fn test_i24_methods() {
  let mut raw = create_u24_le(987654);
  assert_eq!(raw, [0x06, 0x12, 0x0f]);
  assert_eq!(raw.read_u24_le_at(0), 987654);
  assert_eq!(raw.read_u24_be_at(0), 397839);
  raw.write_i16_be_at(1, 1000);
  assert_eq!(raw, [0x06, 0x03, 0xe8]);
}
