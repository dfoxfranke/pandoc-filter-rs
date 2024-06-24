use super::*;

#[derive(Serialize, Deserialize)]
#[serde(transparent, bound = "")]
pub(super) struct MetaValueSerde<A: DecorationScheme>(MetaValueContent<A>);

#[derive(Serialize, Deserialize)]
#[serde(transparent, bound = "")]
pub(super) struct BlocksSerde<A: DecorationScheme>(Vector<Block<A>>);

#[derive(Serialize, Deserialize)]
#[serde(transparent, bound = "")]
pub(super) struct BlockSerde<A: DecorationScheme>(BlockContent<A>);

#[derive(Serialize, Deserialize)]
#[serde(tag = "t", content = "c", bound = "")]
pub(super) enum InlineSerde<A>
where
    A: DecorationScheme,
{
    Str(Rope),
    Space,
    SoftBreak,
    LineBreak,
    Emph(Inlines<A>),
    Underline(Inlines<A>),
    Strong(Inlines<A>),
    Strikeout(Inlines<A>),
    Superscript(Inlines<A>),
    Subscript(Inlines<A>),
    SmallCaps(Inlines<A>),
    Quoted(QuoteType, Inlines<A>),
    Cite(Vector<Citation<A>>, Inlines<A>),
    Code(Attr, Rope),
    Math(MathType, Rope),
    RawInline(InternedString, Rope),
    Link(Attr, Inlines<A>, Target),
    Image(Attr, Inlines<A>, Target),
    Note(Blocks<A>),
    Span(Attr, Inlines<A>),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged, bound = "")]
pub(super) enum CaptionSerde<A>
where
    A: DecorationScheme,
{
    Caption(Option<Inlines<A>>, Blocks<A>),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub(super) enum ColSpecSerde {
    ColSpec(Alignment, ColWidth),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged, bound = "")]
pub(super) enum TableHeadSerde<A>
where
    A: DecorationScheme,
{
    TableHead(Attr, Vector<Row<A>>),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged, bound = "")]
pub(super) enum TableBodySerde<A>
where
    A: DecorationScheme,
{
    TableBody(Attr, i32, Vector<Row<A>>, Vector<Row<A>>),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged, bound = "")]
pub(super) enum TableFootSerde<A>
where
    A: DecorationScheme,
{
    TableFoot(Attr, Vector<Row<A>>),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged, bound = "")]
pub(super) enum RowSerde<A>
where
    A: DecorationScheme,
{
    Row(Attr, Vector<Cell<A>>),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged, bound = "")]
pub(super) enum CellSerde<A>
where
    A: DecorationScheme,
{
    Cell(Attr, Alignment, i32, i32, Blocks<A>),
}

pub(super) type AttrSerde = (
    InternedString,
    Vector<InternedString>,
    Vector<(InternedString, Rope)>,
);

pub(super) type ListAttributesSerde = (i32, ListNumberStyle, ListNumberDelim);

pub(super) type TableSerde<A> = (
    Attr,
    Caption<A>,
    Vector<ColSpec>,
    TableHead<A>,
    Vector<TableBody<A>>,
    TableFoot<A>,
);

pub(super) type TargetSerde = (Rope, Rope);

impl<A> From<MetaValueSerde<A>> for MetaValue<A>
where
    A: DecorationScheme,
{
    fn from(value: MetaValueSerde<A>) -> Self {
        MetaValue {
            decoration: None,
            content: value.0,
        }
    }
}

impl<A> From<MetaValue<A>> for MetaValueSerde<A>
where
    A: DecorationScheme,
{
    fn from(value: MetaValue<A>) -> Self {
        MetaValueSerde(value.content)
    }
}

impl<A> From<BlocksSerde<A>> for Blocks<A>
where
    A: DecorationScheme,
{
    fn from(value: BlocksSerde<A>) -> Self {
        Blocks {
            decoration: None,
            content: value.0,
        }
    }
}

