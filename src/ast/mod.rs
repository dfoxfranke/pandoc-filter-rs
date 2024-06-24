//! Abstract syntax trees for Pandoc documents

use ::serde::de::{MapAccess, SeqAccess, Visitor};
use ::serde::ser::{SerializeMap, SerializeStruct};
use ::serde::{Deserialize, Deserializer, Serialize, Serializer};
use im_rope::Rope;
use imbl::{HashMap, Vector};
use never::Never;
use std::cell::Cell as StdCell;
use std::error::Error;
use std::fmt::{Debug, Display};
use std::marker::PhantomData;
use std::panic::AssertUnwindSafe;
use std::vec::Vec;

use crate::interned::InternedString;

mod serde;

/// This crate's supported API version.
///
/// The types defined in this crate correspond to those which are defined in the
/// version of the Haskell
/// [`pandoc-types`](https://hackage.haskell.org/package/pandoc-types) package
/// given by this constant.
pub const API_VERSION: &[u32] = &[1, 23, 1];

/// Representation of a literal space in a condensed leaf.
pub const CONDENSED_LITERAL_SPACE: char = '\u{1fffe}';
/// Representation of a literal carriage return in a condensed leaf.
pub const CONDENSED_LITERAL_CR: char = '\u{1ffff}';
/// Representation of a literal newline in a condensed leaf.
pub const CONDENSED_LITERAL_NEWLINE: char = '\u{2fffe}';

pub trait DecorationScheme {
    type Pandoc: Clone;
    type MetaValue: Clone;
    type Blocks: Clone;
    type Block: Clone;
    type Inlines: Clone;
    type Inline: Clone;
    type Caption: Clone;
    type Citation: Clone;
    type Table: Clone;
    type TableHead: Clone;
    type TableBody: Clone;
    type TableFoot: Clone;
    type Row: Clone;
    type Cell: Clone;
}

/// A decoration scheme in which all decorations have the same type.
///
/// This type is never constructed. It appears only as a paramater to other types.
pub struct SimpleScheme<A>(PhantomData<A>);

/// A decoration scheme in which no decorations are present.
pub type NullScheme = SimpleScheme<Never>;

/// Top level of the Pandoc AST.
#[derive(Educe)]
#[educe(
    Debug(
        bound = "A::Pandoc : Debug, A::MetaValue : Debug, A::Blocks : Debug, A::Block : Debug, A::Inlines : Debug, A::Inline : Debug, A::Caption : Debug, A::Citation : Debug, A::Table : Debug, A::TableHead : Debug, A::TableBody : Debug, A::TableFoot : Debug, A::Row : Debug, A:: Cell : Debug"
    ),
    Clone,
    PartialEq(
        bound = "A::Pandoc : PartialEq, A::MetaValue : PartialEq, A::Blocks : PartialEq, A::Block : PartialEq, A::Inlines : PartialEq, A::Inline : PartialEq, A::Caption : PartialEq, A::Citation : PartialEq, A::Table : PartialEq, A::TableHead : PartialEq, A::TableBody : PartialEq, A::TableFoot : PartialEq, A::Row : PartialEq, A:: Cell : PartialEq"
    )
)]
pub struct Pandoc<A>
where
    A: DecorationScheme,
{
    pub decoration: Option<A::Pandoc>,
    /// Metadata for the document: title, authors, date, etc.
    pub meta: HashMap<InternedString, MetaValue<A>>,
    /// The body of the document.
    pub blocks: Blocks<A>,
}

#[derive(Educe, Serialize, Deserialize)]
#[educe(
    Debug(
        bound = "A::MetaValue : Debug, A::Blocks : Debug, A::Block : Debug, A::Inlines : Debug, A::Inline : Debug, A::Caption : Debug, A::Citation : Debug, A::Table : Debug, A::TableHead : Debug, A::TableBody : Debug, A::TableFoot : Debug, A::Row : Debug, A:: Cell : Debug"
    ),
    Clone,
    PartialEq(
        bound = "A::MetaValue : PartialEq, A::Blocks : PartialEq, A::Block : PartialEq, A::Inlines : PartialEq, A::Inline : PartialEq, A::Caption : PartialEq, A::Citation : PartialEq, A::Table : PartialEq, A::TableHead : PartialEq, A::TableBody : PartialEq, A::TableFoot : PartialEq, A::Row : PartialEq, A:: Cell : PartialEq"
    )
)]
#[serde(
    from = "serde::MetaValueSerde<A>",
    into = "serde::MetaValueSerde<A>",
    bound = ""
)]
pub struct MetaValue<A>
where
    A: DecorationScheme,
{
    pub decoration: Option<A::MetaValue>,
    pub content: MetaValueContent<A>,
}

/// The value of a metadata entry.
#[derive(Educe, Serialize, Deserialize)]
#[educe(
    Debug(
        bound = "A::MetaValue : Debug, A::Blocks : Debug, A::Block : Debug, A::Inlines : Debug, A::Inline : Debug, A::Caption : Debug, A::Citation : Debug, A::Table : Debug, A::TableHead : Debug, A::TableBody : Debug, A::TableFoot : Debug, A::Row : Debug, A:: Cell : Debug"
    ),
    Clone,
    PartialEq(
        bound = "A::MetaValue : PartialEq, A::Blocks : PartialEq, A::Block : PartialEq, A::Inlines : PartialEq, A::Inline : PartialEq, A::Caption : PartialEq, A::Citation : PartialEq, A::Table : PartialEq, A::TableHead : PartialEq, A::TableBody : PartialEq, A::TableFoot : PartialEq, A::Row : PartialEq, A:: Cell : PartialEq"
    )
)]
#[serde(tag = "t", content = "c", bound = "")]
pub enum MetaValueContent<A>
where
    A: DecorationScheme,
{
    #[serde(rename = "MetaMap")]
    Map(HashMap<InternedString, MetaValue<A>>),
    #[serde(rename = "MetaList")]
    List(Vector<MetaValue<A>>),
    #[serde(rename = "MetaBool")]
    Bool(bool),
    #[serde(rename = "MetaString")]
    String(Rope),
    #[serde(rename = "MetaInlines")]
    Inlines(Inlines<A>),
    #[serde(rename = "MetaBlocks")]
    Blocks(Blocks<A>),
}

