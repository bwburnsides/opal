use crate::model::*;

use crate::diagnostic::graphviz_model::*;

pub struct GraphvizRenderer {
    lines: Vec<String>,
    counter: u32,
    digraph: Digraph,
}

impl GraphvizRenderer {
    pub fn render(geode: &Geode) -> String {
        GraphvizRenderer {
            lines: Vec::new(),
            counter: 0,
            digraph: Digraph::new("OpalGeode".to_owned(), Vec::new(), Vec::new()),
        }
        .render_geode(geode.clone())
    }

    fn render_geode(mut self, geode: Geode) -> String {
        self.digraph.nodes.push(Node::new(
            "geode".to_owned(),
            vec![
                NodeAttribute::Label(geode_label(&geode.name)),
                NodeAttribute::Shape("Mrecord".to_owned()),
                NodeAttribute::FillColor("lightpink".to_owned()),
            ],
        ));

        for item in geode.items {
            match item.item {
                ItemKind::Mod(mod_item) => self.render_mod(mod_item),
                ItemKind::Use(use_item) => self.render_use(use_item),
                ItemKind::Function(function_item) => self.render_function(function_item),
                ItemKind::TypeAlias(type_alias_item) => self.render_type_alias(type_alias_item),
                ItemKind::Struct(struct_item) => self.render_struct(struct_item),
                ItemKind::Enum(enum_item) => self.render_enum(enum_item),
                ItemKind::Const(const_item) => self.render_const(const_item),
                ItemKind::Static(static_item) => self.render_static(static_item),
            }
        }

        self.digraph.generate()
    }

    fn render_mod(&mut self, item: ModItem) {
        todo!()
    }

    fn render_use(&mut self, item: UseItem) {
        todo!()
    }

    fn render_function(&mut self, item: FunctionItem) {
        todo!()
    }

    fn render_type_alias(&mut self, item: TypeAliasItem) {
        self.digraph.nodes.push(Node::new(
            format!("type_{}", self.counter),
            vec![
                NodeAttribute::Label(type_alias_label(&item.name.item)),
                NodeAttribute::Shape("Mrecord".to_owned()),
                NodeAttribute::FillColor("black".to_owned()),
            ],
        ))
    }

    fn render_struct(&mut self, item: StructItem) {
        self.digraph.nodes.push(Node::new(
            format!("struct_{}", self.counter),
            vec![
                NodeAttribute::Label(struct_label(&item.name.item)),
                NodeAttribute::Shape("Mrecord".to_owned()),
                NodeAttribute::FillColor("deepskyblue".to_owned()),
            ],
        ));

        self.counter += 1;
    }

    fn render_enum(&mut self, item: EnumItem) {
        self.digraph.nodes.push(Node::new(
            format!("enum_{}", self.counter),
            vec![
                NodeAttribute::Label(enum_label(&item.name.item)),
                NodeAttribute::Shape("Mrecord".to_owned()),
                NodeAttribute::FillColor("aquamarine".to_owned()),
            ],
        ));

        self.digraph
            .edges
            .push(("geode".to_owned(), format!("enum_{}", self.counter)));

        self.counter += 1;
    }

    fn render_const(&mut self, item: ConstItem) {
        todo!()
    }

    fn render_static(&mut self, item: StaticItem) {
        todo!()
    }
}

fn geode_label(name: &String) -> String {
    format!("Geode: {name}")
}

fn type_alias_label(name: &String) -> String {
    format!("Type: {name}")
}

fn struct_label(name: &String) -> String {
    format!("Struct: {name}")
}

fn enum_label(name: &String) -> String {
    format!("Enum: {name}\nlabel =<<table border=\"0\" cellborder=\"0\" cellpadding=\"3\" bgcolor=\"white\"><tr><td bgcolor=\"black\" align=\"center\" colspan=\"2\"><font color=\"white\">State #9</font></td></tr><tr><td align=\"left\" port=\"r2\">&#40;2&#41; e -&gt; r &bull;</td><td bgcolor=\"grey\" align=\"right\">$</td></tr></table>> ];")
}
