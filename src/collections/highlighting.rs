pub static HIGHLIGHTING: &'static [(&'static str, &'static str)] =
  &[ (r"(^|\W)([a-zA-Z0-9\s_\\.\-\(\):])+[.]rs($|\W)", "rust")
   , (r"(^|\W)([a-zA-Z0-9\s_\\.\-\(\):])+[.]cs($|\W)", "csharp")
   , (r"(^|\W)([a-zA-Z0-9\s_\\.\-\(\):])+[.]c($|\W)", "c")
   , (r"(^|\W)([a-zA-Z0-9\s_\\.\-\(\):])+[.](cpp|cxx|h)($|\W)", "cpp")
   , (r"(^|\W)([a-zA-Z0-9\s_\\.\-\(\):])+[.]hs($|\W)", "haskell")
   , (r"(^|\W)([a-zA-Z0-9\s_\\.\-\(\):])+[.]html($|\W)", "html")
   , (r"(^|\W)([a-zA-Z0-9\s_\\.\-\(\):])+[.]js($|\W)", "js")
   , (r"(^|\W)([a-zA-Z0-9\s_\\.\-\(\):])+[.]css($|\W)", "css")
   , (r"(^|\W)([a-zA-Z0-9\s_\\.\-\(\):])+(CmakeList|[.]cmake)($|\W)", "cmake")
   , (r"(^|\W)([a-zA-Z0-9\s_\\.\-\(\):])+[.]fs($|\W)", "fsharp")
   , (r"(^|\W)([a-zA-Z0-9\s_\\.\-\(\):])+[.](sh|bash|zsh|run|ebuild|eclass)($|\W)", "shell")
   , (r"(^|\W)([a-zA-Z0-9\s_\\.\-\(\):])+[.]patch($|\W)", "diff")
   ];