impl<A> From<MetaValue<A>> for MetaValueContent<A>
where
    A: DecorationScheme,
{
    fn from(value: MetaValue<A>) -> Self {
        value.content
    }
}

impl<A> From<MetaValueContent<A>> for MetaValue<A>
where
    A: DecorationScheme,
{
    fn from(value: MetaValueContent<A>) -> Self {
        MetaValue {
            decoration: None,
            content: value,
        }
    }
}

#[derive(Educe, Serialize, Deserialize)]
#[educe(
    Debug(
        bound = "A::Blocks : Debug, A::Block : Debug, A::Inlines : Debug, A::Inline : Debug, A::Caption : Debug, A::Citation : Debug, A::Table : Debug, A::TableHead : Debug, A::TableBody : Debug, A::TableFoot : Debug, A::Row : Debug, A:: Cell : Debug"
    ),
    Clone,
    PartialEq(
        bound = "A::Blocks : PartialEq, A::Block : PartialEq, A::Inlines : PartialEq, A::Inline : PartialEq, A::Caption : PartialEq, A::Citation : PartialEq, A::Table : PartialEq, A::TableHead : PartialEq, A::TableBody : PartialEq, A::TableFoot : PartialEq, A::Row : PartialEq, A:: Cell : PartialEq"
    )
)]
#[serde(
    from = "serde::BlocksSerde<A>",
    into = "serde::BlocksSerde<A>",
    bound = ""
)]
pub struct Blocks<A>
where
    A: DecorationScheme,
{
    pub decoration: Option<A::Blocks>,
    pub content: Vector<Block<A>>,
}

#[derive(Educe, Serialize, Deserialize)]
#[educe(
    Debug(
        bound = "A::Blocks : Debug, A::Block : Debug, A::Inlines : Debug, A::Inline : Debug, A::Caption : Debug, A::Citation : Debug, A::Table : Debug, A::TableHead : Debug, A::TableBody : Debug, A::TableFoot : Debug, A::Row : Debug, A:: Cell : Debug"
    ),
    Clone,
    PartialEq(
        bound = "A::Blocks : PartialEq, A::Block : PartialEq, A::Inlines : PartialEq, A::Inline : PartialEq, A::Caption : PartialEq, A::Citation : PartialEq, A::Table : PartialEq, A::TableHead : PartialEq, A::TableBody : PartialEq, A::TableFoot : PartialEq, A::Row : PartialEq, A:: Cell : PartialEq"
    )
)]
#[serde(
    from = "serde::BlockSerde<A>",
    into = "serde::BlockSerde<A>",
    bound = ""
)]
pub struct Block<A>
where
    A: DecorationScheme,
{
    pub decoration: Option<A::Block>,
    pub content: BlockContent<A>,
}

/// A block element.
#[derive(Educe, Serialize, Deserialize)]
#[educe(
    Debug(
        bound = "A::Blocks : Debug, A::Block : Debug, A::Inlines : Debug, A::Inline : Debug, A::Caption : Debug, A::Citation : Debug, A::Table : Debug, A::TableHead : Debug, A::TableBody : Debug, A::TableFoot : Debug, A::Row : Debug, A:: Cell : Debug"
    ),
    Clone,
    PartialEq(
        bound = "A::Blocks : PartialEq, A::Block : PartialEq, A::Inlines : PartialEq, A::Inline : PartialEq, A::Caption : PartialEq, A::Citation : PartialEq, A::Table : PartialEq, A::TableHead : PartialEq, A::TableBody : PartialEq, A::TableFoot : PartialEq, A::Row : PartialEq, A:: Cell : PartialEq"
    )
)]
#[serde(tag = "t", content = "c", bound = "")]
pub enum BlockContent<A>
where
    A: DecorationScheme,
{
    /// Plain text, not a paragraph.
    Plain(Inlines<A>),
    /// Paragraph text.
    Para(Inlines<A>),
    /// Multiple non-breaking lines.
    LineBlock(Vector<Inlines<A>>),
    /// A block of code, with attributes attached.
    CodeBlock(Attr, Rope),
    /// A raw block.
    RawBlock(InternedString, Rope),
    /// A block quote.
    BlockQuote(Blocks<A>),
    /// An ordered list (attributes and a vector of items, each a vector of blocks).
    OrderedList(ListAttributes, Vector<Blocks<A>>),
    /// A bulleted list (vector of items, each a vector of blocks).
    BulletList(Vector<Blocks<A>>),
    /// A definition list. Each list item is a pair consisting of a term (a
    /// sequence inlines) and one or more definitions (each a vector of blocks).
    DefinitionList(Vector<(Inlines<A>, Vector<Blocks<A>>)>),
    /// A header: level, attributes, and content.
    Header(i32, Attr, Inlines<A>),
    /// A horizontal rule.
    HorizontalRule,
    /// A table.
    Table(Table<A>),
    /// A figure: attributes, caption, and content.
    Figure(Attr, Caption<A>, Blocks<A>),
    // Generic block container, with attributes.
    Div(Attr, Blocks<A>),
}

