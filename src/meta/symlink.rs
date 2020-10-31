use crate::color::{ColoredString, Colors, Elem};
use crate::flags::Styles;
use ansi_term::{ANSIString, ANSIStrings};
use std::fs::read_link;
use std::path::Path;

#[derive(Clone, Debug)]
pub struct SymLink {
    target: Option<String>,
    valid: bool,
}

impl<'a> From<&'a Path> for SymLink {
    fn from(path: &'a Path) -> Self {
        if let Ok(target) = read_link(path) {
            if target.is_absolute() || path.parent() == None {
                return Self {
                    valid: target.exists(),
                    target: Some(
                        target
                            .to_str()
                            .expect("failed to convert symlink to str")
                            .to_string(),
                    ),
                };
            }

            return Self {
                target: Some(
                    target
                        .to_str()
                        .expect("failed to convert symlink to str")
                        .to_string(),
                ),
                valid: path.parent().unwrap().join(target).exists(),
            };
        }

        Self {
            target: None,
            valid: false,
        }
    }
}

impl SymLink {
    pub fn symlink_string(&self) -> Option<String> {
        if let Some(ref target) = self.target {
            Some(target.to_string())
        } else {
            None
        }
    }

    pub fn render(&self, colors: &Colors, styles: &Styles) -> ColoredString {
        if let Some(target_string) = self.symlink_string() {
            let elem = if self.valid {
                &Elem::SymLink
            } else {
                &Elem::BrokenSymLink
            };

            let strings: &[ColoredString] = &[
                ColoredString::from(format!(" {} ", styles.symlink_arrow)), // ⇒ \u{21d2}
                colors.colorize(target_string, elem),
            ];

            let res = ANSIStrings(strings).to_string();
            ColoredString::from(res)
        } else {
            ANSIString::from("")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Styles;
    use super::SymLink;
    use crate::color::{Colors, Theme};
    use crate::flags::styling::SymlinkArrow;

    #[test]
    fn test_symlink_render_default_valid_target_nocolor() {
        let link = SymLink {
            target: Some("/target".to_string()),
            valid: true,
        };
        assert_eq!(
            format!("{}", " ⇒ /target"),
            link.render(
                &Colors::new(Theme::NoColor),
                &Styles {
                    symlink_arrow: SymlinkArrow::default()
                }
            )
            .to_string()
        );
    }

    #[test]
    fn test_symlink_render_default_invalid_target_nocolor() {
        let link = SymLink {
            target: Some("/target".to_string()),
            valid: false,
        };
        assert_eq!(
            format!("{}", " ⇒ /target"),
            link.render(
                &Colors::new(Theme::NoColor),
                &Styles {
                    symlink_arrow: SymlinkArrow::default()
                }
            )
            .to_string()
        );
    }
}
