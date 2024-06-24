use crate::ast::*;
use imbl::{HashMap, Vector};

pub trait Map<A>
where
    A: DecorationScheme,
{
    fn enter_pandoc(&mut self, pandoc: &Pandoc<A>) {}
    fn enter_meta_value(&mut self, meta: &MetaValue<A>) {}
    fn enter_blocks(&mut self, blocks: &Blocks<A>) {}
    fn enter_block(&mut self, block: &Block<A>) {}
    fn enter_inlines(&mut self, inlines: &Inlines<A>) {}
    fn enter_inline_condensed(&mut self, inline: &Inline<A, CondensedLeaf>) {}
    fn enter_inline_expanded(&mut self, inline: &Inline<A, ExpandedLeaf>) {}
    fn enter_citation(&mut self, citation: &Citation<A>) {}
    fn enter_table(&mut self, table: &Table<A>) {}
    fn enter_table_head(&mut self, head: &TableHead<A>) {}
    fn enter_table_body(&mut self, body: &TableBody<A>) {}
    fn enter_table_foot(&mut self, foot: &TableFoot<A>) {}
    fn enter_row(&mut self, row: &Row<A>) {}
    fn enter_cell(&mut self, cell: &Cell<A>) {}
    fn enter_caption(&mut self, caption: &Caption<A>) {}

    fn map_pandoc(&mut self, pandoc: Pandoc<A>) -> Pandoc<A> {
        pandoc
    }
    fn map_meta_value(&mut self, meta: MetaValue<A>) -> MetaValue<A> {
        meta
    }
    fn map_blocks(&mut self, blocks: Blocks<A>) -> Blocks<A> {
        blocks
    }
    fn map_block(&mut self, block: Block<A>) -> Block<A> {
        block
    }
    fn map_inlines(&mut self, inlines: Inlines<A>) -> Inlines<A> {
        inlines
    }
    fn map_inline_condensed(&mut self, inline: Inline<A, CondensedLeaf>) -> Inline<A, CondensedLeaf> {
        inline
    }
    fn map_inline_expanded(&mut self, inline: Inline<A, ExpandedLeaf>) -> Inline<A, ExpandedLeaf> {
        inline
    }
    fn map_citation(&mut self, citation: Citation<A>) -> Citation<A> {
        citation
    }
    fn map_table(&mut self, table: Table<A>) -> Table<A> {
        table
    }
    fn map_table_head(&mut self, head: TableHead<A>) -> TableHead<A> {
        head
    }
    fn map_table_body(&mut self, body: TableBody<A>) -> TableBody<A> {
        body
    }
    fn map_table_foot(&mut self, foot: TableFoot<A>) -> TableFoot<A> {
        foot
    }
    fn map_row(&mut self, row: Row<A>) -> Row<A> {
        row
    }
    fn map_cell(&mut self, cell: Cell<A>) -> Cell<A> {
        cell
    }
    fn map_caption(&mut self, caption: Caption<A>) -> Caption<A> {
        caption
    }
}

pub trait DecorationMap<A, B>
where
    A: DecorationScheme,
    B: DecorationScheme,
{
    fn map_pandoc_decoration(&self, input: A::Pandoc) -> B::Pandoc;
    fn map_meta_value_decoration(&self, input: A::MetaValue) -> B::MetaValue;
    fn map_blocks_decoration(&self, input: A::Blocks) -> B::Blocks;
    fn map_block_decoration(&self, input: A::Block) -> B::Block;
    fn map_inlines_decoration(&self, input: A::Inlines) -> B::Inlines;
    fn map_inline_decoration(&self, input: A::Inline) -> B::Inline;
    fn map_citation_decoration(&self, input: A::Citation) -> B::Citation;
    fn map_table_decoration(&self, input: A::Table) -> B::Table;
    fn map_table_head_decoration(&self, input: A::TableHead) -> B::TableHead;
    fn map_table_body_decoration(&self, input: A::TableBody) -> B::TableBody;
    fn map_table_foot_decoration(&self, input: A::TableFoot) -> B::TableFoot;
    fn map_row_decoration(&self, input: A::Row) -> B::Row;
    fn map_cell_decoration(&self, input: A::Cell) -> B::Cell;
    fn map_caption_decoration(&self, input: A::Caption) -> B::Caption;
}

