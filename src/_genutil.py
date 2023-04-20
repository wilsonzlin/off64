class ManyTraits:
  def __init__(self, parent_trait_suffix: str, trait_name_suffix: str):
    self.parent_trait_suffix = parent_trait_suffix
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
    sfx = self.trait_name_suffix
    out = ""

    inner = "\n".join(self.traits[("read", "")])
    out += f"pub trait Off64Read{sfx}<'a, T: 'a + AsRef<[u8]>>: Off64Read{pars}<'a, T> {{{inner}}}"

    inner = "\n".join(self.traits[("read", "async")])
    out += f"#[async_trait::async_trait] pub trait Off64AsyncRead{sfx}<'a, T: 'a + AsRef<[u8]>>: Off64AsyncRead{pars}<'a, T> {{{inner}}}"

    for async_ in (True, False):
      for mut in (True, False):
        async_attr = "#[async_trait::async_trait]" if async_ else ""
        async_name = "Async" if async_ else ""
        mut_name = "Mut" if mut else ""

        inner = "\n".join(self.traits[("write", "async" if async_ else "", "mut" if mut else "")])
        out += f"{async_attr} pub trait Off64{async_name}Write{mut_name}{sfx}: Off64{async_name}Write{mut_name}{pars} {{{inner}}}"

    return out
