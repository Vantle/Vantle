use assemble::Assemble;
use url::Url;

fn assembler() -> bool {
    let peer = peer::Assembler::new().assemble();
    peer.connections().is_empty()
}

fn capacity(value: usize) -> bool {
    let peer = peer::Assembler::new().capacity(value).assemble();
    peer.connections().is_empty()
}

fn address(input: String) -> String {
    let url = Url::parse(&input).unwrap();
    let peer = peer::Assembler::new().address(url).assemble();
    peer.address().to_string()
}

fn default() -> bool {
    let peer = peer::Assembler::default().assemble();
    peer.connections().is_empty()
}
