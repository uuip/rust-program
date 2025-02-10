use comfy_table::{Table, TableComponent};

fn main() {
    let mut table = Table::new();
    table
        .set_style(TableComponent::HeaderLines, '-')
        .set_style(TableComponent::MiddleHeaderIntersections, '-')
        .set_header(vec!["Header1", "Header2", "Header3"])
        .add_row(vec![
            "This is a text",
            "This is another text",
            "This is the third text",
        ])
        .add_row(vec![
            "This is another text",
            "Now\nadd some\nmulti line stuff",
            "This is awesome",
        ]);

    println!("{table}");
}
