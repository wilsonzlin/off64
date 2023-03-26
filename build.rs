use std::env::var;
use std::fmt::Write as FmtWrite;
use std::fs::File;
use std::io::Write as IoWrite;
use std::path::PathBuf;
use std::str::FromStr;

fn main() {
  let mut rs_trait = "pub trait Off64Int {\n".to_string();
  let mut rs_impl = "impl Off64Int for [u8] {\n".to_string();
  let mut rs_create = "".to_string();
  let widths = [16, 32, 64];
  for b in [16, 24, 32, 40, 48, 56, 64] {
    let bytes = b / 8;
    let w = widths[match widths.binary_search(&b) {
      Ok(v) | Err(v) => v,
    }];
    let width_bytes = w / 8;
    for s in ['i', 'u'] {
      for e in ['b', 'l'] {
        writeln!(
          &mut rs_trait,
          "fn read_{s}{b}_{e}e_at(&self, offset: u64) -> {s}{w};"
        )
        .unwrap();
        writeln!(
          &mut rs_trait,
          "fn write_{s}{b}_{e}e_at(&mut self, offset: u64, value: {s}{w});"
        )
        .unwrap();
        let (buf_start, buf_end) = match e {
          'b' => (width_bytes - bytes, width_bytes),
          'l' => (0, bytes),
          _ => unreachable!(),
        };
        write!(
          &mut rs_impl,
          "
          fn read_{s}{b}_{e}e_at(&self, offset: u64) -> {s}{w} {{
            let offset = usz!(offset);
            let mut buf = [0u8; {width_bytes}];
            buf[{buf_start}..{buf_end}].copy_from_slice(
              &self[offset..offset + {bytes}],
            );
            {s}{w}::from_{e}e_bytes(buf)
          }}
        "
        )
        .unwrap();
        write!(
          &mut rs_impl,
          "
          fn write_{s}{b}_{e}e_at(&mut self, offset: u64, value: {s}{w}) {{
            let offset = usz!(offset);
            let buf = value.to_{e}e_bytes();
            self[offset..offset + {bytes}].copy_from_slice(&buf[{buf_start}..{buf_end}]);
          }}
        "
        )
        .unwrap();
        write!(
          &mut rs_create,
          "
          pub fn create_{s}{b}_{e}e(value: {s}{w}) -> [u8; {bytes}] {{
            let buf = value.to_{e}e_bytes();
            buf[{buf_start}..{buf_end}].try_into().unwrap()
          }}
        "
        )
        .unwrap();
      }
    }
  }
  rs_trait.push_str("}\n\n");
  rs_impl.push_str("}\n\n");

  let mut f = File::create(
    PathBuf::from_str(&var("OUT_DIR").unwrap())
      .unwrap()
      .join("ints.rs"),
  )
  .unwrap();
  f.write_all(rs_trait.as_bytes()).unwrap();
  f.write_all(rs_impl.as_bytes()).unwrap();
  f.write_all(rs_create.as_bytes()).unwrap();
}
