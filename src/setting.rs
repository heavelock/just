use crate::common::*;

#[derive(Debug, Clone)]
pub(crate) enum Setting<'src> {
  AllowDuplicateRecipes(bool),
  DotenvLoad(bool),
  Export(bool),
  PositionalArguments(bool),
  Shell(Shell<'src>),
  WindowsPowerShell(bool),
  DotenvFilenames(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub(crate) struct Shell<'src> {
  pub(crate) arguments: Vec<StringLiteral<'src>>,
  pub(crate) command: StringLiteral<'src>,
}

impl<'src> Display for Setting<'src> {
  fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
    match self {
      Setting::AllowDuplicateRecipes(value)
      | Setting::DotenvLoad(value)
      | Setting::Export(value)
      | Setting::PositionalArguments(value)
      | Setting::WindowsPowerShell(value) => write!(f, "{}", value),
      Setting::DotenvFilenames(value) => write!(f, "{:?}", value),
      Setting::Shell(shell) => write!(f, "{}", shell),
    }
  }
}

impl<'src> Display for Shell<'src> {
  fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
    write!(f, "[{}", self.command)?;

    for argument in &self.arguments {
      write!(f, ", {}", argument)?;
    }

    write!(f, "]")
  }
}