#[allow(unused_variables)]
pub trait Query<A>
where
    A: DecorationScheme,
{
    fn query_pandoc(&mut self, pandoc: &Pandoc<A>) {}
    fn query_meta_value(&mut self, meta: &MetaValue<A>) {}
    fn query_blocks(&mut self, blocks: &Blocks<A>) {}
    fn query_block(&mut self, block: &Block<A>) {}
    fn query_inlines(&mut self, inlines: &Inlines<A>) {}
    fn query_inline_condensed(&mut self, inline: &Inline<A, CondensedLeaf>) {}
    fn query_inline_expanded(&mut self, inline: &Inline<A, ExpandedLeaf>) {}
    fn query_citation(&mut self, citation: &Citation<A>) {}
    fn query_table(&mut self, table: &Table<A>) {}
    fn query_table_head(&mut self, head: &TableHead<A>) {}
    fn query_table_body(&mut self, body: &TableBody<A>) {}
    fn query_table_foot(&mut self, foot: &TableFoot<A>) {}
    fn query_row(&mut self, row: &Row<A>) {}
    fn query_cell(&mut self, cell: &Cell<A>) {}
    fn query_caption(&mut self, caption: &Caption<A>) {}
}

impl<A> Pandoc<A>
where
    A: DecorationScheme,
{
    pub fn walk<M>(self, map: &mut M) -> Self
    where
        M: Map<A>,
    {
        map.map_pandoc(Pandoc {
            decoration: self.decoration,
            meta: HashMap::from_iter(self.meta.into_iter().map(|(k, v)| (k, v.walk(map)))),
            blocks: self.blocks.walk(map),
        })
    }

    pub fn walk_decorations<M, B>(self, map: &M) -> Pandoc<B>
    where
        M: DecorationMap<A, B>,
        B: DecorationScheme,
    {
        Pandoc {
            decoration: self.decoration.map(|d| map.map_pandoc_decoration(d)),
            meta: HashMap::from_iter(
                self.meta
                    .into_iter()
                    .map(|(k, v)| (k, v.walk_decorations(map))),
            ),
            blocks: self.blocks.walk_decorations(map),
        }
    }

    pub fn query<Q>(&self, query: &mut Q)
    where
        Q: Query<A>,
    {
        for v in self.meta.values() {
            v.query(query);
        }

        self.blocks.query(query);
        query.query_pandoc(self);
    }
}

impl<A> MetaValue<A>
where
    A: DecorationScheme,
{
    pub fn walk<M>(self, map: &M) -> Self
    where
        M: Map<A>,
    {
        map.map_meta_value(MetaValue {
            decoration: self.decoration,
            content: match self.content {
                MetaValueContent::Map(m) => MetaValueContent::Map(HashMap::from_iter(
                    m.into_iter().map(|(k, v)| (k, v.walk(map))),
                )),
                MetaValueContent::List(l) => {
                    MetaValueContent::List(Vector::from_iter(l.into_iter().map(|m| m.walk(map))))
                }
                MetaValueContent::Bool(b) => MetaValueContent::Bool(b),
                MetaValueContent::String(s) => MetaValueContent::String(s),
                MetaValueContent::Inlines(i) => MetaValueContent::Inlines(i.walk(map)),
                MetaValueContent::Blocks(b) => MetaValueContent::Blocks(b.walk(map)),
            },
        })
    }

    pub fn walk_decorations<M, B>(self, map: &M) -> MetaValue<B>
    where
        M: DecorationMap<A, B>,
        B: DecorationScheme,
    {
        MetaValue {
            decoration: self.decoration.map(|d| map.map_meta_value_decoration(d)),
            content: match self.content {
                MetaValueContent::Map(m) => MetaValueContent::Map(HashMap::from_iter(
                    m.into_iter().map(|(k, v)| (k, v.walk_decorations(map))),
                )),
                MetaValueContent::List(l) => MetaValueContent::List(Vector::from_iter(
                    l.into_iter().map(|m| m.walk_decorations(map)),
                )),
                MetaValueContent::Bool(b) => MetaValueContent::Bool(b),
                MetaValueContent::String(s) => MetaValueContent::String(s),
                MetaValueContent::Inlines(i) => MetaValueContent::Inlines(i.walk_decorations(map)),
                MetaValueContent::Blocks(b) => MetaValueContent::Blocks(b.walk_decorations(map)),
            },
        }
    }

    pub fn query<Q>(&self, query: &mut Q)
    where
        Q: Query<A>,
    {
        match &self.content {
            MetaValueContent::Map(m) => {
                for v in m.values() {
                    v.query(query);
                }
            }

            MetaValueContent::List(l) => {
                for v in l {
                    v.query(query);
                }
            }
            MetaValueContent::Inlines(i) => i.query(query),
            MetaValueContent::Blocks(b) => b.query(query),
            _ => (),
        };

        query.query_meta_value(self);
    }
}

