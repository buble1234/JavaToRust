use crate::utils::java::transpiler::JavaTranspiler;

impl JavaTranspiler {
    pub fn fmt_rust(&self, src: &str) -> String {
        let mut out = String::new();
        let mut indent: i32 = 0;
        let ws = "    ";
        for raw in src.lines() {
            let line = raw.trim_end();
            if line.trim().is_empty() {
                out.push('\n');
                continue;
            }
            let mut parts: Vec<String> = Vec::new();
            let mut buf = String::new();
            let mut in_str = false;
            let mut esc = false;
            for ch in line.chars() {
                if in_str {
                    buf.push(ch);
                    if esc { esc = false; }
                    else if ch == '\\' { esc = true; }
                    else if ch == '"' { in_str = false; }
                } else {
                    match ch {
                        '"' => { in_str = true; buf.push(ch); }
                        '{' | '}' => {
                            if !buf.trim().is_empty() {
                                parts.push(buf.trim().to_string());
                                buf.clear();
                            } else { buf.clear(); }
                            parts.push(ch.to_string());
                        }
                        _ => buf.push(ch),
                    }
                }
            }
            if !buf.trim().is_empty() { parts.push(buf.trim().to_string()); }
            if parts.is_empty() { out.push('\n'); continue; }

            let mut i = 0usize;
            while i < parts.len() {
                let token = &parts[i];

                if token == "}" {
                    if indent > 0 { indent -= 1; }
                    let mut line_txt = "}".to_string();
                    if i + 1 < parts.len() && parts[i + 1] == "," {
                        line_txt.push(',');
                        i += 1;
                    }
                    for _ in 0..indent { out.push_str(ws); }
                    out.push_str(&line_txt);
                    out.push('\n');
                } else if token == "{" {
                    for _ in 0..indent { out.push_str(ws); }
                    out.push('{');
                    out.push('\n');
                    indent += 1;
                } else {
                    let mut printed_open = false;

                    if i + 1 < parts.len() && parts[i + 1] == "{" && token.contains("=>") {
                        for _ in 0..indent { out.push_str(ws); }
                        out.push_str(token.trim_end());
                        out.push(' ');
                        out.push('{');
                        out.push('\n');
                        indent += 1;
                        i += 1;
                        printed_open = true;
                    }

                    if !printed_open {
                        let prev_is_close = i > 0 && parts[i - 1] == "}";
                        if prev_is_close && token.starts_with("else") {
                            for _ in 0..indent { out.push_str(ws); }
                            out.push_str("else");
                            let rest = token[4..].trim_start();
                            if !rest.is_empty() {
                                out.push(' ');
                                out.push_str(rest);
                            }
                            out.push('\n');
                        } else {
                            // уебищный форматтинг через токен запятой
                            if token == "," {
                                // я хз че кроме скипа тут делать
                            } else {
                                for _ in 0..indent { out.push_str(ws); }
                                out.push_str(token);
                                out.push('\n');
                            }
                        }
                    }
                }
                i += 1;
            }
        }
        let mut res = String::new();
        let mut prev_blank = false;
        for l in out.lines() {
            let blank = l.trim().is_empty();
            if blank && prev_blank { continue; }
            res.push_str(l);
            res.push('\n');
            prev_blank = blank;
        }
        res
    }
}