impl<A> From<Blocks<A>> for BlocksSerde<A>
where
    A: DecorationScheme,
{
    fn from(value: Blocks<A>) -> Self {
        BlocksSerde(value.content)
    }
}

impl<A> From<Block<A>> for BlockSerde<A>
where
    A: DecorationScheme,
{
    fn from(value: Block<A>) -> Self {
        BlockSerde(value.content)
    }
}

impl<A> From<BlockSerde<A>> for Block<A>
where
    A: DecorationScheme,
{
    fn from(value: BlockSerde<A>) -> Self {
        Block {
            decoration: None,
            content: value.0,
        }
    }
}

impl<A> From<InlineContent<A, ExpandedLeaf>> for InlineSerde<A>
where
    A: DecorationScheme,
{
    fn from(value: InlineContent<A, ExpandedLeaf>) -> Self {
        match value {
            InlineContent::Leaf(ExpandedLeaf::Str(s)) => Self::Str(s),
            InlineContent::Leaf(ExpandedLeaf::LineBreak) => Self::LineBreak,
            InlineContent::Leaf(ExpandedLeaf::SoftBreak) => Self::SoftBreak,
            InlineContent::Leaf(ExpandedLeaf::Space) => Self::Space,
            InlineContent::Emph(i) => Self::Emph(i),
            InlineContent::Underline(i) => Self::Underline(i),
            InlineContent::Strong(i) => Self::Strong(i),
            InlineContent::Strikeout(i) => Self::Strikeout(i),
            InlineContent::Superscript(i) => Self::Superscript(i),
            InlineContent::Subscript(i) => Self::Subscript(i),
            InlineContent::SmallCaps(i) => Self::SmallCaps(i),
            InlineContent::Quoted(q, i) => Self::Quoted(q, i),
            InlineContent::Cite(c, i) => Self::Cite(c, i),
            InlineContent::Code(a, r) => Self::Code(a, r),
            InlineContent::Math(m, r) => Self::Math(m, r),
            InlineContent::RawInline(f, r) => Self::RawInline(f, r),
            InlineContent::Link(a, i, t) => Self::Link(a, i, t),
            InlineContent::Image(a, i, t) => Self::Image(a, i, t),
            InlineContent::Note(b) => Self::Note(b),
            InlineContent::Span(a, i) => Self::Span(a, i),
        }
    }
}

impl<A> From<InlineSerde<A>> for InlineContent<A, ExpandedLeaf>
where
    A: DecorationScheme,
{
    fn from(value: InlineSerde<A>) -> Self {
        match value {
            InlineSerde::Str(s) => Self::Leaf(ExpandedLeaf::Str(s)),
            InlineSerde::Space => Self::Leaf(ExpandedLeaf::Space),
            InlineSerde::SoftBreak => Self::Leaf(ExpandedLeaf::SoftBreak),
            InlineSerde::LineBreak => Self::Leaf(ExpandedLeaf::LineBreak),
            InlineSerde::Emph(i) => Self::Emph(i),
            InlineSerde::Underline(i) => Self::Underline(i),
            InlineSerde::Strong(i) => Self::Strong(i),
            InlineSerde::Strikeout(i) => Self::Strikeout(i),
            InlineSerde::Superscript(i) => Self::Superscript(i),
            InlineSerde::Subscript(i) => Self::Subscript(i),
            InlineSerde::SmallCaps(i) => Self::SmallCaps(i),
            InlineSerde::Quoted(q, i) => Self::Quoted(q, i),
            InlineSerde::Cite(c, i) => Self::Cite(c, i),
            InlineSerde::Code(a, r) => Self::Code(a, r),
            InlineSerde::Math(m, r) => Self::Math(m, r),
            InlineSerde::RawInline(f, r) => Self::RawInline(f, r),
            InlineSerde::Link(a, i, t) => Self::Link(a, i, t),
            InlineSerde::Image(a, i, t) => Self::Image(a, i, t),
            InlineSerde::Note(b) => Self::Note(b),
            InlineSerde::Span(a, i) => Self::Span(a, i),
        }
    }
}

