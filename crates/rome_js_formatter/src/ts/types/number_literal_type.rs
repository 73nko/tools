use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use rome_formatter::write;
use rome_js_syntax::{JsSyntaxNode, TsNumberLiteralType, TsNumberLiteralTypeFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsNumberLiteralType;

impl FormatNodeRule<TsNumberLiteralType> for FormatTsNumberLiteralType {
    fn fmt_fields(&self, node: &TsNumberLiteralType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsNumberLiteralTypeFields {
            minus_token,
            literal_token,
        } = node.as_fields();
        write![f, [minus_token.format(), literal_token.format()]]
    }

    fn needs_parentheses(&self, item: &TsNumberLiteralType) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for TsNumberLiteralType {
    fn needs_parentheses_with_parent(&self, _: &JsSyntaxNode) -> bool {
        false
    }
}
