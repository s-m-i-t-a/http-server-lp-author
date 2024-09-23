#[derive(Debug)]
#[derive(PartialEq)]
enum Lang {
    English,
    Spanish,
    Chinese,
    Texan,
    Czech,
}

struct Greeting {
    message: String,
    lang: Lang,
}

fn main() {
    let mut v: Vec<Greeting> = Vec::new();

    let g: Greeting = Greeting {
        lang: Lang::English,
        message: String::from("Hello WasmEdge!"),
    };
    v.push(g);
    let g: Greeting = Greeting {
        lang: Lang::Spanish,
        message: String::from("Hola WasmEdge!"),
    };
    v.push(g);
    let g: Greeting = Greeting {
        lang: Lang::Texan,
        message: String::from("Howdy WasmEdge!"),
    };
    v.push(g);
    let g: Greeting = Greeting {
        lang: Lang::Chinese,
        message: String::from("WasmEdge 你好!"),
    };
    v.push(g);

    v.push(Greeting {
        lang: Lang::Czech,
        message: String::from("Ahoj WasmEdge!"),
    });

    for e in v {
        if e.lang == Lang::Czech {
            println!("{:?} {}", e.lang, e.message);
        }
    }
}