impl<A> From<Caption<A>> for CaptionSerde<A>
where
    A: DecorationScheme,
{
    fn from(value: Caption<A>) -> Self {
        CaptionSerde::Caption(value.short, value.full)
    }
}

impl<A> From<CaptionSerde<A>> for Caption<A>
where
    A: DecorationScheme,
{
    fn from(value: CaptionSerde<A>) -> Self {
        match value {
            CaptionSerde::Caption(short, full) => Caption {
                decoration: None,
                short,
                full,
            },
        }
    }
}

impl From<ColSpec> for ColSpecSerde {
    fn from(value: ColSpec) -> Self {
        ColSpecSerde::ColSpec(value.alignment, value.col_width)
    }
}

impl From<ColSpecSerde> for ColSpec {
    fn from(value: ColSpecSerde) -> Self {
        match value {
            ColSpecSerde::ColSpec(alignment, col_width) => ColSpec {
                alignment,
                col_width,
            },
        }
    }
}

impl<A> From<TableHeadSerde<A>> for TableHead<A>
where
    A: DecorationScheme,
{
    fn from(value: TableHeadSerde<A>) -> Self {
        match value {
            TableHeadSerde::TableHead(attrs, rows) => TableHead {
                decoration: None,
                attrs,
                rows,
            },
        }
    }
}

impl<A> From<TableHead<A>> for TableHeadSerde<A>
where
    A: DecorationScheme,
{
    fn from(value: TableHead<A>) -> Self {
        TableHeadSerde::TableHead(value.attrs, value.rows)
    }
}

impl<A> From<RowSerde<A>> for Row<A>
where
    A: DecorationScheme,
{
    fn from(value: RowSerde<A>) -> Self {
        match value {
            RowSerde::Row(attrs, cells) => Row {
                decoration: None,
                attrs,
                cells,
            },
        }
    }
}

impl<A> From<Row<A>> for RowSerde<A>
where
    A: DecorationScheme,
{
    fn from(value: Row<A>) -> Self {
        RowSerde::Row(value.attrs, value.cells)
    }
}

impl<A> From<Cell<A>> for CellSerde<A>
where
    A: DecorationScheme,
{
    fn from(value: Cell<A>) -> Self {
        CellSerde::Cell(
            value.attrs,
            value.alignment,
            value.row_span,
            value.col_span,
            value.blocks,
        )
    }
}

impl<A> From<CellSerde<A>> for Cell<A>
where
    A: DecorationScheme,
{
    fn from(value: CellSerde<A>) -> Self {
        match value {
            CellSerde::Cell(attrs, alignment, row_span, col_span, blocks) => Cell {
                decoration: None,
                attrs,
                alignment,
                row_span,
                col_span,
                blocks,
            },
        }
    }
}

impl<A> From<TableBodySerde<A>> for TableBody<A>
where
    A: DecorationScheme,
{
    fn from(value: TableBodySerde<A>) -> Self {
        match value {
            TableBodySerde::TableBody(attrs, row_head_cols, intermediate_head, rows) => TableBody {
                decoration: None,
                attrs,
                row_head_cols,
                intermediate_head,
                rows,
            },
        }
    }
}

impl<A> From<TableBody<A>> for TableBodySerde<A>
where
    A: DecorationScheme,
{
    fn from(value: TableBody<A>) -> Self {
        TableBodySerde::TableBody(
            value.attrs,
            value.row_head_cols,
            value.intermediate_head,
            value.rows,
        )
    }
}

impl<A> From<TableFootSerde<A>> for TableFoot<A>
where
    A: DecorationScheme,
{
    fn from(value: TableFootSerde<A>) -> Self {
        match value {
            TableFootSerde::TableFoot(attrs, rows) => TableFoot {
                decoration: None,
                attrs,
                rows,
            },
        }
    }
}