/// A sequence of [`Inline`]s.
///
/// Inlines have two available representations, called "expanded" and
/// "condensed". The expanded representation is the one that Pandoc uses
/// natively, wherein each word and each break between words is represented by a
/// separate element. The condensed representation attempts to be more efficient
/// by using ordinary Unicode characters to represent word breaks, so that an
/// entire paragraph (if it is free of markup) can be represented by a single
/// rope. For the details of this representation, see the documentation of
/// [`CondensedLeaf`].
#[derive(Educe)]
#[educe(
    Debug(
        bound = "A::Blocks : Debug, A::Block : Debug, A::Inlines : Debug, A::Inline : Debug, A::Caption : Debug, A::Citation : Debug, A::Table : Debug, A::TableHead : Debug, A::TableBody : Debug, A::TableFoot : Debug, A::Row : Debug, A:: Cell : Debug"
    ),
    Clone,
    PartialEq(
        bound = "A::Blocks : PartialEq, A::Block : PartialEq, A::Inlines : PartialEq, A::Inline : PartialEq, A::Caption : PartialEq, A::Citation : PartialEq, A::Table : PartialEq, A::TableHead : PartialEq, A::TableBody : PartialEq, A::TableFoot : PartialEq, A::Row : PartialEq, A:: Cell : PartialEq"
    )
)]
pub struct Inlines<A>
where
    A: DecorationScheme,
{
    pub decoration: Option<A::Inlines>,
    pub content: InlinesContent<A>,
}

#[derive(Educe)]
#[educe(
    Debug(
        bound = "A::Blocks : Debug, A::Block : Debug, A::Inlines : Debug, A::Inline : Debug, A::Caption : Debug, A::Citation : Debug, A::Table : Debug, A::TableHead : Debug, A::TableBody : Debug, A::TableFoot : Debug, A::Row : Debug, A:: Cell : Debug"
    ),
    Clone,
    PartialEq(
        bound = "A::Blocks : PartialEq, A::Block : PartialEq, A::Inlines : PartialEq, A::Inline : PartialEq, A::Caption : PartialEq, A::Citation : PartialEq, A::Table : PartialEq, A::TableHead : PartialEq, A::TableBody : PartialEq, A::TableFoot : PartialEq, A::Row : PartialEq, A:: Cell : PartialEq"
    )
)]
pub enum InlinesContent<A>
where
    A: DecorationScheme,
{
    Condensed(Vector<Inline<A, CondensedLeaf>>),
    Expanded(Vector<Inline<A, ExpandedLeaf>>),
}

#[derive(Educe)]
#[educe(
    Debug(
        bound = "A::Blocks : Debug, A::Block : Debug, A::Inlines : Debug, A::Inline : Debug, A::Caption : Debug, A::Citation : Debug, A::Table : Debug, A::TableHead : Debug, A::TableBody : Debug, A::TableFoot : Debug, A::Row : Debug, A:: Cell : Debug, L: Debug"
    ),
    Clone(bound = "L: Clone"),
    PartialEq(
        bound = "A::Blocks : PartialEq, A::Block : PartialEq, A::Inlines : PartialEq, A::Inline : PartialEq, A::Caption : PartialEq, A::Citation : PartialEq, A::Table : PartialEq, A::TableHead : PartialEq, A::TableBody : PartialEq, A::TableFoot : PartialEq, A::Row : PartialEq, A:: Cell : PartialEq, L: PartialEq"
    )
)]
pub struct Inline<A, L>
where
    A: DecorationScheme,
{
    pub decoration: Option<A::Inline>,
    pub content: InlineContent<A, L>,
}

/// An inline element.
///
/// The type parameter `L` should be either [`CondensedLeaf`] or [`ExpandedLeaf`].
#[derive(Educe)]
#[educe(
    Debug(
        bound = "A::Blocks : Debug, A::Block : Debug, A::Inlines : Debug, A::Inline : Debug, A::Caption : Debug, A::Citation : Debug, A::Table : Debug, A::TableHead : Debug, A::TableBody : Debug, A::TableFoot : Debug, A::Row : Debug, A:: Cell : Debug, L: Debug"
    ),
    Clone(bound = "L: Clone"),
    PartialEq(
        bound = "A::Blocks : PartialEq, A::Block : PartialEq, A::Inlines : PartialEq, A::Inline : PartialEq, A::Caption : PartialEq, A::Citation : PartialEq, A::Table : PartialEq, A::TableHead : PartialEq, A::TableBody : PartialEq, A::TableFoot : PartialEq, A::Row : PartialEq, A:: Cell : PartialEq, L: PartialEq"
    )
)]
pub enum InlineContent<A, L>
where
    A: DecorationScheme,
{
    /// A leaf node.
    Leaf(L),
    // Emphasized text.
    Emph(Inlines<A>),
    /// Underlined text.
    Underline(Inlines<A>),
    /// Strongly emphasized text.
    Strong(Inlines<A>),
    /// Struck-out text.
    Strikeout(Inlines<A>),
    /// Superscripted text.
    Superscript(Inlines<A>),
    /// Subscripted text.
    Subscript(Inlines<A>),
    /// Small caps text.
    SmallCaps(Inlines<A>),
    /// Quoted text.
    Quoted(QuoteType, Inlines<A>),
    /// Citation.
    Cite(Vector<Citation<A>>, Inlines<A>),
    /// Inline code.
    Code(Attr, Rope),
    /// TeX math.
    Math(MathType, Rope),
    /// Raw inline.
    RawInline(InternedString, Rope),
    /// Hyperlink.
    Link(Attr, Inlines<A>, Target),
    /// Image (with alt text).
    Image(Attr, Inlines<A>, Target),
    /// Footnote or endnote.
    Note(Blocks<A>),
    /// Generic inline container with attributes.
    Span(Attr, Inlines<A>),
}

