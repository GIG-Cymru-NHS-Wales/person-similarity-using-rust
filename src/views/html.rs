////
// HTML rendering helpers.
////

/// Render strings into an HTML table tag.
pub fn html_table_tag(table: Vec<Vec<String>>) -> String {
    format!("<table>\n{}</table>\n", html_table_tr_tags(table))
}

/// Render strings into HTML table tr tags.
pub fn html_table_tr_tags(rows: Vec<Vec<String>>) -> String {
    rows.iter()
        .map(|row| 
            format!("<tr>{}</tr>\n", html_table_td_tags(row))
        )
        .collect::<String>()
}

/// Render strings into HTML table td tags.
pub fn html_table_td_tags(cells: &Vec<String>) -> String {
    cells.iter().map(|cell| 
        format!("<td>{}</td>", cell)
    ).collect::<String>()
}