impl<A> From<TableFoot<A>> for TableFootSerde<A>
where
    A: DecorationScheme,
{
    fn from(value: TableFoot<A>) -> Self {
        TableFootSerde::TableFoot(value.attrs, value.rows)
    }
}

impl From<Attr> for AttrSerde {
    fn from(value: Attr) -> Self {
        (value.identifier, value.classes, value.attrs)
    }
}

impl From<AttrSerde> for Attr {
    fn from(value: AttrSerde) -> Self {
        Attr {
            identifier: value.0,
            classes: value.1,
            attrs: value.2,
        }
    }
}

impl From<ListAttributes> for ListAttributesSerde {
    fn from(value: ListAttributes) -> Self {
        (value.start_number, value.number_style, value.number_delim)
    }
}

impl From<ListAttributesSerde> for ListAttributes {
    fn from(value: ListAttributesSerde) -> Self {
        ListAttributes {
            start_number: value.0,
            number_style: value.1,
            number_delim: value.2,
        }
    }
}

impl<A> From<Table<A>> for TableSerde<A>
where
    A: DecorationScheme,
{
    fn from(value: Table<A>) -> Self {
        (
            value.attrs,
            value.caption,
            value.colspecs,
            value.head,
            value.body,
            value.foot,
        )
    }
}

impl<A> From<TableSerde<A>> for Table<A>
where
    A: DecorationScheme,
{
    fn from(value: TableSerde<A>) -> Self {
        Table {
            decoration: None,
            attrs: value.0,
            caption: value.1,
            colspecs: value.2,
            head: value.3,
            body: value.4,
            foot: value.5,
        }
    }
}

impl From<Target> for TargetSerde {
    fn from(value: Target) -> Self {
        (value.url, value.title)
    }
}

impl From<TargetSerde> for Target {
    fn from(value: TargetSerde) -> Self {
        Target {
            url: value.0,
            title: value.1,
        }
    }
}

impl<A> Serialize for Pandoc<A>
where
    A: DecorationScheme,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        struct SortedMap<'a, K, V>(&'a HashMap<K, V>);

        impl<'a, K, V> Serialize for SortedMap<'a, K, V>
        where
            K: std::hash::Hash + Eq + Ord + Serialize,
            V: Serialize,
        {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let mut entries = Vec::from_iter(self.0.iter());
                entries.sort_by_key(|(k, _)| *k);

                let mut state = serializer.serialize_map(Some(entries.len()))?;
                for (k, v) in entries {
                    state.serialize_entry(k, v)?;
                }
                state.end()
            }
        }

        let mut state = serializer.serialize_struct("Pandoc", 3)?;
        state.serialize_field("pandoc-api-version", API_VERSION)?;
        state.serialize_field("meta", &SortedMap(&self.meta))?;
        state.serialize_field("blocks", &self.blocks)?;
        state.end()
    }
}