#[derive(Educe, Serialize, Deserialize)]
#[educe(
    Debug(
        bound = "A::Blocks : Debug, A::Block : Debug, A::Inlines : Debug, A::Inline : Debug, A::Caption : Debug, A::Citation : Debug, A::Table : Debug, A::TableHead : Debug, A::TableBody : Debug, A::TableFoot : Debug, A::Row : Debug, A:: Cell : Debug"
    ),
    Clone,
    PartialEq(
        bound = "A::Blocks : PartialEq, A::Block : PartialEq, A::Inlines : PartialEq, A::Inline : PartialEq, A::Caption : PartialEq, A::Citation : PartialEq, A::Table : PartialEq, A::TableHead : PartialEq, A::TableBody : PartialEq, A::TableFoot : PartialEq, A::Row : PartialEq, A:: Cell : PartialEq"
    )
)]
#[serde(
    from = "serde::CaptionSerde<A>",
    into = "serde::CaptionSerde<A>",
    bound = ""
)]
pub struct Caption<A>
where
    A: DecorationScheme,
{
    pub decoration: Option<A::Caption>,
    pub short: Option<Inlines<A>>,
    pub full: Blocks<A>,
}

#[derive(Educe, Serialize, Deserialize)]
#[educe(
    Debug(
        bound = "A::Blocks : Debug, A::Block : Debug, A::Inlines : Debug, A::Inline : Debug, A::Caption : Debug, A::Citation : Debug, A::Table : Debug, A::TableHead : Debug, A::TableBody : Debug, A::TableFoot : Debug, A::Row : Debug, A:: Cell : Debug"
    ),
    Clone,
    PartialEq(
        bound = "A::Blocks : PartialEq, A::Block : PartialEq, A::Inlines : PartialEq, A::Inline : PartialEq, A::Caption : PartialEq, A::Citation : PartialEq, A::Table : PartialEq, A::TableHead : PartialEq, A::TableBody : PartialEq, A::TableFoot : PartialEq, A::Row : PartialEq, A:: Cell : PartialEq"
    )
)]
#[serde(
    from = "serde::TableSerde<A>",
    into = "serde::TableSerde<A>",
    bound = ""
)]
pub struct Table<A>
where
    A: DecorationScheme,
{
    pub decoration: Option<A::Table>,
    pub attrs: Attr,
    pub caption: Caption<A>,
    pub colspecs: Vector<ColSpec>,
    pub head: TableHead<A>,
    pub body: Vector<TableBody<A>>,
    pub foot: TableFoot<A>,
}

#[derive(Educe, Serialize, Deserialize)]
#[educe(
    Debug(
        bound = "A::Blocks : Debug, A::Block : Debug, A::Inlines : Debug, A::Inline : Debug, A::Caption : Debug, A::Citation : Debug, A::Table : Debug, A::TableHead : Debug, A::TableBody : Debug, A::TableFoot : Debug, A::Row : Debug, A:: Cell : Debug"
    ),
    Clone,
    PartialEq(
        bound = "A::Blocks : PartialEq, A::Block : PartialEq, A::Inlines : PartialEq, A::Inline : PartialEq, A::Caption : PartialEq, A::Citation : PartialEq, A::Table : PartialEq, A::TableHead : PartialEq, A::TableBody : PartialEq, A::TableFoot : PartialEq, A::Row : PartialEq, A:: Cell : PartialEq"
    )
)]
#[serde(
    from = "serde::TableHeadSerde<A>",
    into = "serde::TableHeadSerde<A>",
    bound = ""
)]
pub struct TableHead<A>
where
    A: DecorationScheme,
{
    pub decoration: Option<A::TableHead>,
    pub attrs: Attr,
    pub rows: Vector<Row<A>>,
}

#[derive(Educe, Serialize, Deserialize)]
#[educe(
    Debug(
        bound = "A::Blocks : Debug, A::Block : Debug, A::Inlines : Debug, A::Inline : Debug, A::Caption : Debug, A::Citation : Debug, A::Table : Debug, A::TableHead : Debug, A::TableBody : Debug, A::TableFoot : Debug, A::Row : Debug, A:: Cell : Debug"
    ),
    Clone,
    PartialEq(
        bound = "A::Blocks : PartialEq, A::Block : PartialEq, A::Inlines : PartialEq, A::Inline : PartialEq, A::Caption : PartialEq, A::Citation : PartialEq, A::Table : PartialEq, A::TableHead : PartialEq, A::TableBody : PartialEq, A::TableFoot : PartialEq, A::Row : PartialEq, A:: Cell : PartialEq"
    )
)]
#[serde(
    from = "serde::TableBodySerde<A>",
    into = "serde::TableBodySerde<A>",
    bound = ""
)]
pub struct TableBody<A>
where
    A: DecorationScheme,
{
    pub decoration: Option<A::TableBody>,
    pub attrs: Attr,
    pub row_head_cols: i32,
    pub intermediate_head: Vector<Row<A>>,
    pub rows: Vector<Row<A>>,
}

#[derive(Educe, Serialize, Deserialize)]
#[educe(
    Debug(
        bound = "A::Blocks : Debug, A::Block : Debug, A::Inlines : Debug, A::Inline : Debug, A::Caption : Debug, A::Citation : Debug, A::Table : Debug, A::TableHead : Debug, A::TableBody : Debug, A::TableFoot : Debug, A::Row : Debug, A:: Cell : Debug"
    ),
    Clone,
    PartialEq(
        bound = "A::Blocks : PartialEq, A::Block : PartialEq, A::Inlines : PartialEq, A::Inline : PartialEq, A::Caption : PartialEq, A::Citation : PartialEq, A::Table : PartialEq, A::TableHead : PartialEq, A::TableBody : PartialEq, A::TableFoot : PartialEq, A::Row : PartialEq, A:: Cell : PartialEq"
    )
)]
#[serde(
    from = "serde::TableFootSerde<A>",
    into = "serde::TableFootSerde<A>",
    bound = ""
)]
pub struct TableFoot<A>
where
    A: DecorationScheme,
{
    pub decoration: Option<A::TableFoot>,
    pub attrs: Attr,
    pub rows: Vector<Row<A>>,
}

