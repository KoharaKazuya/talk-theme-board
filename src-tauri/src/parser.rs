use pulldown_cmark::{html, Parser};
use std::{fs::File, io::prelude::*, io::Cursor};

#[tauri::command]
pub async fn load_text(content: String) -> Result<Vec<Vec<String>>, String> {
    let themes = parse(&content).map_err(|e| format!("failed to parse: {}", e))?;
    Ok(themes)
}

#[tauri::command]
pub async fn load_file(path: String) -> Result<Vec<Vec<String>>, String> {
    let md = read_file(path).map_err(|e| format!("failed to read file: {}", e))?;
    let themes = parse(&md).map_err(|e| format!("failed to parse: {}", e))?;
    Ok(themes)
}

fn read_file(path: String) -> anyhow::Result<String> {
    let mut f = File::open(&path)?;
    let mut md = String::new();
    f.read_to_string(&mut md)?;
    Ok(md)
}

fn parse(md: &str) -> anyhow::Result<Vec<Vec<String>>> {
    let parser = Parser::new(md);

    let mut line = None;
    let mut lines = Vec::new();
    let mut parents = Vec::new();
    for event in parser {
        use pulldown_cmark::{Event::*, Tag};
        match event {
            Start(Tag::List(_)) => {
                if let Some(l) = line.take() {
                    lines.push(l);
                }
            }
            End(Tag::List(_)) => {}
            Start(Tag::Item) => {
                line = Some((parents.last().copied(), Vec::new()));
                parents.push(lines.len());
            }
            End(Tag::Item) => {
                parents.pop();
                if let Some(l) = line.take() {
                    lines.push(l);
                }
            }
            e => {
                if let Some(ref mut l) = line {
                    l.1.push(e);
                }
            }
        }
    }
    if let Some(l) = line.take() {
        lines.push(l);
    }

    let mut result: Vec<Vec<String>> = Vec::new();
    for (parent, events) in lines {
        let new_line = {
            let mut bytes = Vec::new();
            html::write_html(Cursor::new(&mut bytes), events.into_iter())?;
            String::from_utf8_lossy(&bytes).to_string()
        };

        let mut line = if let Some(i) = parent {
            result.get(i).unwrap().clone()
        } else {
            Vec::new()
        };

        line.push(new_line);

        result.push(line);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn 階層構造をパースできる() {
        let md = indoc! {r#"
            - 話題1
              - 話題1の詳細1
              - 話題1の詳細2
                - ネスト3
            - 話題2
              - 話題2の詳細1
        "#};

        let result = parse(md).unwrap();

        assert_eq!(
            result,
            vec![
                vec!["話題1"],
                vec!["話題1", "話題1の詳細1"],
                vec!["話題1", "話題1の詳細2"],
                vec!["話題1", "話題1の詳細2", "ネスト3"],
                vec!["話題2"],
                vec!["話題2", "話題2の詳細1"],
            ]
        );
    }
}