impl<A> Blocks<A>
where
    A: DecorationScheme,
{
    pub fn walk<M>(self, map: &M) -> Self
    where
        M: Map<A>,
    {
        map.map_blocks(Blocks {
            decoration: self.decoration,
            content: Vector::from_iter(self.content.into_iter().map(|v| v.walk(map))),
        })
    }

    pub fn walk_decorations<M, B>(self, map: &M) -> Blocks<B>
    where
        M: DecorationMap<A, B>,
        B: DecorationScheme,
    {
        Blocks {
            decoration: self.decoration.map(|d| map.map_blocks_decoration(d)),
            content: Vector::from_iter(self.content.into_iter().map(|v| v.walk_decorations(map))),
        }
    }

    pub fn query<Q>(&self, query: &mut Q)
    where
        Q: Query<A>,
    {
        for b in &self.content {
            b.query(query);
        }

        query.query_blocks(self);
    }
}

impl<A> Block<A>
where
    A: DecorationScheme,
{
    pub fn walk<M>(self, map: &M) -> Self
    where
        M: Map<A>,
    {
        map.map_block(Block {
            decoration: self.decoration,
            content: match self.content {
                BlockContent::Plain(i) => BlockContent::Plain(i.walk(map)),
                BlockContent::Para(i) => BlockContent::Para(i.walk(map)),
                BlockContent::LineBlock(is) => {
                    BlockContent::LineBlock(Vector::from_iter(is.into_iter().map(|i| i.walk(map))))
                }
                BlockContent::CodeBlock(a, r) => BlockContent::CodeBlock(a, r),
                BlockContent::RawBlock(f, r) => BlockContent::RawBlock(f, r),
                BlockContent::BlockQuote(bs) => BlockContent::BlockQuote(bs.walk(map)),
                BlockContent::OrderedList(a, bss) => BlockContent::OrderedList(
                    a,
                    Vector::from_iter(bss.into_iter().map(|bs| bs.walk(map))),
                ),
                BlockContent::BulletList(bss) => BlockContent::BulletList(Vector::from_iter(
                    bss.into_iter().map(|bs| bs.walk(map)),
                )),
                BlockContent::DefinitionList(dl) => BlockContent::DefinitionList(
                    Vector::from_iter(dl.into_iter().map(|(is, bss)| {
                        (
                            is.walk(map),
                            Vector::from_iter(bss.into_iter().map(|bs| bs.walk(map))),
                        )
                    })),
                ),
                BlockContent::Header(l, a, is) => BlockContent::Header(l, a, is.walk(map)),
                BlockContent::HorizontalRule => BlockContent::HorizontalRule,
                BlockContent::Table(t) => BlockContent::Table(t.walk(map)),
                BlockContent::Figure(a, c, b) => BlockContent::Figure(a, c.walk(map), b.walk(map)),
                BlockContent::Div(a, bs) => BlockContent::Div(a, bs.walk(map)),
            },
        })
    }

    pub fn walk_decorations<M, B>(self, map: &M) -> Block<B>
    where
        M: DecorationMap<A, B>,
        B: DecorationScheme,
    {
        Block {
            decoration: self.decoration.map(|d| map.map_block_decoration(d)),
            content: match self.content {
                BlockContent::Plain(i) => BlockContent::Plain(i.walk_decorations(map)),
                BlockContent::Para(i) => BlockContent::Para(i.walk_decorations(map)),
                BlockContent::LineBlock(is) => BlockContent::LineBlock(Vector::from_iter(
                    is.into_iter().map(|i| i.walk_decorations(map)),
                )),
                BlockContent::CodeBlock(a, r) => BlockContent::CodeBlock(a, r),
                BlockContent::RawBlock(f, r) => BlockContent::RawBlock(f, r),
                BlockContent::BlockQuote(bs) => BlockContent::BlockQuote(bs.walk_decorations(map)),
                BlockContent::OrderedList(a, bss) => BlockContent::OrderedList(
                    a,
                    Vector::from_iter(bss.into_iter().map(|bs| bs.walk_decorations(map))),
                ),
                BlockContent::BulletList(bss) => BlockContent::BulletList(Vector::from_iter(
                    bss.into_iter().map(|bs| bs.walk_decorations(map)),
                )),
                BlockContent::DefinitionList(dl) => BlockContent::DefinitionList(
                    Vector::from_iter(dl.into_iter().map(|(is, bss)| {
                        (
                            is.walk_decorations(map),
                            Vector::from_iter(bss.into_iter().map(|bs| bs.walk_decorations(map))),
                        )
                    })),
                ),
                BlockContent::Header(l, a, is) => {
                    BlockContent::Header(l, a, is.walk_decorations(map))
                }
                BlockContent::HorizontalRule => BlockContent::HorizontalRule,
                BlockContent::Table(t) => BlockContent::Table(t.walk_decorations(map)),
                BlockContent::Figure(a, c, b) => {
                    BlockContent::Figure(a, c.walk_decorations(map), b.walk_decorations(map))
                }
                BlockContent::Div(a, bs) => BlockContent::Div(a, bs.walk_decorations(map)),
            },
        }
    }

    pub fn query<Q>(&self, query: &mut Q)
    where
        Q: Query<A>,
    {
        match &self.content {
            BlockContent::Plain(i) => {
                i.query(query);
            }
            BlockContent::Para(i) => {
                i.query(query);
            }
            BlockContent::LineBlock(is) => {
                for i in is {
                    i.query(query);
                }
            }
            BlockContent::BlockQuote(bs) => {
                bs.query(query);
            }
            BlockContent::OrderedList(_, bss) => {
                for bs in bss {
                    bs.query(query);
                }
            }
            BlockContent::BulletList(bss) => {
                for bs in bss {
                    bs.query(query);
                }
            }
            BlockContent::DefinitionList(dl) => {
                for (is, bss) in dl {
                    is.query(query);
                    for bs in bss {
                        bs.query(query);
                    }
                }
            }
            BlockContent::Header(_, _, is) => {
                is.query(query);
            }
            BlockContent::Table(t) => {
                t.query(query);
            }
            BlockContent::Figure(_, c, bs) => {
                c.query(query);
                bs.query(query);
            }
            BlockContent::Div(_, bs) => {
                bs.query(query);
            }
            _ => {}
        }
        query.query_block(self);
    }
}