#[derive(Educe, Serialize, Deserialize)]
#[educe(
    Debug(
        bound = "A::Blocks : Debug, A::Block : Debug, A::Inlines : Debug, A::Inline : Debug, A::Caption : Debug, A::Citation : Debug, A::Table : Debug, A::TableHead : Debug, A::TableBody : Debug, A::TableFoot : Debug, A::Row : Debug, A:: Cell : Debug"
    ),
    Clone,
    PartialEq(
        bound = "A::Blocks : PartialEq, A::Block : PartialEq, A::Inlines : PartialEq, A::Inline : PartialEq, A::Caption : PartialEq, A::Citation : PartialEq, A::Table : PartialEq, A::TableHead : PartialEq, A::TableBody : PartialEq, A::TableFoot : PartialEq, A::Row : PartialEq, A:: Cell : PartialEq"
    )
)]
#[serde(from = "serde::RowSerde<A>", into = "serde::RowSerde<A>", bound = "")]
pub struct Row<A>
where
    A: DecorationScheme,
{
    pub decoration: Option<A::Row>,
    pub attrs: Attr,
    pub cells: Vector<Cell<A>>,
}

#[derive(Educe, Serialize, Deserialize)]
#[educe(
    Debug(
        bound = "A::Blocks : Debug, A::Block : Debug, A::Inlines : Debug, A::Inline : Debug, A::Caption : Debug, A::Citation : Debug, A::Table : Debug, A::TableHead : Debug, A::TableBody : Debug, A::TableFoot : Debug, A::Row : Debug, A:: Cell : Debug"
    ),
    Clone,
    PartialEq(
        bound = "A::Blocks : PartialEq, A::Block : PartialEq, A::Inlines : PartialEq, A::Inline : PartialEq, A::Caption : PartialEq, A::Citation : PartialEq, A::Table : PartialEq, A::TableHead : PartialEq, A::TableBody : PartialEq, A::TableFoot : PartialEq, A::Row : PartialEq, A:: Cell : PartialEq"
    )
)]
#[serde(from = "serde::CellSerde<A>", into = "serde::CellSerde<A>", bound = "")]
pub struct Cell<A>
where
    A: DecorationScheme,
{
    pub decoration: Option<A::Cell>,
    pub attrs: Attr,
    pub alignment: Alignment,
    pub row_span: i32,
    pub col_span: i32,
    pub blocks: Blocks<A>,
}

#[derive(Educe, Serialize, Deserialize)]
#[educe(
    Debug(
        bound = "A::Blocks : Debug, A::Block : Debug, A::Inlines : Debug, A::Inline : Debug, A::Caption : Debug, A::Citation : Debug, A::Table : Debug, A::TableHead : Debug, A::TableBody : Debug, A::TableFoot : Debug, A::Row : Debug, A:: Cell : Debug"
    ),
    Clone,
    PartialEq(
        bound = "A::Blocks : PartialEq, A::Block : PartialEq, A::Inlines : PartialEq, A::Inline : PartialEq, A::Caption : PartialEq, A::Citation : PartialEq, A::Table : PartialEq, A::TableHead : PartialEq, A::TableBody : PartialEq, A::TableFoot : PartialEq, A::Row : PartialEq, A:: Cell : PartialEq"
    )
)]
#[serde(bound = "")]
pub struct Citation<A>
where
    A: DecorationScheme,
{
    #[serde(skip)]
    pub decoration: Option<A::Citation>,
    #[serde(rename = "citationId")]
    pub id: InternedString,
    #[serde(rename = "citationPrefix")]
    pub prefix: Inlines<A>,
    #[serde(rename = "citationSuffix")]
    pub suffix: Inlines<A>,
    #[serde(rename = "citationMode")]
    pub mode: CitationMode,
    #[serde(rename = "citationNoteNum")]
    pub num: i32,
    #[serde(rename = "citationHash")]
    pub hash: i32,
}

/// A condensed representation of a run of text.
///
/// This representation uses a single [`Rope`] to represent all four variants of an [`ExpandedLeaf`].
/// It is encoded as follows:
///
/// * `Space`, `SoftBreak`, and `LineBreak` are encoded as `' '`, `'\r'`, and `'\n'` respectively.
/// * Literal `' '`, `'\r'`, and `'\n'` characters within `Str`s are encoded as [`CONDENSED_LITERAL_SPACE`],
///   [`CONDENSED_LITERAL_CR`], and [`CONDENSED_LITERAL_NEWLINE`] respectively. Each of these constants is a
///   Unicode [noncharacter](https://www.unicode.org/faq/private_use.html#noncharacters).
/// * If any of these three noncharacters occurs within a `Str` (which it never should), it is replaced
///   by [`char::REPLACEMENT_CHARACTER`].
/// * All other characters are mapped to themselves.
pub type CondensedLeaf = Rope;

/// The Pandoc-native representation of a run of text.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ExpandedLeaf {
    Str(Rope),
    Space,
    SoftBreak,
    LineBreak,
}

/// Attributes.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(from = "serde::AttrSerde", into = "serde::AttrSerde")]
pub struct Attr {
    pub identifier: InternedString,
    pub classes: Vector<InternedString>,
    pub attrs: Vector<(InternedString, Rope)>,
}

