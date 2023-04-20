class ManyTraits:
  def __init__(self, parent_trait_suffix: str, parent_trait_write_generic_arguments: str, trait_name_suffix: str):
    self.parent_trait_suffix = parent_trait_suffix
    self.parent_trait_write_generic_arguments = parent_trait_write_generic_arguments
    self.trait_name_suffix = trait_name_suffix
    self.traits = {}

  def _insert_trait_method(self, k, code):
    if k not in self.traits:
      self.traits[k] = []
    self.traits[k].append(code)

  def add_read_methods(self, fn):
    for (async_, await_) in (("", ""), ("async", ".await")):
      self._insert_trait_method(("read", async_), fn(async_, await_))

  def add_write_methods(self, fn):
    for (async_, await_) in (("", ""), ("async", ".await")):
      for mut in ("", "mut"):
        self._insert_trait_method(("write", async_, mut), fn(async_, await_, mut))

  def output(self):
    pars = self.parent_trait_suffix
    parwa = self.parent_trait_write_generic_arguments
    sfx = self.trait_name_suffix
    out = ""

    inner = "\n".join(self.traits[("read", "")])
    out += f"pub trait Off64Read{sfx}<'a, T: 'a + AsRef<[u8]>>: Off64Read{pars}<'a, T> {{{inner}}}"

    inner = "\n".join(self.traits[("read", "async")])
    out += f"#[async_trait::async_trait] pub trait Off64AsyncRead{sfx}<'a, T: 'a + AsRef<[u8]>>: Off64AsyncRead{pars}<'a, T> {{{inner}}}"

    inner = "\n".join(self.traits[("write", "", "")])
    out += f"pub trait Off64Write{sfx}: for<'a> Off64Write{pars}{parwa} {{{inner}}}"

    inner = "\n".join(self.traits[("write", "async", "")])
    out += f"#[async_trait::async_trait] pub trait Off64AsyncWrite{sfx}: for<'a> Off64AsyncWrite{pars}{parwa} {{{inner}}}"

    inner = "\n".join(self.traits[("write", "", "mut")])
    out += f"pub trait Off64WriteMut{sfx}: for<'a> Off64WriteMut{pars}{parwa} {{{inner}}}"

    inner = "\n".join(self.traits[("write", "async", "mut")])
    out += f"#[async_trait::async_trait] pub trait Off64AsyncWriteMut{sfx}: for<'a> Off64AsyncWriteMut{pars}{parwa} {{{inner}}}"

    return out