impl<A> Inlines<A>
where
    A: DecorationScheme,
{
    pub fn walk<M>(self, map: &M) -> Self
    where
        M: Map<A>,
    {
        map.map_inlines(Inlines {
            decoration: self.decoration,
            content: match self.content {
                InlinesContent::Condensed(is) => InlinesContent::Condensed(Vector::from_iter(
                    is.into_iter().map(|i| i.walk(map)),
                )),
                InlinesContent::Expanded(is) => {
                    InlinesContent::Expanded(Vector::from_iter(is.into_iter().map(|i| i.walk(map))))
                }
            },
        })
    }

    pub fn walk_decorations<M, B>(self, map: &M) -> Inlines<B>
    where
        M: DecorationMap<A, B>,
        B: DecorationScheme,
    {
        Inlines {
            decoration: self.decoration.map(|d| map.map_inlines_decoration(d)),
            content: match self.content {
                InlinesContent::Condensed(is) => InlinesContent::Condensed(Vector::from_iter(
                    is.into_iter().map(|i| i.walk_decorations(map)),
                )),
                InlinesContent::Expanded(is) => InlinesContent::Expanded(Vector::from_iter(
                    is.into_iter().map(|i| i.walk_decorations(map)),
                )),
            },
        }
    }

    pub fn query<Q>(&self, query: &mut Q)
    where
        Q: Query<A>,
    {
        match &self.content {
            InlinesContent::Condensed(is) => {
                for i in is {
                    i.query(query);
                }
            }
            InlinesContent::Expanded(is) => {
                for i in is {
                    i.query(query);
                }
            }
        }

        query.query_inlines(self);
    }
}

impl<A> Inline<A, CondensedLeaf>
where
    A: DecorationScheme,
{
    pub fn walk<M>(self, map: &M) -> Self
    where
        M: Map<A>,
    {
        map.map_inline_condensed(self.walk_general(map))
    }

    pub fn query<Q>(&self, query: &mut Q)
    where
        Q: Query<A>,
    {
        self.query_general(query);
        query.query_inline_condensed(self);
    }
}