/// List attributes.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(
    from = "serde::ListAttributesSerde",
    into = "serde::ListAttributesSerde"
)]
pub struct ListAttributes {
    /// The number assigned tot the first element of the list.
    pub start_number: i32,
    /// How list numbers are styled.
    pub number_style: ListNumberStyle,
    /// How list numbers are delimited.
    pub number_delim: ListNumberDelim,
}

/// Style of list numbers.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum ListNumberStyle {
    #[serde(rename = "DefaultStyle")]
    /// Default styling
    Default,
    /// Bullet
    Example,
    /// Decimal number
    Decimal,
    /// Lowercase Roman numerals
    LowerRoman,
    /// Uppercase Roman numerals
    UpperRoman,
    /// Lowercase alphabetic
    LowerAlpha,
    /// Uppercase alphabetic
    UpperAlpha,
}

/// Delimiting style for list numbers.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum ListNumberDelim {
    /// Default delimiter.
    #[serde(rename = "DefaultDelim")]
    Default,
    /// Period ater number.
    Period,
    /// Paren after number.
    OneParen,
    /// Parens surrounding number.
    TwoParens,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(from = "serde::ColSpecSerde", into = "serde::ColSpecSerde")]
pub struct ColSpec {
    pub alignment: Alignment,
    pub col_width: ColWidth,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum ColWidth {
    #[serde(rename = "ColWidth")]
    Percent(f64),
    #[serde(rename = "ColWidthDefault")]
    Default,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum Alignment {
    #[serde(rename = "AlignLeft")]
    Left,
    #[serde(rename = "AlignRight")]
    Right,
    #[serde(rename = "AlignCenter")]
    Center,
    #[serde(rename = "AlignDefault")]
    Default,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum QuoteType {
    #[serde(rename = "SingleQuote")]
    Single,
    #[serde(rename = "DoubleQuote")]
    Double,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(from = "serde::TargetSerde", into = "serde::TargetSerde")]
pub struct Target {
    pub url: Rope,
    pub title: Rope,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum MathType {
    #[serde(rename = "DisplayMath")]
    Display,
    #[serde(rename = "InlineMath")]
    Inline,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum CitationMode {
    AuthorInText,
    SuppressAuthor,
    #[serde(rename = "NormalCitation")]
    Normal,
}

enum Condenser<A>
where
    A: DecorationScheme,
{
    Expanded(CondenserWorker<A, imbl::vector::ConsumingIter<Inline<A, ExpandedLeaf>>>),
    Condensed(imbl::vector::ConsumingIter<Inline<A, CondensedLeaf>>),
}

struct CondenserWorker<A, I>
where
    A: DecorationScheme,
{
    inner: I,
    residual: Option<Inline<A, ExpandedLeaf>>,
}

enum Expander<A>
where
    A: DecorationScheme,
{
    Expanded(imbl::vector::ConsumingIter<Inline<A, ExpandedLeaf>>),
    Condensed(ExpanderWorker<A, imbl::vector::ConsumingIter<Inline<A, CondensedLeaf>>>),
}

struct ExpanderWorker<A, I>
where
    A: DecorationScheme,
{
    inner: I,
    residual: Option<Inline<A, CondensedLeaf>>,
}

impl<A> DecorationScheme for SimpleScheme<A>
where
    A: Clone,
{
    type Pandoc = A;
    type MetaValue = A;
    type Blocks = A;
    type Block = A;
    type Inlines = A;
    type Inline = A;
    type Caption = A;
    type Citation = A;
    type Table = A;
    type TableHead = A;
    type TableBody = A;
    type TableFoot = A;
    type Row = A;
    type Cell = A;
}

impl<A> Pandoc<A>
where
    A: DecorationScheme,
{
    /// Deserialize without automatically condensing inlines.
    pub fn deserialize_noautocondense<'de, D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        no_autocondense(|| Pandoc::deserialize(deserializer))
    }
}

impl<A> Inlines<A>
where
    A: DecorationScheme,
{
    pub fn condensed(&self) -> impl Iterator<Item = Inline<A, CondensedLeaf>> {
        match &self.content {
            InlinesContent::Condensed(i) => Condenser::Condensed(i.clone().into_iter()),
            InlinesContent::Expanded(i) => {
                Condenser::Expanded(CondenserWorker::new(i.clone().into_iter()))
            }
        }
    }

    pub fn into_condensed(self) -> impl Iterator<Item = Inline<A, CondensedLeaf>> {
        match self.content {
            InlinesContent::Condensed(i) => Condenser::Condensed(i.into_iter()),
            InlinesContent::Expanded(i) => Condenser::Expanded(CondenserWorker::new(i.into_iter())),
        }
    }

    pub fn expanded(&self) -> impl Iterator<Item = Inline<A, ExpandedLeaf>> {
        match &self.content {
            InlinesContent::Condensed(i) => {
                Expander::Condensed(ExpanderWorker::new(i.clone().into_iter()))
            }
            InlinesContent::Expanded(i) => Expander::Expanded(i.clone().into_iter()),
        }
    }

    pub fn into_expanded(self) -> impl Iterator<Item = Inline<A, ExpandedLeaf>> {
        match self.content {
            InlinesContent::Condensed(i) => Expander::Condensed(ExpanderWorker::new(i.into_iter())),
            InlinesContent::Expanded(i) => Expander::Expanded(i.into_iter()),
        }
    }

    pub fn condense(&mut self) -> &mut Vector<Inline<A, CondensedLeaf>> {
        if let InlinesContent::Condensed(ref mut i) = self.content {
            return i;
        }

        let mut other = InlinesContent::Condensed(Vector::new());
        std::mem::swap(&mut self.content, &mut other);

        let input = match other {
            InlinesContent::Condensed(_) => unreachable!(),
            InlinesContent::Expanded(i) => i,
        };

        let output = match self.content {
            InlinesContent::Condensed(ref mut i) => i,
            InlinesContent::Expanded(_) => unreachable!(),
        };

        let condenser = CondenserWorker::new(input.into_iter());

        for i in condenser {
            output.push_back(i);
        }

        output
    }

    pub fn expand(&mut self) -> &mut Vector<Inline<A, ExpandedLeaf>> {
        if let InlinesContent::Expanded(ref mut i) = self.content {
            return i;
        }

        let mut other = InlinesContent::Expanded(Vector::new());
        std::mem::swap(&mut self.content, &mut other);

        let input = match other {
            InlinesContent::Condensed(i) => i,
            InlinesContent::Expanded(_) => unreachable!(),
        };

        let output = match self.content {
            InlinesContent::Condensed(_) => unreachable!(),
            InlinesContent::Expanded(ref mut i) => i,
        };

        let expander = ExpanderWorker::new(input.into_iter());

        for i in expander {
            output.push_back(i);
        }

        output
    }
}

impl<A> FromIterator<Inline<A, ExpandedLeaf>> for InlinesContent<A>
where
    A: DecorationScheme,
{
    fn from_iter<T: IntoIterator<Item = Inline<A, ExpandedLeaf>>>(iter: T) -> Self {
        InlinesContent::Expanded(Vector::from_iter(iter))
    }
}

impl<A> FromIterator<Inline<A, CondensedLeaf>> for InlinesContent<A>
where
    A: DecorationScheme,
{
    fn from_iter<T: IntoIterator<Item = Inline<A, CondensedLeaf>>>(iter: T) -> Self {
        InlinesContent::Condensed(Vector::from_iter(iter))
    }
}

impl<A> From<Vector<Inline<A, ExpandedLeaf>>> for InlinesContent<A>
where
    A: DecorationScheme,
{
    fn from(value: Vector<Inline<A, ExpandedLeaf>>) -> Self {
        InlinesContent::Expanded(value)
    }
}

impl<A> From<Vector<Inline<A, CondensedLeaf>>> for InlinesContent<A>
where
    A: DecorationScheme,
{
    fn from(value: Vector<Inline<A, CondensedLeaf>>) -> Self {
        InlinesContent::Condensed(value)
    }
}

impl<A, L> Inline<A, L>
where
    A: DecorationScheme,
{
    fn trivial_cast<M>(self) -> Inline<A, M> {
        Inline {
            decoration: self.decoration,
            content: match self.content {
                InlineContent::Leaf(_) => unreachable!("Called trivial_cast on a leaf inline"),
                InlineContent::Emph(i) => InlineContent::Emph(i),
                InlineContent::Underline(i) => InlineContent::Underline(i),
                InlineContent::Strong(i) => InlineContent::Strong(i),
                InlineContent::Strikeout(i) => InlineContent::Strikeout(i),
                InlineContent::Superscript(i) => InlineContent::Superscript(i),
                InlineContent::Subscript(i) => InlineContent::Subscript(i),
                InlineContent::SmallCaps(i) => InlineContent::SmallCaps(i),
                InlineContent::Quoted(q, i) => InlineContent::Quoted(q, i),
                InlineContent::Cite(c, i) => InlineContent::Cite(c, i),
                InlineContent::Code(a, r) => InlineContent::Code(a, r),
                InlineContent::Math(m, r) => InlineContent::Math(m, r),
                InlineContent::RawInline(f, r) => InlineContent::RawInline(f, r),
                InlineContent::Link(a, i, t) => InlineContent::Link(a, i, t),
                InlineContent::Image(a, i, t) => InlineContent::Image(a, i, t),
                InlineContent::Note(b) => InlineContent::Note(b),
                InlineContent::Span(a, i) => InlineContent::Span(a, i),
            },
        }
    }
}

impl<A, I> CondenserWorker<A, I>
where
    A: DecorationScheme,
{
    fn new(inner: I) -> CondenserWorker<A, I> {
        CondenserWorker {
            inner,
            residual: None,
        }
    }
}

impl<A, I> Iterator for CondenserWorker<A, I>
where
    A: DecorationScheme,
    I: Iterator<Item = Inline<A, ExpandedLeaf>>,
{
    type Item = Inline<A, CondensedLeaf>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut output: CondensedLeaf = Rope::new();

        loop {
            if let Some(Inline {
                decoration,
                content,
            }) = self.residual.take().or_else(|| self.inner.next())
            {
                match content {
                    InlineContent::Leaf(ExpandedLeaf::Str(s)) => {
                        for ch in s.chars() {
                            match ch {
                                ' ' => {
                                    output.push_back(CONDENSED_LITERAL_SPACE);
                                }
                                '\r' => {
                                    output.push_back(CONDENSED_LITERAL_CR);
                                }
                                '\n' => {
                                    output.push_back(CONDENSED_LITERAL_NEWLINE);
                                }
                                CONDENSED_LITERAL_SPACE
                                | CONDENSED_LITERAL_CR
                                | CONDENSED_LITERAL_NEWLINE => {
                                    output.push_back(char::REPLACEMENT_CHARACTER);
                                }
                                _ => {
                                    output.push_back(ch);
                                }
                            }
                        }
                    }
                    InlineContent::Leaf(ExpandedLeaf::Space) => {
                        output.push_back(' ');
                    }
                    InlineContent::Leaf(ExpandedLeaf::SoftBreak) => {
                        output.push_back('\r');
                    }
                    InlineContent::Leaf(ExpandedLeaf::LineBreak) => {
                        output.push_back('\n');
                    }
                    _ if !output.is_empty() => {
                        self.residual = Some(Inline {
                            decoration,
                            content,
                        });
                        return Some(Inline {
                            decoration: None,
                            content: InlineContent::Leaf(output),
                        });
                    }

                    _ => {
                        return Some(
                            Inline {
                                decoration,
                                content,
                            }
                            .trivial_cast(),
                        );
                    }
                }
            } else if !output.is_empty() {
                return Some(Inline {
                    decoration: None,
                    content: InlineContent::Leaf(output),
                });
            } else {
                return None;
            }
        }
    }
}

impl<A> Iterator for Condenser<A>
where
    A: DecorationScheme,
{
    type Item = Inline<A, CondensedLeaf>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Expanded(i) => i.next(),
            Self::Condensed(i) => i.next(),
        }
    }
}

impl<A, I> ExpanderWorker<A, I>
where
    A: DecorationScheme,
{
    fn new(inner: I) -> ExpanderWorker<A, I> {
        ExpanderWorker {
            inner,
            residual: None,
        }
    }
}

impl<A, I> Iterator for ExpanderWorker<A, I>
where
    A: DecorationScheme,
    I: Iterator<Item = Inline<A, CondensedLeaf>>,
{
    type Item = Inline<A, ExpandedLeaf>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut output = Rope::new();

        loop {
            if let Some(Inline {
                decoration,
                content,
            }) = self.residual.take().or_else(|| self.inner.next())
            {
                match content {
                    InlineContent::Leaf(mut r) => {
                        while let Some(ch) = r.pop_front() {
                            match ch {
                                ' ' | '\r' | '\n' if !output.is_empty() => {
                                    r.push_front(ch);
                                    self.residual = Some(Inline {
                                        decoration: None,
                                        content: InlineContent::Leaf(r),
                                    });
                                    return Some(Inline {
                                        decoration: None,
                                        content: InlineContent::Leaf(ExpandedLeaf::Str(output)),
                                    });
                                }
                                ' ' => {
                                    self.residual = Some(Inline {
                                        decoration: None,
                                        content: InlineContent::Leaf(r),
                                    });
                                    return Some(Inline {
                                        decoration: None,
                                        content: InlineContent::Leaf(ExpandedLeaf::Space),
                                    });
                                }
                                '\r' => {
                                    self.residual = Some(Inline {
                                        decoration: None,
                                        content: InlineContent::Leaf(r),
                                    });
                                    return Some(Inline {
                                        decoration: None,
                                        content: InlineContent::Leaf(ExpandedLeaf::SoftBreak),
                                    });
                                }
                                '\n' => {
                                    self.residual = Some(Inline {
                                        decoration: None,
                                        content: InlineContent::Leaf(r),
                                    });
                                    return Some(Inline {
                                        decoration: None,
                                        content: InlineContent::Leaf(ExpandedLeaf::LineBreak),
                                    });
                                }
                                CONDENSED_LITERAL_SPACE => {
                                    output.push_back(' ');
                                }
                                CONDENSED_LITERAL_CR => {
                                    output.push_back('\r');
                                }
                                CONDENSED_LITERAL_NEWLINE => {
                                    output.push_back('\n');
                                }
                                _ => {
                                    output.push_back(ch);
                                }
                            }
                        }
                    }
                    _ if !output.is_empty() => {
                        self.residual = Some(Inline {
                            decoration,
                            content,
                        });
                        return Some(Inline {
                            decoration: None,
                            content: InlineContent::Leaf(ExpandedLeaf::Str(output)),
                        });
                    }
                    _ => {
                        return Some(
                            Inline {
                                decoration,
                                content,
                            }
                            .trivial_cast(),
                        );
                    }
                }
            } else if !output.is_empty() {
                return Some(Inline {
                    decoration: None,
                    content: InlineContent::Leaf(ExpandedLeaf::Str(output)),
                });
            } else {
                return None;
            }
        }
    }
}

