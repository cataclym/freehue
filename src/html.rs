pub fn create_html_body(title: &str, color: String) -> String {
    format!(r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <title>{title}</title>
            <style>
                body {{
                    background-color: {color};
                }}
            </style>
        </head>
        <body>

        </body>
        </html>
    "#, title = title, color = color)
}