impl<'de, A> Deserialize<'de> for Pandoc<A>
where
    A: DecorationScheme,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Debug, Clone)]
        struct ApiVersionError(Vec<u32>);

        impl Display for ApiVersionError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "incompatible Pandoc API version: got ")?;
                for i in 0..self.0.len() - 1 {
                    write!(f, "{}.", self.0[i])?
                }
                write!(f, "{}", self.0[self.0.len() - 1])?;
                write!(f, ", expected {}.{}.*", API_VERSION[0], API_VERSION[1])?;
                Ok(())
            }
        }

        impl Error for ApiVersionError {}

        struct PandocVisitor<A>(PhantomData<A>);

        const FIELDS: &[&str] = &["pandoc-api-version", "meta", "blocks"];

        impl<'de, A> Visitor<'de> for PandocVisitor<A>
        where
            A: DecorationScheme,
        {
            type Value = Pandoc<A>;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "struct Pandoc")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut meta: Option<HashMap<InternedString, MetaValue<A>>> = None;
                let mut blocks: Option<Blocks<A>> = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "pandoc-api-version" => {
                            let version: Vec<u32> = map.next_value()?;
                            if !version.starts_with(&[API_VERSION[0], API_VERSION[1]]) {
                                return Err(::serde::de::Error::custom(ApiVersionError(version)));
                            }
                        }
                        "meta" => {
                            if meta.is_some() {
                                return Err(::serde::de::Error::duplicate_field("meta"));
                            } else {
                                meta = Some(map.next_value()?);
                            }
                        }
                        "blocks" => {
                            if blocks.is_some() {
                                return Err(::serde::de::Error::duplicate_field("blocks"));
                            } else {
                                blocks = Some(map.next_value()?);
                            }
                        }
                        _ => return Err(::serde::de::Error::unknown_field(key, FIELDS)),
                    }
                }

                Ok(Pandoc {
                    decoration: None,
                    meta: meta.ok_or_else(|| ::serde::de::Error::missing_field("meta"))?,
                    blocks: blocks.ok_or_else(|| ::serde::de::Error::missing_field("blocks"))?,
                })
            }
        }

        deserializer.deserialize_struct("Pandoc", FIELDS, PandocVisitor(PhantomData))
    }
}

impl<A> Serialize for Inlines<A>
where
    A: DecorationScheme,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_seq(self.expanded().map(|i| serde::InlineSerde::from(i.content)))
    }
}

impl<'de, A> Deserialize<'de> for Inlines<A>
where
    A: DecorationScheme,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SeqAccessIter<'de, 'e, S, A>
        where
            S: SeqAccess<'de>,
        {
            inner: S,
            phantom: PhantomData<A>,
            error: &'e mut Option<<S as SeqAccess<'de>>::Error>,
        }

        impl<'de, 'e, S, A> Iterator for SeqAccessIter<'de, 'e, S, A>
        where
            S: SeqAccess<'de>,
            A: DecorationScheme,
        {
            type Item = Inline<A, ExpandedLeaf>;

            fn next(&mut self) -> Option<Self::Item> {
                match self.inner.next_element::<serde::InlineSerde<A>>() {
                    Ok(i) => i.map(|x| Inline {
                        decoration: None,
                        content: x.into(),
                    }),
                    Err(e) => {
                        *self.error = Some(e);
                        None
                    }
                }
            }
        }

        struct SeqVisitor<A>(PhantomData<A>);

        impl<'de, A> Visitor<'de> for SeqVisitor<A>
        where
            A: DecorationScheme,
        {
            type Value = Inlines<A>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "a sequence of inlines")
            }

            fn visit_seq<S>(self, seq: S) -> Result<Self::Value, S::Error>
            where
                S: SeqAccess<'de>,
            {
                let mut error: Option<S::Error> = None;

                let access = SeqAccessIter {
                    inner: seq,
                    phantom: PhantomData,
                    error: &mut error,
                };

                let condenser = CondenserWorker::new(access);
                let condensed = Vector::from_iter(condenser);

                match error {
                    Some(e) => Err(e),
                    None => Ok(Inlines {
                        decoration: None,
                        content: InlinesContent::Condensed(condensed),
                    }),
                }
            }
        }

        if AUTOCONDENSE.with(|cell| cell.get()) {
            deserializer.deserialize_seq(SeqVisitor(PhantomData))
        } else {
            let inlines_serde: Vec<serde::InlineSerde<A>> = Deserialize::deserialize(deserializer)?;
            let inlines = Vector::from_iter(inlines_serde.into_iter().map(|x| Inline {
                decoration: None,
                content: x.into(),
            }));
            Ok(Inlines {
                decoration: None,
                content: InlinesContent::Expanded(inlines),
            })
        }
    }
}