impl<A> Inline<A, ExpandedLeaf>
where
    A: DecorationScheme,
{
    pub fn walk<M>(self, map: &M) -> Self
    where
        M: Map<A>,
    {
        map.map_inline_expanded(self.walk_general(map))
    }

    pub fn query<Q>(&self, query: &mut Q)
    where
        Q: Query<A>,
    {
        self.query_general(query);
        query.query_inline_expanded(self);
    }
}

impl<A, L> Inline<A, L>
where
    A: DecorationScheme,
{
    fn walk_general<M>(self, map: &M) -> Self
    where
        M: Map<A>,
    {
        Inline {
            decoration: self.decoration,
            content: match self.content {
                InlineContent::Leaf(l) => InlineContent::Leaf(l),
                InlineContent::Emph(is) => InlineContent::Emph(is.walk(map)),
                InlineContent::Underline(is) => InlineContent::Underline(is.walk(map)),
                InlineContent::Strong(is) => InlineContent::Strong(is.walk(map)),
                InlineContent::Strikeout(is) => InlineContent::Strikeout(is.walk(map)),
                InlineContent::Superscript(is) => InlineContent::Superscript(is.walk(map)),
                InlineContent::Subscript(is) => InlineContent::Subscript(is.walk(map)),
                InlineContent::SmallCaps(is) => InlineContent::SmallCaps(is.walk(map)),
                InlineContent::Quoted(q, is) => InlineContent::Quoted(q, is.walk(map)),
                InlineContent::Cite(cs, is) => InlineContent::Cite(
                    Vector::from_iter(cs.into_iter().map(|c| c.walk(map))),
                    is.walk(map),
                ),
                InlineContent::Code(a, r) => InlineContent::Code(a, r),
                InlineContent::Math(m, r) => InlineContent::Math(m, r),
                InlineContent::RawInline(f, r) => InlineContent::RawInline(f, r),
                InlineContent::Link(a, is, t) => InlineContent::Link(a, is.walk(map), t),
                InlineContent::Image(a, is, t) => InlineContent::Image(a, is.walk(map), t),
                InlineContent::Note(bs) => InlineContent::Note(bs.walk(map)),
                InlineContent::Span(a, is) => InlineContent::Span(a, is.walk(map)),
            },
        }
    }

    pub fn walk_decorations<M, B>(self, map: &M) -> Inline<B, L>
    where
        M: DecorationMap<A, B>,
        B: DecorationScheme,
    {
        Inline {
            decoration: self.decoration.map(|d| map.map_inline_decoration(d)),
            content: match self.content {
                InlineContent::Leaf(l) => InlineContent::Leaf(l),
                InlineContent::Emph(is) => InlineContent::Emph(is.walk_decorations(map)),
                InlineContent::Underline(is) => InlineContent::Underline(is.walk_decorations(map)),
                InlineContent::Strong(is) => InlineContent::Strong(is.walk_decorations(map)),
                InlineContent::Strikeout(is) => InlineContent::Strikeout(is.walk_decorations(map)),
                InlineContent::Superscript(is) => {
                    InlineContent::Superscript(is.walk_decorations(map))
                }
                InlineContent::Subscript(is) => InlineContent::Subscript(is.walk_decorations(map)),
                InlineContent::SmallCaps(is) => InlineContent::SmallCaps(is.walk_decorations(map)),
                InlineContent::Quoted(q, is) => InlineContent::Quoted(q, is.walk_decorations(map)),
                InlineContent::Cite(cs, is) => InlineContent::Cite(
                    Vector::from_iter(cs.into_iter().map(|c| c.walk_decorations(map))),
                    is.walk_decorations(map),
                ),
                InlineContent::Code(a, r) => InlineContent::Code(a, r),
                InlineContent::Math(m, r) => InlineContent::Math(m, r),
                InlineContent::RawInline(f, r) => InlineContent::RawInline(f, r),
                InlineContent::Link(a, is, t) => {
                    InlineContent::Link(a, is.walk_decorations(map), t)
                }
                InlineContent::Image(a, is, t) => {
                    InlineContent::Image(a, is.walk_decorations(map), t)
                }
                InlineContent::Note(bs) => InlineContent::Note(bs.walk_decorations(map)),
                InlineContent::Span(a, is) => InlineContent::Span(a, is.walk_decorations(map)),
            },
        }
    }

    fn query_general<Q>(&self, query: &mut Q)
    where
        Q: Query<A>,
    {
        match &self.content {
            InlineContent::Emph(is) => {
                is.query(query);
            }
            InlineContent::Underline(is) => {
                is.query(query);
            }
            InlineContent::Strong(is) => {
                is.query(query);
            }
            InlineContent::Strikeout(is) => {
                is.query(query);
            }
            InlineContent::Superscript(is) => {
                is.query(query);
            }
            InlineContent::Subscript(is) => {
                is.query(query);
            }
            InlineContent::SmallCaps(is) => {
                is.query(query);
            }
            InlineContent::Quoted(_, is) => {
                is.query(query);
            }
            InlineContent::Cite(cs, is) => {
                for c in cs {
                    c.query(query);
                }
                is.query(query);
            }
            InlineContent::Link(_, is, _) => {
                is.query(query);
            }
            InlineContent::Image(_, is, _) => {
                is.query(query);
            }
            InlineContent::Note(bs) => {
                bs.query(query);
            }
            InlineContent::Span(_, is) => {
                is.query(query);
            }
            _ => (),
        };
    }
}