impl<A> Iterator for Expander<A>
where
    A: DecorationScheme,
{
    type Item = Inline<A, ExpandedLeaf>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Expanded(i) => i.next(),
            Self::Condensed(i) => i.next(),
        }
    }
}

thread_local! {
    static AUTOCONDENSE: StdCell<bool> = const { StdCell::new(true) };
}

/// Suppress automatic condensing of inlines when deserializing.
fn no_autocondense<F, R>(block: F) -> R
where
    F: FnOnce() -> R,
{
    AUTOCONDENSE.with(|cell| {
        let old = cell.get();
        cell.set(false);
        let result = std::panic::catch_unwind(AssertUnwindSafe(block));
        cell.set(old);

        match result {
            Ok(x) => x,
            Err(e) => std::panic::resume_unwind(e),
        }
    })
}

macro_rules! test_roundtrip {
    ($name:ident, $file:literal) => {
        #[test]
        fn $name() {
            let input = ::std::include_str!($file);
            let input_value: ::serde_json::Value = ::serde_json::from_str(input).unwrap();
            let ast: $crate::ast::Pandoc<NullScheme> = ::serde_json::from_str(input).unwrap();
            let output_value = ::serde_json::to_value(&ast).unwrap();
            ::std::assert_eq!(output_value, input_value);
        }
    };
}

test_roundtrip!(test_roundtrip_testsuite, "../../testcases/testsuite.json");
test_roundtrip!(test_roundtrip_tables, "../../testcases/tables.json");
test_roundtrip!(
    test_roundtrip_markdown_citations,
    "../../testcases/markdown-citations.json"
);
test_roundtrip!(
    test_roundtrip_markdown_reader_more,
    "../../testcases/markdown-reader-more.json"
);
test_roundtrip!(test_pipe_tables, "../../testcases/pipe-tables.json");
