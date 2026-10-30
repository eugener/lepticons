#!/usr/bin/env bash
# Lists the public LucideGlyph names in a generated lucide_icon_data.rs,
# one per line as `variant <Name>` or `alias <Name>`.
# Usage: glyph-names.sh FILE
set -euo pipefail

awk '
  /^pub enum LucideGlyph \{/ { in_enum = 1; next }
  in_enum && /^\}/ { in_enum = 0 }
  in_enum && /^    [A-Z][A-Za-z0-9]*,$/ { gsub(/[ ,]/, ""); print "variant", $0 }
  /^ *pub const [A-Za-z0-9]+: Self = / { sub(/^ *pub const /, ""); sub(/:.*/, ""); print "alias", $0 }
' "$1"