impl<A> Citation<A>
where
    A: DecorationScheme,
{
    pub fn walk<M>(self, map: &M) -> Self
    where
        M: Map<A>,
    {
        map.map_citation(Citation {
            decoration: self.decoration,
            id: self.id,
            prefix: self.prefix.walk(map),
            suffix: self.suffix.walk(map),
            mode: self.mode,
            num: self.num,
            hash: self.hash,
        })
    }

    pub fn walk_decorations<M, B>(self, map: &M) -> Citation<B>
    where
        M: DecorationMap<A, B>,
        B: DecorationScheme,
    {
        Citation {
            decoration: self.decoration.map(|d| map.map_citation_decoration(d)),
            id: self.id,
            prefix: self.prefix.walk_decorations(map),
            suffix: self.suffix.walk_decorations(map),
            mode: self.mode,
            num: self.num,
            hash: self.hash,
        }
    }

    pub fn query<Q>(&self, query: &mut Q)
    where
        Q: Query<A>,
    {
        self.prefix.query(query);
        self.suffix.query(query);
        query.query_citation(self);
    }
}

impl<A> Table<A>
where
    A: DecorationScheme,
{
    pub fn walk<M>(self, map: &M) -> Self
    where
        M: Map<A>,
    {
        map.map_table(Table {
            decoration: self.decoration,
            attrs: self.attrs,
            caption: self.caption.walk(map),
            colspecs: self.colspecs,
            head: self.head.walk(map),
            body: Vector::from_iter(self.body.into_iter().map(|b| b.walk(map))),
            foot: self.foot.walk(map),
        })
    }

    pub fn walk_decorations<M, B>(self, map: &M) -> Table<B>
    where
        M: DecorationMap<A, B>,
        B: DecorationScheme,
    {
        Table {
            decoration: self.decoration.map(|d| map.map_table_decoration(d)),
            attrs: self.attrs,
            caption: self.caption.walk_decorations(map),
            colspecs: self.colspecs,
            head: self.head.walk_decorations(map),
            body: Vector::from_iter(self.body.into_iter().map(|b| b.walk_decorations(map))),
            foot: self.foot.walk_decorations(map),
        }
    }

    pub fn query<Q>(&self, query: &mut Q)
    where
        Q: Query<A>,
    {
        self.caption.query(query);
        self.head.query(query);
        for b in &self.body {
            b.query(query);
        }
        self.foot.query(query);
        query.query_table(self);
    }
}

impl<A> TableHead<A>
where
    A: DecorationScheme,
{
    pub fn walk<M>(self, map: &M) -> Self
    where
        M: Map<A>,
    {
        map.map_table_head(TableHead {
            decoration: self.decoration,
            attrs: self.attrs,
            rows: Vector::from_iter(self.rows.into_iter().map(|row| row.walk(map))),
        })
    }

    pub fn walk_decorations<M, B>(self, map: &M) -> TableHead<B>
    where
        M: DecorationMap<A, B>,
        B: DecorationScheme,
    {
        TableHead {
            decoration: self.decoration.map(|d| map.map_table_head_decoration(d)),
            attrs: self.attrs,
            rows: Vector::from_iter(self.rows.into_iter().map(|row| row.walk_decorations(map))),
        }
    }

    pub fn query<Q>(&self, query: &mut Q)
    where
        Q: Query<A>,
    {
        for row in &self.rows {
            row.query(query);
        }
        query.query_table_head(self);
    }
}

impl<A> TableBody<A>
where
    A: DecorationScheme,
{
    pub fn walk<M>(self, map: &M) -> Self
    where
        M: Map<A>,
    {
        map.map_table_body(TableBody {
            decoration: self.decoration,
            attrs: self.attrs,
            row_head_cols: self.row_head_cols,
            intermediate_head: Vector::from_iter(
                self.intermediate_head.into_iter().map(|row| row.walk(map)),
            ),
            rows: Vector::from_iter(self.rows.into_iter().map(|row| row.walk(map))),
        })
    }

    pub fn walk_decorations<M, B>(self, map: &M) -> TableBody<B>
    where
        M: DecorationMap<A, B>,
        B: DecorationScheme,
    {
        TableBody {
            decoration: self.decoration.map(|d| map.map_table_body_decoration(d)),
            attrs: self.attrs,
            row_head_cols: self.row_head_cols,
            intermediate_head: Vector::from_iter(
                self.intermediate_head
                    .into_iter()
                    .map(|row| row.walk_decorations(map)),
            ),
            rows: Vector::from_iter(self.rows.into_iter().map(|row| row.walk_decorations(map))),
        }
    }

    pub fn query<Q>(&self, query: &mut Q)
    where
        Q: Query<A>,
    {
        for row in &self.intermediate_head {
            row.query(query);
        }

        for row in &self.rows {
            row.query(query);
        }

        query.query_table_body(self);
    }
}

impl<A> TableFoot<A>
where
    A: DecorationScheme,
{
    pub fn walk<M>(self, map: &M) -> Self
    where
        M: Map<A>,
    {
        map.map_table_foot(TableFoot {
            decoration: self.decoration,
            attrs: self.attrs,
            rows: Vector::from_iter(self.rows.into_iter().map(|row| row.walk(map))),
        })
    }

    pub fn walk_decorations<M, B>(self, map: &M) -> TableFoot<B>
    where
        M: DecorationMap<A, B>,
        B: DecorationScheme,
    {
        TableFoot {
            decoration: self.decoration.map(|d| map.map_table_foot_decoration(d)),
            attrs: self.attrs,
            rows: Vector::from_iter(self.rows.into_iter().map(|row| row.walk_decorations(map))),
        }
    }

    pub fn query<Q>(&self, query: &mut Q)
    where
        Q: Query<A>,
    {
        for row in &self.rows {
            row.query(query);
        }
        query.query_table_foot(self);
    }
}

impl<A> Row<A>
where
    A: DecorationScheme,
{
    pub fn walk<M>(self, map: &M) -> Self
    where
        M: Map<A>,
    {
        map.map_row(Row {
            decoration: self.decoration,
            attrs: self.attrs,
            cells: Vector::from_iter(self.cells.into_iter().map(|cell| cell.walk(map))),
        })
    }

    pub fn walk_decorations<M, B>(self, map: &M) -> Row<B>
    where
        M: DecorationMap<A, B>,
        B: DecorationScheme,
    {
        Row {
            decoration: self.decoration.map(|d| map.map_row_decoration(d)),
            attrs: self.attrs,
            cells: Vector::from_iter(
                self.cells
                    .into_iter()
                    .map(|cell| cell.walk_decorations(map)),
            ),
        }
    }

    pub fn query<Q>(&self, query: &mut Q)
    where
        Q: Query<A>,
    {
        for cell in &self.cells {
            cell.query(query);
        }
        query.query_row(self);
    }
}

impl<A> Cell<A>
where
    A: DecorationScheme,
{
    pub fn walk<M>(self, map: &M) -> Self
    where
        M: Map<A>,
    {
        map.map_cell(Cell {
            decoration: self.decoration,
            attrs: self.attrs,
            alignment: self.alignment,
            row_span: self.row_span,
            col_span: self.col_span,
            blocks: self.blocks.walk(map),
        })
    }

    pub fn walk_decorations<M, B>(self, map: &M) -> Cell<B>
    where
        M: DecorationMap<A, B>,
        B: DecorationScheme,
    {
        Cell {
            decoration: self.decoration.map(|d| map.map_cell_decoration(d)),
            attrs: self.attrs,
            alignment: self.alignment,
            row_span: self.row_span,
            col_span: self.col_span,
            blocks: self.blocks.walk_decorations(map),
        }
    }

    pub fn query<Q>(&self, query: &mut Q)
    where
        Q: Query<A>,
    {
        self.blocks.query(query);
        query.query_cell(self);
    }
}

impl<A> Caption<A>
where
    A: DecorationScheme,
{
    pub fn walk<M>(self, map: &M) -> Self
    where
        M: Map<A>,
    {
        map.map_caption(Caption {
            decoration: self.decoration,
            short: self.short.map(|is| is.walk(map)),
            full: self.full.walk(map),
        })
    }

    pub fn walk_decorations<M, B>(self, map: &M) -> Caption<B>
    where
        M: DecorationMap<A, B>,
        B: DecorationScheme,
    {
        Caption {
            decoration: self.decoration.map(|d| map.map_caption_decoration(d)),
            short: self.short.map(|is| is.walk_decorations(map)),
            full: self.full.walk_decorations(map),
        }
    }

    pub fn query<Q>(&self, query: &mut Q)
    where
        Q: Query<A>,
    {
        if let Some(is) = &self.short {
            is.query(query);
        }

        self.full.query(query);
        query.query_caption(self);
    }
}

impl<F, A, B> DecorationMap<SimpleScheme<A>, SimpleScheme<B>> for F
where
    A: Clone,
    B: Clone,
    F: Fn(A) -> B,
{
    fn map_pandoc_decoration(&self, input: <SimpleScheme<A> as DecorationScheme>::Pandoc) -> <SimpleScheme<B> as DecorationScheme>::Pandoc {
        self(input)
    }

    fn map_meta_value_decoration(&self, input: <SimpleScheme<A> as DecorationScheme>::MetaValue) -> <SimpleScheme<B> as DecorationScheme>::MetaValue {
        self(input)
    }

    fn map_blocks_decoration(&self, input: <SimpleScheme<A> as DecorationScheme>::Blocks) -> <SimpleScheme<B> as DecorationScheme>::Blocks {
        self(input)
    }

    fn map_block_decoration(&self, input: <SimpleScheme<A> as DecorationScheme>::Block) -> <SimpleScheme<B> as DecorationScheme>::Block {
        self(input)
    }

    fn map_inlines_decoration(&self, input: <SimpleScheme<A> as DecorationScheme>::Inlines) -> <SimpleScheme<B> as DecorationScheme>::Inlines {
        self(input)
    }

    fn map_inline_decoration(&self, input: <SimpleScheme<A> as DecorationScheme>::Inline) -> <SimpleScheme<B> as DecorationScheme>::Inline {
        self(input)
    }

    fn map_citation_decoration(&self, input: <SimpleScheme<A> as DecorationScheme>::Citation) -> <SimpleScheme<B> as DecorationScheme>::Citation {
        self(input)
    }

    fn map_table_decoration(&self, input: <SimpleScheme<A> as DecorationScheme>::Table) -> <SimpleScheme<B> as DecorationScheme>::Table {
        self(input)
    }

    fn map_table_head_decoration(&self, input: <SimpleScheme<A> as DecorationScheme>::TableHead) -> <SimpleScheme<B> as DecorationScheme>::TableHead {
        self(input)
    }

    fn map_table_body_decoration(&self, input: <SimpleScheme<A> as DecorationScheme>::TableBody) -> <SimpleScheme<B> as DecorationScheme>::TableBody {
        self(input)
    }

    fn map_table_foot_decoration(&self, input: <SimpleScheme<A> as DecorationScheme>::TableFoot) -> <SimpleScheme<B> as DecorationScheme>::TableFoot {
        self(input)
    }

    fn map_row_decoration(&self, input: <SimpleScheme<A> as DecorationScheme>::Row) -> <SimpleScheme<B> as DecorationScheme>::Row {
        self(input)
    }

    fn map_cell_decoration(&self, input: <SimpleScheme<A> as DecorationScheme>::Cell) -> <SimpleScheme<B> as DecorationScheme>::Cell {
        self(input)
    }

    fn map_caption_decoration(&self, input: <SimpleScheme<A> as DecorationScheme>::Caption) -> <SimpleScheme<B> as DecorationScheme>::Caption {
        self(input)
    }
}

